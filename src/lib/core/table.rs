use core::ql2::*;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use std::str;
use client::RethinkDB;
use core::select::Db;
use core::RQLQuery;


/// Represents `table_create` command. Must be constructed from a `Db`
pub struct TableCreate<'a> {
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String,
    primary_key : String,
    replicas : i32,
    shards   : i32,
    primary_replica_tag : String
}

pub struct TableDrop<'a> {
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String
}

impl<'a> TableDrop<'a> {
    pub fn new(db : &'a Db, name : &str) -> TableDrop<'a> {
        TableDrop {
            term : Term_TermType::TABLE_DROP,
            stm  : "table_drop".to_string(),
            db   : db,
            name : name.to_string()
        }
    }
}


impl<'a> TableCreate<'a> {
    pub fn new(db : &'a Db, name : &str) -> TableCreate<'a> {
        TableCreate {
            term : Term_TermType::TABLE_CREATE,
            stm  : "table_create".to_string(),
            db   : db,
            name : name.to_string(),
            primary_key : "id".to_string(),
            replicas    : 1i32,
            shards      : 1i32,
            primary_replica_tag : "".to_string()
        }
    }

    pub fn primary_key(&'a mut self, primary_key : String) -> &mut TableCreate<'a> {
        self.primary_key = primary_key.clone();
        self
    }

    pub fn replicas(&'a mut self, total : i32) -> &mut TableCreate<'a> {
        self.replicas = total;
        self
    }

    pub fn shards(&'a mut self, total : i32) -> &mut TableCreate<'a> {
        self.shards = total;
        self
    }
}

impl<'a> RQLQuery<'a> for TableDrop<'a> {
    fn to_query_types(&'a self) -> json::Json {
        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ]
        ]
    }
}

impl<'a> RQLQuery<'a> for TableCreate<'a> {
    fn to_query_types(&'a self) -> json::Json {

        json_array![
            Json::I64(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ],
            json_opts![
                   "primary_key" => json_string!(self.primary_key.clone()),
                   "shards"      => json_i64!(self.shards as i64),
                   "replicas"    => json_i64!(self.replicas as i64)]
                   // TODO LAST PARAM PENDING : TAG
        ]

    }
}