#![feature(convert)]
#[cfg(test)]
#[warn(unused_imports)]

use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::thread::sleep_ms;
use std::collections::BTreeMap;
use RethinkDB;
use r::*;


#[test]
fn test_insert() {
    let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    let db = db("test");

    let mut nachoData = BTreeMap::new();
    nachoData.insert("id".to_string(), Json::I64(1i64));
    nachoData.insert("name".to_string(), Json::String("Tomate".to_string()));
    nachoData.insert("age".to_string(), Json::I64(6i64));

    let res = db.table("person_get").insert(Json::Object(nachoData)).run(&mut rethinkdb);

    println!("{:?}", res);
}

#[test]
fn test_replace() {
    let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    let db = db("test");

    let mut nachoData = BTreeMap::new();
    nachoData.insert("id".to_string(), Json::I64(1i64));
    nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
    nachoData.insert("age".to_string(), Json::I64(6i64));
    nachoData.insert("weight".to_string(), Json::I64(100i64));

    let res = db.table("person_get").replace(Json::Object(nachoData)).run(&mut rethinkdb);

    println!("{:?}", res);
}
