extern crate hyper;
extern crate hyper_rustls;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate redis;

mod args;
mod error;
mod response;

use args::Args;
use error::{Error, Result};
use hyper::client::Client;
use hyper::header::{Authorization, Basic};
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use redis::Commands;
use response::Response;
use std::env;
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

    let client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
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
        let json: String = conn.brpoplpush(REDIS_KEY, REDIS_TMP_KEY, 4000)?;
        process(&[json])?;
        conn.lpop(REDIS_TMP_KEY)?;
    }
}

fn upload(jsons: &[String], client: &Client, url: &str) -> Result<()> {
    println!("Uploading {} measurements, last: {}",
        jsons.len(), jsons.last().unwrap_or(&String::from("None")));

    let json = format!("[{}]", jsons.join(","));
    let mut req = client.post(url).body(&json);

    if let (Ok(u), Ok(p)) = (env::var("API_SERVER_USERNAME"), env::var("API_SERVER_PASSWORD")) {
        println!("Authenticating as user \"{}\"",u);
        req = req.header(Authorization(Basic { username: u, password: Some(p) }));
    }

    let mut res = req.send()?;
    let mut res_body = String::new();
    res.read_to_string(&mut res_body)?;

    println!("Received response: {}", res_body);

    let response: Response = serde_json::from_str(&res_body)?;

    if response.status == "ok" {
        Ok(())
    } else {
        Err(Error::Response(response.err))
    }
}
