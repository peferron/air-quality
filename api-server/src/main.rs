extern crate chrono;
extern crate hyper;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

mod args;
mod error;
mod influx;
mod measurement;

use args::Args;
use error::{Error, Result};
use hyper::server::{Server, Request, Response};
use measurement::Measurement;
use std::io;
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

    let client = influx::Client::new(&args.influx_write_url, &args.influx_line_prefix);

    if true {
        let server = Server::http(&args.listen_addr[..])?;
        server.handle(move |req: Request, res: Response| handle(req, res, &client))?;
    }

    Ok(())
}

fn handle(req: Request, res: Response, client: &influx::Client) {
    let res_body = match handle_req(req, &client) {
        Ok(()) => json!({"status": "ok"}),
        Err(e) => {
            println!("Error handling request: {:?}", e);
            json!({"status": "error"})
        },
    };

    match res.send(res_body.to_string().as_bytes()) {
        Ok(()) => println!("Sent response: {}", res_body),
        Err(e) => println!("Error sending response: {:?}", e),
    }
}

fn handle_req(mut req: Request, client: &influx::Client) -> Result<()> {
    if req.method != hyper::Post {
        return Err(Error::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "Only POST requests are supported"
        )))
    }

    let mut req_body = String::new();
    req.read_to_string(&mut req_body)?;

    let measurements: Vec<Measurement> = serde_json::from_str(&req_body)?;
    client.write(&measurements)
}
