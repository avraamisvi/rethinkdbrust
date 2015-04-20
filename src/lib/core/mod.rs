use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use std::str;
use client::RethinkDB;


macro_rules! json_array {
    ( $( $e:expr ),* )  => {{
        let mut a = Vec::new();
        $(
            a.push($e);
        )*
        Json::Array(a)
    }}
}

macro_rules! json_string {
    ($s:expr) => { Json::String($s.clone()) }
}

macro_rules! json_opts {
    ( $( $k:expr => $v:expr),* ) => {{
        let mut d = BTreeMap::new();
        $(
            d.insert($k.to_string(), $v);
        )*
        Json::Object(d)


    }}
}

macro_rules! json_i64 {
    ($s:expr) => { Json::I64($s) }
}

macro_rules! json_bool {
    ($s:expr) => { Json::Boolean($s) }
}

pub mod ql2;

/// All provides default `run` function for all RQLQueries.
pub trait RQLQuery<'a> {

    /// Takes a mutable reference of `RethinkDB` that handles the connection pool.
    fn run(&'a self, rethinkdb : &mut RethinkDB) -> json::Json {

        rethinkdb.send(Json::Array(vec![Json::I64(1), self.to_query_types()]))

    }

    /// All implementations knows how to convert to the right Json protocol required by
    /// RethinkDB
    fn to_query_types(&'a self) -> json::Json;

}

pub mod select;
pub mod table;
pub mod write;
