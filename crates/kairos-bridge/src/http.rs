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
struct WorkerHealth(Arc<AtomicBool>);
impl Drop for WorkerHealth {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
fn retry_accept(error: &std::io::Error) -> bool {
    matches!(
        error.kind(),
        std::io::ErrorKind::WouldBlock
            | std::io::ErrorKind::Interrupted
            | std::io::ErrorKind::ConnectionAborted
            | std::io::ErrorKind::ConnectionReset
    )
}
pub struct Bridge {
    state: Shared,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    alive: Arc<AtomicBool>,
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
        let alive = Arc::new(AtomicBool::new(true));
        let health = WorkerHealth(alive.clone());
        let worker = thread::Builder::new().name("kairos-browser-bridge".into()).spawn(move || {
            let _health = health;
            while !worker_stop.load(Ordering::Relaxed) {
                if let Ok(mut state) = worker_state.lock() {
                    if state.active.as_ref().is_some_and(|a| a.received.elapsed() > FRESH_FOR) {
                        state.active = None;
                    }
                    if state.pending.as_ref().is_some_and(|p| Instant::now() >= p.until) {
                        state.pending = None;
                    }
                    if let Some(active) = &mut state.active {
                        if active.username_until.is_some_and(|until| Instant::now() >= until) {
                            active.context.username = None;
                            active.username_until = None;
                        }
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
                    Err(e) if retry_accept(&e) => thread::sleep(Duration::from_millis(20)),
                    Err(_) => break,
                }
            }
        }).map_err(|_| Error::Io)?;
        Ok(Self {
            state,
            stop,
            worker: Some(worker),
            alive,
        })
    }
    pub fn status(&self) -> Result<PairingStatus, Error> {
        if !self.alive.load(Ordering::Acquire) {
            return Err(Error::Io);
        }
        Ok(self.state.lock().map_err(|_| Error::Io)?.status())
    }
    pub fn begin_pairing(&self) -> Result<Zeroizing<String>, Error> {
        if !self.alive.load(Ordering::Acquire) {
            return Err(Error::Io);
        }
        self.state.lock().map_err(|_| Error::Io)?.begin_pairing()
    }
    pub fn cancel_pairing(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.pending = None;
        }
    }
    pub fn latest_for(&self, window: Option<BrowserWindow>) -> Option<Context> {
        if !self.alive.load(Ordering::Acquire) {
            return None;
        }
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
    // Winsock accept inherits FIONBIO. Explicitly restore blocking I/O before
    // applying timeouts, including when headers/body arrive in separate packets.
    stream.set_nonblocking(false).map_err(|_| Error::Io)?;
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
    let mut headers = [httparse::EMPTY_HEADER; 48];
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
    let mut origin = None;
    for header in request.headers.iter() {
        let value = std::str::from_utf8(header.value).map_err(|_| Error::Invalid)?;
        match header.name.to_ascii_lowercase().as_str() {
            "host" if host.is_none() => host = Some(value),
            "x-kairos-token" if token.is_none() => token = Some(value),
            "content-length" if length.is_none() => {
                length = Some(value.parse::<usize>().map_err(|_| Error::Invalid)?)
            }
            "content-type" if content_type.is_none() => content_type = Some(value),
            "origin" if origin.is_none() && extension_origin(value) => origin = Some(value),
            "host" | "x-kairos-token" | "content-length" | "content-type" | "origin"
            | "referer" | "transfer-encoding" => return Err(Error::Invalid),
            // Fetch metadata differs between extension engines. It is not an
            // authentication signal; Origin + custom token + no preflight are.
            _ => {}
        }
    }
    if host != Some("127.0.0.1:19471") || !content_type.is_some_and(json_content_type) {
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

fn extension_origin(origin: &str) -> bool {
    if let Some(id) = origin.strip_prefix("chrome-extension://") {
        return id.len() == 32 && id.bytes().all(|b| (b'a'..=b'p').contains(&b));
    }
    if let Some(id) = origin.strip_prefix("moz-extension://") {
        return id.len() == 36
            && id.bytes().enumerate().all(|(index, b)| {
                if matches!(index, 8 | 13 | 18 | 23) {
                    b == b'-'
                } else {
                    b.is_ascii_hexdigit()
                }
            });
    }
    false
}
fn json_content_type(value: &str) -> bool {
    let mut parts = value.split(';');
    if !parts
        .next()
        .is_some_and(|p| p.trim().eq_ignore_ascii_case("application/json"))
    {
        return false;
    }
    if let Some(charset) = parts.next() {
        if !charset.trim().eq_ignore_ascii_case("charset=utf-8")
            && !charset.trim().eq_ignore_ascii_case("charset=\"utf-8\"")
        {
            return false;
        }
    }
    parts.next().is_none()
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
    fn extension_origins_and_json_charset_work_without_allowing_websites() {
        for origin in [
            "chrome-extension://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "moz-extension://12345678-1234-1234-1234-123456789abc",
        ] {
            let raw = request(
                &format!("Origin: {origin}\r\nSec-Fetch-Site: cross-site\r\n"),
                "{}",
            )
            .replace(
                "application/json\r\n",
                "application/json; charset=UTF-8\r\n",
            );
            assert!(parse(raw.as_bytes()).unwrap().is_some());
        }
        for origin in [
            "null",
            "https://evil.invalid",
            "chrome-extension://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.evil.invalid",
            "moz-extension://12345678-1234-1234-1234-123456789abc/evil",
            "chrome-extension://AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        ] {
            assert!(parse(request(&format!("Origin: {origin}\r\n"), "{}").as_bytes()).is_err());
        }
        assert!(parse(request("Origin: chrome-extension://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\nOrigin: https://evil.invalid\r\n", "{}").as_bytes()).is_err());
    }
    #[test]
    fn accept_resets_are_recoverable_and_dead_worker_is_visible() {
        for kind in [
            std::io::ErrorKind::ConnectionReset,
            std::io::ErrorKind::ConnectionAborted,
            std::io::ErrorKind::Interrupted,
        ] {
            assert!(retry_accept(&std::io::Error::from(kind)));
        }
        assert!(!retry_accept(&std::io::Error::from(
            std::io::ErrorKind::PermissionDenied
        )));
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        let bridge = Bridge::with_listener(Box::new(Paired), || None, listener).unwrap();
        bridge.alive.store(false, Ordering::Release);
        assert_eq!(bridge.status(), Err(Error::Io));
        assert!(bridge.latest_for(None).is_none());
        assert_eq!(bridge.begin_pairing().err(), Some(Error::Io));
    }
    #[test]
    fn reject_web_origins_dns_rebinding_and_ambiguous_framing() {
        for bad in [
            request("Origin: https://evil.invalid\r\n", "{}"),
            request("Origin: null\r\n", "{}"),
            request("Referer: https://evil.invalid/\r\n", "{}"),
            request("Host: evil.invalid\r\n", "{}"),
            request("Transfer-Encoding: chunked\r\n", "{}"),
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
    #[test]
    fn accepted_socket_waits_for_delayed_headers_and_fragmented_body() {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let _bridge = Bridge::with_listener(Box::new(Paired), || None, listener).unwrap();
        let body = format!(
            r#"{{"timestamp":{},"nonce":"{}","url":"https://example.invalid/"}}"#,
            unix_seconds(),
            "c".repeat(32)
        );
        let raw = request("", &body);
        let offset = raw.find("\r\n\r\n").unwrap() + 4;
        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        thread::sleep(Duration::from_millis(60));
        stream.write_all(&raw.as_bytes()[..offset]).unwrap();
        thread::sleep(Duration::from_millis(60));
        stream.write_all(&raw.as_bytes()[offset..]).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        assert!(response.starts_with("HTTP/1.1 204 No Content"));
    }
}
