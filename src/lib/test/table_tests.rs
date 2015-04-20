#![feature(convert)]
#[cfg(test)]
#[warn(unused_imports)]

use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::thread::sleep_ms;
use std::collections::BTreeMap;
use RethinkDB;
use api::*;
use core::*;


// // socat  -v -x TCP4-LISTEN:7888,fork,reuseaddr TCP4:localhost:28015
#[test]
fn test_create() {
    let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    let db = db("test");
    let tc = db.table_create("person_get").replicas(1i32).run(&mut rethinkdb);
    assert_eq!(1, 2);
}
