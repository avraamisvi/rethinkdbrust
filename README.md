# RethinkDB 2.0 Rust Driver

This is a very early stage WIP driver for json protocol.

# Example

````rust
  use rethinkdb::RethinkDB;
  use rethinkdb::api::*;

	  let mut rethinkdb = RethinkDB::connect("localhost", 7888, "AUTH", 3);
    let tc = db("test").table_create("person").replicas(1i32).run(&mut rethinkdb);

    struct Person {
    	name : String,
    	age  : u32,
    }
    let nacho = Person{name : "nacho".to_string(), age : 6};

    db("test").table("person").insert(json::encode(nacho)).run(&mut rethinkdb);


````

# Messages implemented
   - DB
   - TABLE_CREATE
   - TABLE
   - TABLE_DROP
   - GET

# Contributing
By now it is just a PR with the project bein able to build.


# Licence
MIT
