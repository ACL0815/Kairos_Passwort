//! Strict HTTP framing, loopback bind, no CORS and no secret-bearing responses.
//! Modelled after: Syncthing GUI Host validation (see docs/sources/browser.md).
use crate::*;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::atomic::{AtomicBool, Ordering},
    thread::{self, JoinHandle},
};

const MAX_BYTES: usize = 12 * 1024;
pub struct Bridge {
    state: Shared,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}
impl Bridge {
    pub fn start(
        store: Box<dyn SecretStore>,
        foreground: impl Fn() -> Option<BrowserWindow> + Send + 'static,
    ) -> Result<Self, Error> {
        let listener =
            TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, PORT)).map_err(|_| Error::Io)?;
        Self::with_listener(store, foreground, listener)
    }
    fn with_listener(
        store: Box<dyn SecretStore>,
        foreground: impl Fn() -> Option<BrowserWindow> + Send + 'static,
        listener: TcpListener,
    ) -> Result<Self, Error> {
        let state = Arc::new(Mutex::new(State::new(store)?));
        listener.set_nonblocking(true).map_err(|_| Error::Io)?;
        let stop = Arc::new(AtomicBool::new(false));
        let worker_state = state.clone();
        let worker_stop = stop.clone();
        let worker = thread::Builder::new().name("kairos-browser-bridge".into()).spawn(move || {
            while !worker_stop.load(Ordering::Relaxed) {
                if let Ok(mut state) = worker_state.lock() {
                    if state.active.as_ref().is_some_and(|a| a.received.elapsed() > FRESH_FOR) {
                        state.active = None;
                    }
                    if state.pending.as_ref().is_some_and(|p| Instant::now() >= p.until) {
                        state.pending = None;
                    }
                    state.nonces.retain(|_, received| received.elapsed() <= Duration::from_secs(61));
                }
                match listener.accept() {
                    Ok((mut stream, peer)) if peer.ip().is_loopback() => {
                        let result = read_request(&mut stream).and_then(|request| {
                            let mut state = worker_state.lock().map_err(|_| Error::Io)?;
                            state.handle(&request.path, &request.token, &request.body, foreground())
                        });
                        // Uniform failure, no reflective content, no auth/pairing oracle in body.
                        let status = if result.is_ok() { "204 No Content" } else { "403 Forbidden" };
                        let _ = stream.write_all(format!("HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n").as_bytes());
                    }
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => thread::sleep(Duration::from_millis(20)),
                    Err(_) => break,
                }
            }
        }).map_err(|_| Error::Io)?;
        Ok(Self {
            state,
            stop,
            worker: Some(worker),
        })
    }
    pub fn status(&self) -> Result<PairingStatus, Error> {
        Ok(self.state.lock().map_err(|_| Error::Io)?.status())
    }
    pub fn begin_pairing(&self) -> Result<Zeroizing<String>, Error> {
        self.state.lock().map_err(|_| Error::Io)?.begin_pairing()
    }
    pub fn cancel_pairing(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.pending = None;
        }
    }
    pub fn latest_for(&self, window: Option<BrowserWindow>) -> Option<Context> {
        self.state.lock().ok()?.context(window)
    }
}
impl Drop for Bridge {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

struct Request {
    path: String,
    token: Zeroizing<String>,
    body: Zeroizing<Vec<u8>>,
}
fn read_request(stream: &mut TcpStream) -> Result<Request, Error> {
    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|_| Error::Io)?;
    stream
        .set_write_timeout(Some(Duration::from_millis(200)))
        .map_err(|_| Error::Io)?;
    let started = Instant::now();
    let mut bytes = Zeroizing::new(vec![0u8; MAX_BYTES]);
    let mut used = 0;
    loop {
        if started.elapsed() > Duration::from_secs(1) || used == MAX_BYTES {
            return Err(Error::Invalid);
        }
        let count = stream.read(&mut bytes[used..]).map_err(|_| Error::Io)?;
        if count == 0 {
            return Err(Error::Invalid);
        }
        used += count;
        if let Some(request) = parse(&bytes[..used])? {
            return Ok(request);
        }
    }
}
fn parse(bytes: &[u8]) -> Result<Option<Request>, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::Invalid);
    }
    let mut headers = [httparse::EMPTY_HEADER; 24];
    let mut request = httparse::Request::new(&mut headers);
    let offset = match request.parse(bytes).map_err(|_| Error::Invalid)? {
        httparse::Status::Partial => return Ok(None),
        httparse::Status::Complete(offset) => offset,
    };
    if request.method != Some("POST") || request.version != Some(1) {
        return Err(Error::Invalid);
    }
    let path = request.path.ok_or(Error::Invalid)?;
    let mut host = None;
    let mut token = None;
    let mut length = None;
    let mut content_type = None;
    for header in request.headers.iter() {
        let value = std::str::from_utf8(header.value).map_err(|_| Error::Invalid)?;
        match header.name.to_ascii_lowercase().as_str() {
            "host" if host.is_none() => host = Some(value),
            "x-kairos-token" if token.is_none() => token = Some(value),
            "content-length" if length.is_none() => {
                length = Some(value.parse::<usize>().map_err(|_| Error::Invalid)?)
            }
            "content-type" if content_type.is_none() => content_type = Some(value),
            "host" | "x-kairos-token" | "content-length" | "content-type" | "origin"
            | "referer" | "transfer-encoding" => return Err(Error::Invalid),
            "sec-fetch-site" if value != "none" => return Err(Error::Invalid),
            _ => {}
        }
    }
    if host != Some("127.0.0.1:19471") || content_type != Some("application/json") {
        return Err(Error::Invalid);
    }
    let length = length.filter(|n| *n <= 8192).ok_or(Error::Invalid)?;
    let token = token
        .filter(|t| {
            t.len() == 43
                && t.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        })
        .ok_or(Error::Unauthorized)?;
    match bytes.len().cmp(&(offset + length)) {
        std::cmp::Ordering::Less => Ok(None),
        std::cmp::Ordering::Greater => Err(Error::Invalid),
        std::cmp::Ordering::Equal => Ok(Some(Request {
            path: path.into(),
            token: Zeroizing::new(token.into()),
            body: Zeroizing::new(bytes[offset..].to_vec()),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Paired;
    impl SecretStore for Paired {
        fn load(&mut self) -> Result<Option<Zeroizing<String>>, Error> {
            Ok(Some(Zeroizing::new("a".repeat(43))))
        }
        fn save(&mut self, _: &str) -> Result<(), Error> {
            Err(Error::Storage)
        }
    }
    fn request(headers: &str, body: &str) -> String {
        format!("POST /v1/context HTTP/1.1\r\nHost: 127.0.0.1:19471\r\nContent-Type: application/json\r\nX-Kairos-Token: {}\r\nContent-Length: {}\r\n{headers}\r\n{body}", "a".repeat(43), body.len())
    }
    #[test]
    fn valid_framing_and_partial_body() {
        let raw = request("", "{}");
        assert!(parse(raw.as_bytes()).unwrap().is_some());
        assert!(parse(&raw.as_bytes()[..raw.len() - 1]).unwrap().is_none());
    }
    #[test]
    fn reject_web_origins_dns_rebinding_and_ambiguous_framing() {
        for bad in [
            request("Origin: https://evil.invalid\r\n", "{}"),
            request("Origin: null\r\n", "{}"),
            request("Referer: https://evil.invalid/\r\n", "{}"),
            request("Host: evil.invalid\r\n", "{}"),
            request("Transfer-Encoding: chunked\r\n", "{}"),
            request("Sec-Fetch-Site: cross-site\r\n", "{}"),
            request("Content-Length: 2\r\n", "{}"),
            request("", "{}").replace("127.0.0.1:19471", "evil.invalid:19471"),
            request("", "{}").replace("POST", "OPTIONS"),
            request("", "{}") + "POST / HTTP/1.1",
        ] {
            assert!(parse(bad.as_bytes()).is_err());
        }
    }
    #[test]
    fn real_loopback_transport_authenticates_rejects_replay_and_never_echoes_secrets() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let window = BrowserWindow {
            hwnd: 10,
            process: 20,
        };
        let bridge =
            Bridge::with_listener(Box::new(Paired), move || Some(window), listener).unwrap();
        let body = format!(
            r#"{{"timestamp":{},"nonce":"{}","url":"https://example.invalid/signup?private#fragment"}}"#,
            unix_seconds(),
            "f".repeat(32)
        );
        for (raw, expected) in [
            (request("", &body), "204 No Content"),
            (request("", &body), "403 Forbidden"),
            (
                request("", &body).replace(&"a".repeat(43), &"b".repeat(43)),
                "403 Forbidden",
            ),
            (
                request("Origin: https://evil.invalid\r\n", &body),
                "403 Forbidden",
            ),
            (
                request("", &body).replace("127.0.0.1:19471", "evil.invalid:19471"),
                "403 Forbidden",
            ),
        ] {
            let mut stream = TcpStream::connect(address).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            stream.write_all(raw.as_bytes()).unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            assert!(response.starts_with(&format!("HTTP/1.1 {expected}\r\n")));
            assert!(response.ends_with("\r\n\r\n"));
            assert!(!response.to_lowercase().contains("access-control"));
            assert!(!response.contains(&"a".repeat(43)));
            assert!(!response.contains("example.invalid"));
        }
        assert_eq!(
            bridge.latest_for(Some(window)).unwrap().url,
            "https://example.invalid/signup"
        );
    }
}
