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

pub mod ql2;


pub struct TableInsert<'a> {
    term    : ql2::Term_TermType,
    stm     : String,
    table   : &'a select::Table<'a>,
    object  : BTreeMap<String, json::Json>,
    conflict: String,
    durability: String,
    return_changes: bool
}


/// All provides default `run` function for all RQLQueries.
pub trait RQLQuery<'a> {

    /// Takes a mutable reference of `RethinkDB` that handles the connection pool.
    fn run(&'a self, rethinkdb : &mut RethinkDB) -> bool {

        rethinkdb.send(Json::Array(vec![Json::I64(1), self.to_query_types()]));
        true
    }
    
    /// All implementations knows how to convert to the right Json protocol required by
    /// RethinkDB
    fn to_query_types(&'a self) -> json::Json;

}


impl<'a> RQLQuery<'a> for TableInsert<'a> {
    fn to_query_types(&'a self) -> json::Json {

        let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(Json::Object(self.object.clone()));

        let mut d = BTreeMap::new();
        d.insert("conflict".to_string(), Json::String("update".to_string()));
        d.insert("durability".to_string(), Json::String("hard".to_string()));//?
        d.insert("return_changes".to_string(), Json::Boolean(true));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)

    }
}


impl<'a> TableInsert<'a> {
    fn new(table: &'a select::Table, object: BTreeMap<String, json::Json>) -> TableInsert<'a> {
        TableInsert {
            term    : ql2::Term_TermType::INSERT,
            stm     : "insert".to_string(),
            table   : table,
            object  : object,
            conflict: "error".to_string(),//default "error" accordingly rethinkdb documentation
            durability: "hard".to_string(),
            return_changes: true
        }
    }

    pub fn conflict(&mut self, value: &str) -> &TableInsert<'a> {//TODO: use methods that handle specific options.
        self.conflict = value.to_string();
        self
    }

    pub fn durability(&mut self, value: &str) -> &TableInsert<'a> {
        self.conflict = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &TableInsert<'a> {
        self.return_changes = value;
        self
    }
}

pub mod select;
pub mod table;
