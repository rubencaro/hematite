extern crate actix_web;
extern crate rocksdb;
use actix_web::{server, App, HttpRequest};
use rocksdb::{Error, Options, DB};

fn main() {
    let _db = setup_db().unwrap();

    let addr = "127.0.0.1:8088";
    let srv = server::new(|| app_factory()).bind(addr).unwrap();

    println!("Listening on {:?}", addr);
    srv.run();
}

fn setup_db() -> Result<DB, Error> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, "data.rocksdb")
}

fn app_factory() -> App {
    App::new().resource("/", |r| r.f(index))
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

// inner workings of MergeOperator in https://github.com/facebook/rocksdb/wiki/Merge-Operator
// multithreaded rust-rocksdb in https://github.com/spacejam/rust-rocksdb/blob/master/tests/test_multithreaded.rs
// rust side mege operator in https://docs.rs/rocksdb/0.10.1/rocksdb/merge_operator/index.html
// use from actix like this https://actix.rs/docs/databases/ to allow multi(green)threaded operation