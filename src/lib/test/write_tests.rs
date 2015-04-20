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


#[test]
fn test_insert() {
    let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    let db = db("test");

    //let tc = db.table_create("person_get").primary_key("name".to_string()).run(&mut rethinkdb);

    let mut nachoData = BTreeMap::new();
    nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
    nachoData.insert("age".to_string(), Json::I64(6i64));

    //db.table("person_get").insert(nachoData).run(&mut rethinkdb);

    let nacho_json = db.table("person_get").get(Json::String("Nacho".to_string())).run(&mut rethinkdb);
    println!("{:?}", nacho_json);
    match nacho_json.find_path(&["r", "name"]).unwrap() {
        &Json::String(ref name) => assert_eq!(*name, "Nacho".to_string()),
        _ => panic!("The returned object is strange")
    }

    let tc = db.table_create("person_get").primary_key("name".to_string()).run(&mut rethinkdb);
    sleep_ms(5000);

    db.table("person_get").insert(Json::Object(nachoData)).run(&mut rethinkdb);
    db.table("person_get").get(Json::String("Nacho".to_string())).run(&mut rethinkdb);


}
