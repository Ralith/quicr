extern crate tokio;
extern crate quicr;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate futures;
#[macro_use]
extern crate structopt;

use std::fs;
use std::fmt;
use std::path::{self, Path, PathBuf};
use std::str;
use std::rc::Rc;
use std::ascii;
use std::net::SocketAddr;

use futures::{Future, Stream};
use tokio::executor::current_thread;
use tokio::runtime::current_thread::Runtime;
use failure::{ResultExt, Fail};
use structopt::StructOpt;

use slog::{Logger, Drain};
use failure::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct PrettyErr<'a>(&'a Fail);
impl<'a> fmt::Display for PrettyErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)?;
        let mut x: &Fail = self.0;
        while let Some(cause) = x.cause() {
            f.write_str(": ")?;
            fmt::Display::fmt(&cause, f)?;
            x = cause;
        }
        Ok(())
    }
}

pub trait ErrorExt {
    fn pretty(&self) -> PrettyErr;
}

impl ErrorExt for Error {
    fn pretty(&self) -> PrettyErr { PrettyErr(self.cause()) }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Opt {
    /// file to log TLS keys to for debugging
    #[structopt(parse(from_os_str), long = "keylog")]
    keylog: Option<PathBuf>,
    /// directory to serve files from
    #[structopt(parse(from_os_str))]
    root: PathBuf,
    /// TLS private key in PEM format
    #[structopt(parse(from_os_str), short = "k", long = "key", requires = "cert")]
    key: Option<PathBuf>,
    /// TLS certificate in PEM format
    #[structopt(parse(from_os_str), short = "c", long = "cert", requires = "key")]
    cert: Option<PathBuf>,
    /// Enable stateless retries
    #[structopt(long = "stateless-retry")]
    stateless_retry: bool,
    /// Address to listen on
    #[structopt(long = "listen", default_value = "[::]:4433")]
    listen: SocketAddr,
}

fn main() {
    let opt = Opt::from_args();
    let code = {
        let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
        let drain = slog_term::FullFormat::new(decorator).use_original_order().build().fuse();
        if let Err(e) = run(Logger::root(drain, o!()), opt) {
            eprintln!("ERROR: {}", e.pretty());
            1
        } else { 0 }
    };
    ::std::process::exit(code);
}

fn run(log: Logger, options: Opt) -> Result<()> {
    let root = Rc::new(options.root);
    if !root.exists() { bail!("root path does not exist"); }

    let mut runtime = Runtime::new()?;

    let mut builder = quicr::Endpoint::new();
    builder.logger(log.clone())
        .config(quicr::Config {
            protocols: vec![b"hq-11"[..].into()],
            max_remote_bi_streams: 64,
            keylog: options.keylog,
            use_stateless_retry: options.stateless_retry,
            ..quicr::Config::default()
        })
        .listen();

    if let Some(key_path) = options.key {
        let key = fs::read(&key_path).context("failed to read private key")?;;
        builder.private_key_pem(&key).context("failed to load private key")?;

        let cert_path = options.cert.unwrap(); // Ensured present by option parsing
        let cert = fs::read(&cert_path).context("failed to read certificate")?;
        builder.certificate_pem(&cert).context("failed to load certificate")?;
    } else {
        builder.generate_insecure_certificate().context("failed to generate certificate")?;
    }

    let (_, driver, incoming) = builder.bind(options.listen)?;

    runtime.spawn(incoming.for_each(move |conn| { handle_connection(&root, &log, conn); Ok(()) }));
    runtime.block_on(driver)?;

    Ok(())
}

fn handle_connection(root: &PathBuf, log: &Logger, conn: quicr::NewConnection) {
    let quicr::NewConnection { incoming, connection } = conn;
    let log = log.new(o!("local_id" => format!("{}", connection.local_id())));
    info!(log, "got connection";
          "remote_id" => %connection.remote_id(),
          "address" => %connection.remote_address(),
          "protocol" => connection.protocol().map_or_else(|| "<none>".into(), |x| String::from_utf8_lossy(&x).into_owned()));
    let log2 = log.clone();
    let root = root.clone();

    // Each stream initiated by the client constitutes a new request.
    current_thread::spawn(
        incoming
            .map_err(move |e| info!(log2, "connection terminated"; "reason" => %e))
            .for_each(move |stream| { handle_request(&root, &log, stream); Ok(()) })
    );
}

fn handle_request(root: &PathBuf, log: &Logger, stream: quicr::NewStream) {
    let stream = match stream {
        quicr::NewStream::Bi(stream) => stream,
        quicr::NewStream::Uni(_) => unreachable!(), // config.max_remote_uni_streams is defaulted to 0
    };
    let root = root.clone();
    let log = log.clone();
    let log2 = log.clone();
    let log3 = log.clone();

    current_thread::spawn(
        quicr::read_to_end(stream, 64 * 1024) // Read the request, which must be at most 64KiB
            .map_err(|e| format_err!("failed reading request: {}", e))
            .and_then(move |(stream, req)| {
                let mut escaped = String::new();
                for &x in &req[..] {
                    let part = ascii::escape_default(x).collect::<Vec<_>>();
                    escaped.push_str(str::from_utf8(&part).unwrap());
                }
                info!(log, "got request"; "content" => escaped);
                // Execute the request
                let resp = process_get(&root, &req).unwrap_or_else(move |e| {
                    error!(log, "failed to process request"; "reason" => %e.pretty());
                    format!("failed to process request: {}\n", e.pretty()).into_bytes().into()
                });
                // Write the response
                tokio::io::write_all(stream, resp).map_err(|e| format_err!("failed to send response: {}", e))
            })
            // Gracefully terminate the stream
            .and_then(|(stream, _)| tokio::io::shutdown(stream).map_err(|e| format_err!("failed to shutdown stream: {}", e)))
            .map(move |_| info!(log3, "request complete"))
            .map_err(move |e| error!(log2, "request failed"; "reason" => %e.pretty()))
    )
}

fn process_get(root: &Path, x: &[u8]) -> Result<Box<[u8]>> {
    if x.len() < 4 || &x[0..4] != b"GET " { bail!("missing GET"); }
    if x[4..].len() < 2 || &x[x.len()-2..] != b"\r\n" { bail!("missing \\r\\n"); }
    let path = str::from_utf8(&x[4..x.len()-2]).context("path is malformed UTF-8")?;
    let path = Path::new(&path);
    let mut real_path = PathBuf::from(root);
    let mut components = path.components();
    match components.next() {
        Some(path::Component::RootDir) => {}
        _ => { bail!("path must be absolute"); }
    }
    for c in components {
        match c {
            path::Component::Normal(x) => { real_path.push(x); }
            x => { bail!("illegal component in path: {:?}", x); }
        }
    }
    let data = fs::read(&real_path).context("failed reading file")?;
    Ok(data.into())
}
