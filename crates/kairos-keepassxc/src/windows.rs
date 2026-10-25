//! Direct Windows pipe transport, NOT the browser's length-prefixed stdio link.
//!
//! Modelled after: KeePassXC BrowserShared.cpp, BrowserHost.cpp and
//! NativeMessagingProxy.cpp at 860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda.
//! Tokio supplies overlapped I/O, cancellation and SECURITY_IDENTIFICATION.

use crate::{
    protocol::{frame_length, Envelope},
    Error, Transport, MAX_MESSAGE_BYTES,
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe::{ClientOptions, NamedPipeClient},
    runtime::{Builder, Handle, Runtime},
};
use zeroize::Zeroizing;

/// A bounded synchronous facade over Tokio's overlapped named-pipe client.
/// Create and use outside an async runtime, on a blocking application worker.
/// KeePassXC browser integration must already be enabled by the user.
/// Neither KeePassXC nor keepassxc-proxy is launched or configured here.
pub struct NamedPipeTransport {
    runtime: Runtime,
    pipe: Option<NamedPipeClient>,
    pending: Zeroizing<Vec<u8>>,
    timeout: Duration,
}

impl NamedPipeTransport {
    /// A timeout from 1 ms to 2 minutes covers the entire exchange, including
    /// user pairing/database dialogs and any broadcasts, with no write retry.
    pub fn connect(timeout: Duration) -> Result<Self, Error> {
        check_context()?;
        if timeout < Duration::from_millis(1) || timeout > Duration::from_secs(120) {
            return Err(Error::InvalidInput);
        }
        let path = pipe_name(&qt_username()?)?;
        Self::connect_path(&path, timeout)
    }

    fn connect_path(path: &str, timeout: Duration) -> Result<Self, Error> {
        check_context()?;
        let runtime = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .map_err(|_| Error::Transport)?;
        let pipe = runtime
            .block_on(async { ClientOptions::new().open(path) })
            .map_err(|_| Error::Transport)?;
        Ok(Self {
            runtime,
            pipe: Some(pipe),
            pending: Zeroizing::new(Vec::with_capacity(MAX_MESSAGE_BYTES)),
            timeout,
        })
    }
}

impl Transport for NamedPipeTransport {
    fn exchange(&mut self, request: &[u8]) -> Result<Zeroizing<Vec<u8>>, Error> {
        check_context()?;
        if request.len() > MAX_MESSAGE_BYTES {
            return Err(Error::MessageTooLarge);
        }
        let pipe = self.pipe.as_mut().ok_or(Error::SessionClosed)?;
        let pending = &mut self.pending;
        let result = self.runtime.block_on(async {
            tokio::time::timeout(self.timeout, async {
                // Raw compact UTF-8 JSON, no u32 prefix or newline delimiter.
                // The server's readAll parser expects one outstanding request.
                pipe.write_all(request)
                    .await
                    .map_err(|_| Error::Transport)?;
                let mut broadcasts = 0;
                loop {
                    if let Some(length) = frame_length(pending)? {
                        let reply = Zeroizing::new(pending[..length].to_vec());
                        pending.drain(..length);
                        let envelope = Envelope::parse(&reply)?;
                        if matches!(
                            envelope.action.as_deref(),
                            Some("database-locked" | "database-unlocked")
                        ) {
                            broadcasts += 1;
                            if broadcasts > 16 {
                                return Err(Error::UnexpectedAction);
                            }
                            continue;
                        }
                        return Ok(reply);
                    }
                    let mut chunk = Zeroizing::new([0u8; 8192]);
                    let count = pipe
                        .read(chunk.as_mut_slice())
                        .await
                        .map_err(|_| Error::Transport)?;
                    if count == 0 {
                        return Err(Error::Transport);
                    }
                    if pending.len() + count > MAX_MESSAGE_BYTES {
                        return Err(Error::MessageTooLarge);
                    }
                    pending.extend_from_slice(&chunk[..count]);
                }
            })
            .await
            .map_err(|_| Error::Timeout)?
        });
        if result.is_err() {
            // Dropping the pipe cancels overlapped operations. Late replies and
            // partially sent requests cannot cross into another exchange.
            self.pipe.take();
            self.pending.clear();
        }
        result
    }
}

fn check_context() -> Result<(), Error> {
    if Handle::try_current().is_ok() {
        Err(Error::InvalidContext)
    } else {
        Ok(())
    }
}

fn pipe_name(username: &str) -> Result<String, Error> {
    if username.is_empty()
        || username
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\'))
    {
        return Err(Error::Transport);
    }
    // QLocalSocket turns BrowserShared's leading slash into the pipe namespace.
    Ok(format!(
        r"\\.\pipe\org.keepassxc.KeePassXC.BrowserServer_{username}"
    ))
}

fn qt_username() -> Result<String, Error> {
    // KeePassXC appends qgetenv("USERNAME") (C-runtime bytes) to a QString,
    // implicitly decoding those bytes as UTF-8. Windows' wide environment API
    // can therefore produce a different pipe name for non-ASCII account names.
    // getenv_s copies into our own bounded buffer under the CRT's lock.
    extern "C" {
        fn getenv_s(
            count: *mut usize,
            buffer: *mut std::ffi::c_char,
            size: usize,
            name: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
    let mut bytes = vec![0u8; 32768];
    let mut count = 0;
    // SAFETY: live writable buffer, valid size/count pointers and a static NUL
    // terminated name. getenv_s never returns a borrowed environment pointer.
    let status = unsafe {
        getenv_s(
            &mut count,
            bytes.as_mut_ptr().cast(),
            bytes.len(),
            c"USERNAME".as_ptr(),
        )
    };
    if status != 0 || count <= 1 || count > bytes.len() {
        return Err(Error::Transport);
    }
    Ok(String::from_utf8_lossy(&bytes[..count - 1]).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qt_pipe_name_preserves_username() {
        assert_eq!(
            pipe_name("Axel").unwrap(),
            r"\\.\pipe\org.keepassxc.KeePassXC.BrowserServer_Axel"
        );
        assert_eq!(
            pipe_name("Änne").unwrap(),
            r"\\.\pipe\org.keepassxc.KeePassXC.BrowserServer_Änne"
        );
        assert_eq!(pipe_name("a\\b"), Err(Error::Transport));
        assert!(pipe_name(&qt_username().unwrap())
            .unwrap()
            .starts_with(r"\\.\pipe\org.keepassxc."));
    }

    fn test_server(mode: u8) -> (String, std::thread::JoinHandle<()>) {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc,
        };
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let path = format!(
            r"\\.\pipe\kairos-pr6-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let server_path = path.clone();
        let (ready, wait) = mpsc::channel();
        let thread = std::thread::spawn(move || {
            let runtime = Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();
            runtime.block_on(async {
                let mut pipe = tokio::net::windows::named_pipe::ServerOptions::new()
                    .first_pipe_instance(true)
                    .create(&server_path)
                    .unwrap();
                ready.send(()).unwrap();
                pipe.connect().await.unwrap();
                let mut request = [0; 1024];
                assert!(pipe.read(&mut request).await.unwrap() > 0);
                if mode == 1 {
                    tokio::time::sleep(Duration::from_millis(150)).await;
                    return;
                }
                // A broadcast and a fragmented response share a byte stream.
                pipe.write_all(br#"{"action":"database-unlocked"}{"action":"dummy","nonce":"a"#)
                    .await
                    .unwrap();
                tokio::time::sleep(Duration::from_millis(15)).await;
                pipe.write_all(br#"b"}"#).await.unwrap();
                // Keep the server alive until the client closes its handle.
                let _ = pipe.read(&mut request).await;
            });
        });
        wait.recv_timeout(Duration::from_secs(5)).unwrap();
        (path, thread)
    }

    #[test]
    fn real_private_pipe_assembles_fragments_and_skips_broadcast() {
        let (path, server) = test_server(0);
        let mut transport =
            NamedPipeTransport::connect_path(&path, Duration::from_secs(2)).unwrap();
        let response = transport.exchange(br#"{"action":"dummy"}"#).unwrap();
        assert_eq!(response.as_slice(), br#"{"action":"dummy","nonce":"ab"}"#);
        drop(transport);
        server.join().unwrap();
    }

    #[test]
    fn real_private_pipe_timeout_closes_transport() {
        let (path, server) = test_server(1);
        let mut transport =
            NamedPipeTransport::connect_path(&path, Duration::from_millis(30)).unwrap();
        assert!(matches!(
            transport.exchange(br#"{"action":"dummy"}"#),
            Err(Error::Timeout)
        ));
        assert!(matches!(
            transport.exchange(br#"{"action":"dummy"}"#),
            Err(Error::SessionClosed)
        ));
        server.join().unwrap();
    }
}
