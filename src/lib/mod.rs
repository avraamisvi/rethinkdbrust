 #![feature(core)]
 #[warn(unused_imports)]
extern crate byteorder;
extern crate rustc_serialize;

mod core;
mod client;

mod test;
pub use client::*;

pub mod r {

/*!
The main interface. User should start interactions opening a RethinkDB connection with
the `RethinkDB` instance.

All interactions return a `rethinkdb::RQLResponse` that represents either a `Json` response
or a `RQLResponse::Cursor`.


# Examples

```no_run
use rethinkdb::RethinkDB;
use rethinkdb::r::*; // Must be imported by your code.

let mut rethinkdb = RethinkDB::connect("localhost", 7888, "AUTH", 3);
db("test").table_create("person_create").replicas(1i32).run(&mut rethinkdb);
```
*/	
	
	pub use core::select::*;
	pub use core::table::*;
	pub use core::write::*;
}




