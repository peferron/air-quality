extern crate hyper;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate redis;

mod args;
mod error;
mod response;

use args::Args;
use error::{Error, Result};
use hyper::client::Client;
use redis::Commands;
use response::Response;
use std::io::Read;
use std::process;

type ProcessFn = Fn(&[String]) -> Result<()>;

const REDIS_KEY: &'static str = "measurements";
const REDIS_TMP_KEY: &'static str = "measurements-tmp";

fn main() {
    if let Err(e) = run() {
        match e {
            Error::Args(usage) => println!("{}", usage),
            _ => println!("Fatal error: {:#?}", e),
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::from_env()?;
    println!("Starting with {:#?}", args);

    let conn = redis::Client::open(args.redis_url.as_ref())?.get_connection()?;
    let client = Client::new();
    let api_url = args.api_url.clone();

    let process = move |jsons: &[String]| upload(jsons, &client, &api_url);

    flush(&conn, REDIS_KEY, &process)?;
    flush(&conn, REDIS_TMP_KEY, &process)?;
    watch(&conn, &process)
}

fn flush(conn: &redis::Connection, key: &str, process: &ProcessFn) -> Result<()> {
    loop {
        let jsons: Vec<String> = conn.lrange(key, -500, -1)?;

        if jsons.is_empty() {
            return Ok(());
        }

        process(&jsons)?;
        conn.ltrim(key, 0, -(jsons.len() as isize + 1))?;
    }
}

fn watch(conn: &redis::Connection, process: &ProcessFn) -> Result<()> {
    loop {
        let json: String = conn.brpoplpush(REDIS_KEY, REDIS_TMP_KEY, 90)?;
        process(&[json])?;
        conn.lpop(REDIS_TMP_KEY)?;
    }
}

fn upload(jsons: &[String], client: &Client, url: &str) -> Result<()> {
    println!("Uploading {} measurements, last: {}",
        jsons.len(), jsons.last().unwrap_or(&String::from("None")));

    let json = format!("[{}]", jsons.join(","));
    let mut response = client.post(url).body(&json).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;

    println!("Received response: {}", response_body);

    let parsed_response: Response = serde_json::from_str(&response_body)?;

    if parsed_response.status == "ok" {
        Ok(())
    } else {
        Err(Error::Response(parsed_response.err))
    }
}
