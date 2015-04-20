use core::ql2::*;
use core::{RQLQuery};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use std::str;
use client::RethinkDB;
use core::select::{Table};

pub struct Insert<'a> {
    term    : Term_TermType,
    stm     : String,
    table   : &'a Table<'a>,
    object  : Json,
    conflict: String,
    durability: String,
    return_changes: bool
}

impl<'a> RQLQuery<'a> for Insert<'a> {
    fn to_query_types(&'a self) -> Json {

        let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(self.object.clone());

        let mut d = BTreeMap::new();
        d.insert("conflict".to_string(), Json::String("update".to_string()));
        d.insert("durability".to_string(), Json::String("hard".to_string()));//?
        d.insert("return_changes".to_string(), Json::Boolean(true));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)

    }
}

impl<'a> Insert<'a> {
    pub fn new(table: &'a Table, object: Json) -> Insert<'a> {
        Insert {
            term    : Term_TermType::INSERT,
            stm     : "insert".to_string(),
            table   : table,
            object  : object,
            conflict: "error".to_string(),//default "error" accordingly rethinkdb documentation
            durability: "hard".to_string(),
            return_changes: true
        }
    }

    pub fn conflict(&mut self, value: &str) -> &Insert<'a> {//TODO: use methods that handle specific options.
        self.conflict = value.to_string();
        self
    }

    pub fn durability(&mut self, value: &str) -> &Insert<'a> {
        self.conflict = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &Insert<'a> {
        self.return_changes = value;
        self
    }
}
