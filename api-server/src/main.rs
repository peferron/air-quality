extern crate hyper;

mod args;
mod error;

use args::Args;
use error::{Error, Result};
use hyper::{Get, Post};
use hyper::server::{Server, Request, Response};
use std::io::Read;
use std::process;

fn main() {
    if let Err(e) = run() {
        match e {
            Error::Args(usage) => println!("{}", usage),
            _ => println!("Fatal error: {:?}", e),
        }
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::from_env()?;
    println!("Starting with {:?}", args);

    let server = Server::http(&args.listen_addr[..])?;
    server.handle(handle)?;

    Ok(())
}

fn handle(req: Request, res: Response) {
    let res_body = match req.method {
        Get => handle_get(req),
        Post => handle_post(req),
        _ => String::from("{\"status\":\"error\"}"),
    };

    match res.send(res_body.as_bytes()) {
        Ok(()) => println!("\nSent response: {}", res_body),
        Err(e) => println!("\nError sending response: {:?}", e),
    }
}

fn handle_get(mut req: Request) -> String {
    unreachable!();
}

fn handle_post(mut req: Request) -> String {
    let mut req_body = String::new();
    req.read_to_string(&mut req_body).unwrap();
    println!("\nReceived POST request: {}", req_body);
    String::from("{\"status\":\"ok\"}")
}
