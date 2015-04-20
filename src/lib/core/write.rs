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

pub struct Update<'a> {
    term    : Term_TermType,
    stm     : String,
    table   : &'a Table<'a>,
    object  : Json,
    non_atomic: bool,
    durability: String,
    return_changes: bool
}

pub struct Replace<'a> {
    term    : Term_TermType,
    stm     : String,
    table   : &'a Table<'a>,
    object  : Json,
    non_atomic: bool,
    durability: String,
    return_changes: bool
}

impl<'a> RQLQuery<'a> for Insert<'a> {
    fn to_query_types(&'a self) -> Json {

        /*let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(self.object.clone());

        let mut d = BTreeMap::new();
        d.insert("conflict".to_string(), Json::String(self.conflict));
        d.insert("durability".to_string(), Json::String(self.durability));//?
        d.insert("return_changes".to_string(), Json::Boolean(self.return_changes));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)*/

        //============================
        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.table.to_query_types(),
                self.object.clone()
            ],
            json_opts![
                "conflict" => json_string!(self.conflict),
                "durability" => json_string!(self.durability),
                "return_changes" => json_bool!(self.return_changes)
            ]
        ]

    }
}

impl<'a> RQLQuery<'a> for Update<'a> {
    fn to_query_types(&'a self) -> Json {

        /*let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(self.object.clone());

        let mut d = BTreeMap::new();
        d.insert("non_atomic".to_string(), Json::Boolean(self.non_atomic));
        d.insert("durability".to_string(), Json::String(self.durability));//?
        d.insert("return_changes".to_string(), Json::Boolean(self.return_changes));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)*/

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.table.to_query_types(),
                self.object.clone()
            ],
            json_opts![
                "non_atomic" => json_bool!(self.non_atomic),
                "durability" => json_string!(self.durability),
                "return_changes" => json_bool!(self.return_changes)
            ]
        ]

    }
}

impl<'a> RQLQuery<'a> for Replace<'a> {
    fn to_query_types(&'a self) -> Json {

        /*let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(self.object.clone());

        let mut d = BTreeMap::new();
        d.insert("non_atomic".to_string(), Json::Boolean(self.non_atomic));
        d.insert("durability".to_string(), Json::String(self.durability));//?
        d.insert("return_changes".to_string(), Json::Boolean(self.return_changes));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)*/

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.table.to_query_types(),
                self.object.clone()
            ],
            json_opts![
                "non_atomic" => json_bool!(self.non_atomic),
                "durability" => json_string!(self.durability),
                "return_changes" => json_bool!(self.return_changes)
            ]
        ]

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
        self.durability = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &Insert<'a> {
        self.return_changes = value;
        self
    }
}

impl<'a> Update<'a> {
    pub fn new(table: &'a Table, object: Json) -> Update<'a> {
        Update {
            term    : Term_TermType::UPDATE,
            stm     : "update".to_string(),
            table   : table,
            object  : object,
            non_atomic: false,
            durability: "hard".to_string(),
            return_changes: true
        }
    }

    pub fn non_atomic(&mut self, value: bool) -> &Update<'a> {
        self.non_atomic = value;
        self
    }

    pub fn durability(&mut self, value: &str) -> &Update<'a> {
        self.durability = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &Update<'a> {
        self.return_changes = value;
        self
    }
}

impl<'a> Replace<'a> {
    pub fn new(table: &'a Table, object: Json) -> Replace<'a> {
        Replace {
            term    : Term_TermType::UPDATE,
            stm     : "replace".to_string(),
            table   : table,
            object  : object,
            non_atomic: false,
            durability: "hard".to_string(),
            return_changes: true
        }
    }

    pub fn non_atomic(&mut self, value: bool) -> &Replace<'a> {
        self.non_atomic = value;
        self
    }

    pub fn durability(&mut self, value: &str) -> &Replace<'a> {
        self.durability = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &Replace<'a> {
        self.return_changes = value;
        self
    }
}
