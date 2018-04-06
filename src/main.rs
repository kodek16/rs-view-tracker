extern crate chrono;
extern crate chrono_tz;
extern crate futures;
extern crate hyper;
extern crate toml;

#[macro_use]
extern crate serde_derive;

mod config;

use std::fs;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::path::Path;
use std::thread;

use std::io::Write;

use chrono::Utc;
use chrono_tz::Tz;

use futures::future::Future;

use hyper::header::Referer;
use hyper::server::{Http, Request, Response, Service};
use hyper::Uri;

struct Tracker {
    logs_dir: String,
    timezone: Tz,
}

impl Service for Tracker {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let logs_dir = self.logs_dir.to_owned();
        let timezone = self.timezone.to_owned();
        thread::spawn(move || {
            log_visit(req, logs_dir, timezone);
        });

        // An empty response will do.
        Box::new(futures::future::ok(Response::new()))
    }
}

fn log_visit(req: Request, logs_dir_path: String, timezone: Tz) {
    let page = match req.headers().get::<Referer>() {
        Some(referer) => match referer.parse::<Uri>() {
            Ok(uri) => uri.path().to_string(),

            // Ignore malformed (and malicious) Referer headers.
            Err(_) => return,
        },
        None => String::from("unknown"),
    };

    let create_err = fs::create_dir_all(&logs_dir_path).err();

    match fs::metadata(&logs_dir_path) {
        Ok(ref metadata) if !metadata.is_dir() => {
            panic!("'{}' is not a directory.", &logs_dir_path)
        },
        Err(_) => {
            panic!("Couldn't create logs directory: {:?}", create_err.unwrap());
        },
        _ => {},
    };

    let time = Utc::now().with_timezone(&timezone);

    let file_path = Path::new(&logs_dir_path)
        .join(format!("{}.csv", time.format("%Y%m%d")));

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .expect(&format!("Couldn't open log file '{}'.", file_path.display()));

    writeln!(&mut file, "{},{}", page, time.to_rfc3339())
        .expect("Couldn't write to log file.");
}

fn main() {
    let config = config::load();

    // Fail fast if timezone is invalid.
    let timezone: Tz = config.timezone.parse()
        .expect("Invalid timezone");

    // Listen on all interfaces by default.
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);

    let server = Http::new().bind(&addr, move || {
        Ok(Tracker {
            logs_dir: config.logs_dir.to_string(),
            timezone: timezone
        })
    }).unwrap();
    server.run().unwrap();
}
