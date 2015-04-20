use std::thread;
use std::sync::{Arc, Mutex};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::str;
use core::ql2::*;
use byteorder::Error as BError;
use rustc_serialize::json::ParserError;

#[derive(Debug)]
pub enum RethinkDBError {
    InternalIoError(Error),
    ServerError,
    WriteError(BError),
    JsonParseError(ParserError)
}

impl From<ParserError> for RethinkDBError {
    fn from (err : ParserError) -> RethinkDBError {
        RethinkDBError::JsonParseError(err)
    }
}

impl From<BError> for RethinkDBError {
    fn from (err : BError) ->  RethinkDBError {
        RethinkDBError::WriteError(err)
    }

}

impl From<Error> for RethinkDBError {
    fn from(err: Error) -> RethinkDBError {
        RethinkDBError::InternalIoError(err)
    }
}

struct Cursor {
    name : String
}


pub enum RQLResponse {
    Value(Json),
    Cursor(Cursor)
}

pub type RethinkDBResult<T> = Result<T, RethinkDBError>;


/// Represents a database connection. It is the actual struct that holds `TcpStream`
/// to server;
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String,
    token    : i64
}

impl Connection {

    /// Connects to the provided server `host` and `port`. `auth` is used for authentication.
    pub fn connect(host: &str , port: u16, auth : &str) -> Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let mut conn = Connection{
                host    : host.to_string(),
                port    : port,
                stream  : stream,
                auth    : auth.to_string(),
                token   : 1i64, //TODO: Should be reseted after a lot of queries.
        };

        conn.handshake();
        conn
    }

    /// Handshakes the connection. By now only supports `V0_4` and `JSON`.
    fn handshake(&mut self)  {
        self.stream.write_u32::<LittleEndian>(VersionDummy_Version::V0_4 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(VersionDummy_Protocol::JSON as u32);
        self.stream.flush();

        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
        }

    }

    /// Talks to the server sending and reading back the propper JSON messages
    fn send(&mut self, json : Json) -> RethinkDBResult<RQLResponse> {

        self.token += 1;
        self.stream.write_i64::<LittleEndian>(self.token);
        let message = json.to_string();
        let len = message.len();
        self.stream.write_i32::<LittleEndian>(len as i32);
        println!("{}",message);
        write!(self.stream, "{}", message);
        self.stream.flush();
        self.recv()
    }

    fn recv(&mut self) ->  RethinkDBResult<RQLResponse> {  //Read result. Should go into a different method?

        let recv_token = try!{self.stream.read_i64::<LittleEndian>()};
        let recv_len = try!{self.stream.read_i32::<LittleEndian>()};

        let mut buf = BufStream::new(&self.stream);
        
        let mut c = Vec::with_capacity(recv_len as usize);
        unsafe { c.set_len(recv_len as usize) };
        buf.read(&mut c);
        let json_str = str::from_utf8(&c).ok().unwrap();

        
        let mut recv_json = try!{json::Json::from_str(json_str)};

        match recv_json {
            Ok(obj) if obj.is_objec() => {
                match obj.find("t") {
                    Ok(Json::I64(s)) if s == Response_ResponseType::SUCCESS_ATOM as i64 => {
                        match obj.find("r") {
                            Ok(o) => Ok(RQLResponse::Value(o)),
                            _ => panic!("r is not present")
                        }
                    },
                    _ => panic!("Right now we support SUCCESS_ATOM only")

                }
            }
            _ => panic!("Failed to parse json")
        }

    }

}

//The main interface entrance. User could should start interactions with RethinkDB
/// # Examples
///
/// ```no_run
/// use rethinkdb::RethinkDB;
/// use rethinkdb::api::*;
///
/// let mut rethinkdb = RethinkDB::connect("localhost", 7888, "AUTH", 3);
/// db("test").table_create("person_create").replicas(1i32).run(&mut rethinkdb);
/// ```

pub struct RethinkDB {
    pool : Connection
}

impl RethinkDB {
    /// Connects to RethinkDB with `pool_size` connections inside the pool.
    pub fn connect(host: &str , port: u16, auth : &str, pool_size : usize) -> RethinkDB {
        // let mut pool = Pool::with_capacity(pool_size, 0, ||  Connection::connect(host, port, auth));
            
        RethinkDB {
            pool : Connection::connect(host, port, auth)
        }
    }

    /// Used to safely grab a reusable connection from the pool and talk to 
    /// the server.
    #[inline(always)]
    pub fn send(&mut self, message : Json) -> Json {
        // let con_arc = self.pool.clone();
        // let mut pool = con_arc.lock().unwrap();
        // let mut conn = &mut pool.checkout().unwrap();
        self.pool.send(message.clone())
    }

}