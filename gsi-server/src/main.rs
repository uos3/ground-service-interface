//! # Ground Service Interface Server
//!
//! The GSI server provides a web API to access the service interface of the OBC software.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(target = "armv7-unknown-linux-gnueabihf")]
use rppal;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}