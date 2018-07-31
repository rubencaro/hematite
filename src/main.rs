extern crate actix_web;
extern crate rocksdb;
use actix_web::{server, App, HttpRequest};
use rocksdb::{Error, Options, DB};

fn main() {
    let _db = setup_db().unwrap();

    let addr = "127.0.0.1:8088";
    let srv = server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .unwrap();

    println!("Listening on {:?}", addr);
    srv.run();
}

fn setup_db() -> Result<DB, Error> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, "data.rocksdb")
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}
