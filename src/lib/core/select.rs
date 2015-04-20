use core::ql2::*;
use core::{RQLQuery};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use std::str;
use client::RethinkDB;
use core::table::{TableCreate, TableDrop};
use core::write::{Insert};

/// Represents `db` command. Must be constructed with `rethinkdb::api::db`.
pub struct Db {
    term : Term_TermType,
    stm  : String,
    name : String
}

/// Represents `get` command.
pub struct Get<'a> {
    term : Term_TermType,
    stm  : String,
    table   : &'a Table<'a>,
    pk   : Json
}

/// Represents `db` command.
pub struct Table<'a> {//TODO criar um so struct ( Command? )
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String
}

/// Starting point for any databse interaction. Takes database name as arg
/// and gives you access to methods to interact with RethinkDB.
pub fn db(name : &str) -> Db {
    Db {
        term : Term_TermType::DB,
        stm  : "db".to_string(),
        name : name.to_string()
    }
}


impl<'a> RQLQuery<'a> for Table<'a> {
    fn to_query_types(&'a self) -> json::Json {

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ],
            json_opts![
                "use_outdated" => Json::Boolean(true),
                "identifier_format".to_string() => json_string!("name".to_string())
            ]
        ]

    }
}


impl<'a> RQLQuery<'a> for Db {
    fn to_query_types(&'a self) -> json::Json {

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                json_string!(self.name.clone())
            ]
        ]
    }
}


impl<'a> Table<'a> {

    /// Inserts the given `object` into the databse.
    pub fn insert (&'a self, object: Json) -> Insert { // TODO: fix this type. must be Json::Object
        Insert::new(self, object)
    }

    /// Returns a single row by `primary_key`
    pub fn get(&self, pk : Json) -> Get {
        Get::new(self, pk)
    }
}


/// This is the main implementation of this API. All commands must be created from
/// a `Db` instance;
impl Db {

    ///  Creates table named `name`.
    pub fn table_create (&self, name : &str) -> TableCreate {
        TableCreate::new(self, name)
    }

    /// Given the table `name`, returns all data.
    pub fn table (&self, name : &str) -> Table {
        Table {
            term : Term_TermType::TABLE,
            stm  : "table".to_string(),
            db   : self,
            name : name.to_string()
        }
    }

    /// Drops the table from the database.
    pub fn table_drop(&self, name : &str) -> TableDrop {
        TableDrop::new(self, name)
    }


}


impl<'a> RQLQuery<'a> for Get<'a> {
    fn to_query_types(&'a self) -> json::Json {
        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.table.to_query_types(),
                self.pk.clone()
            ]
        ]
    }
}

impl<'a> Get<'a> {
    pub fn new(table : &'a Table, pk : Json) -> Get<'a> { // TODO: Create multople methods by key type
        Get {
            term : Term_TermType::GET,
            stm  : "get".to_string(),
            table : table,
            pk   : pk
        }
    }
}
