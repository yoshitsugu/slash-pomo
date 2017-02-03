#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate redis;
extern crate serde_json;
extern crate rocket_contrib;

pub mod models;
pub mod commands;
