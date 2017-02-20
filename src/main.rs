#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate regex;
extern crate serde;
extern crate rocket_contrib;
extern crate serde_json;
extern crate pomo;

use rocket::request::Form;
use rocket::config::{Config, Environment};
use regex::Regex;
use rocket_contrib::JSON;
use std::env;
use pomo::models::{SlashParams, Message};
use pomo::commands::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DEFAULT_MAX_POMO: i32 = 8;

fn pomo_response(user_id: String, text: &str) -> String {
    let reset = Regex::new(r"^r(eset)? *(\d*)$").unwrap();
    let reset_only_remaining = Regex::new(r"^(rr|reset remaining)? *(\d*)$").unwrap();
    let done = Regex::new(r"^d(one)? *$").unwrap();
    let show = Regex::new(r"^s(how)? *$").unwrap();
    let set_tomato = Regex::new(r"^(st|set tomato) (.+)$").unwrap();
    let set_icon = Regex::new(r"^(si|set icon) *(.*)$").unwrap();
    let help = Regex::new(r"^h(elp)? *$").unwrap();
    let usage = format!("```/pomo command version {}\n\nUsage:\n/pomo [show/s]: show detail\n/pomo [done/d]: pomo - 1\n/pomo [reset/r] (count=8): reset count (default 8)\n/pomo [reset remaining/rr] (count=8): reset only remaining count (default 8)\n/pomo [set tomato/st] (emoji): set alternative :tomato: emoji\n/pomo [set icon/si] (emoji): set alternative icon emoji\n```", VERSION);
    if reset.is_match(text) {
        let cap = reset.captures(text).unwrap();
        let reset_count: i32 = if &cap[2] == "" { DEFAULT_MAX_POMO } else { (&cap[2]).to_string().parse().unwrap() };
        let new_score = set_pomo(user_id, reset_count).unwrap();
        new_score.show_remaining()
    } else if reset_only_remaining.is_match(text) {
        let cap = reset_only_remaining.captures(text).unwrap();
        let reset_only_remaining_count: i32 = if &cap[2] == "" { DEFAULT_MAX_POMO } else { (&cap[2]).to_string().parse().unwrap() };
        let new_score = set_remaining(user_id, reset_only_remaining_count).unwrap();
        new_score.show_remaining()
    } else if done.is_match(text) {
        let score = done_pomo(user_id).unwrap();
        if score.remaining == 0 {
            "All DONE!! :tada:".to_string()
        } else {
            score.show_remaining()
        }
    } else if show.is_match(text) {
        let score = show_pomo(user_id).unwrap();
        score.show_detail()
    } else if set_tomato.is_match(text) {
        let cap = set_tomato.captures(text).unwrap();
        set_tomato_emoji(user_id, &cap[2]).unwrap();
        format!("set tomato emoji to {}", &cap[2])
    } else if set_icon.is_match(text) {
        let cap = set_icon.captures(text).unwrap();
        set_icon_emoji(user_id, &cap[2]).unwrap();
        format!("set icon emoji to {}", &cap[2])
    } else if help.is_match(text) {
        usage.to_string()
    } else {
        format!("Unknown Command!\n{}", usage)
    }
}

fn get_slack_token() -> Result<String, String> {
    match env::var("SLACK_TOKEN") {
        Ok(token) => Ok(token),
        Err(_) => panic!("SLACK_TOKEN undefined")
    }
}

#[post("/", data = "<input>")]
fn pomo_command(input: Form<SlashParams>) -> JSON<Message> {
    
    let input_inner = input.into_inner();
    let slack_token = get_slack_token().unwrap_or(String::new());
    if slack_token != input_inner.token {
        return JSON(Message{response_type: "ephemeral".to_string(), text: "Not Allowed to Access".to_string()});
    }
    let text = &*(input_inner.text);
    let message = Message { response_type: "in_channel".to_string(), text: pomo_response(input_inner.user_id, text) };
    JSON(message)
}

fn get_server_port() -> usize {
    let port_str = env::var("PORT").unwrap_or(String::new());
    port_str.parse().unwrap_or(8000)
}

fn main() {
    let config = Config::default_for(Environment::active().unwrap(), "/custom").unwrap()
        .address("0.0.0.0".into())
        .port(get_server_port());
    rocket::custom(&config).mount("/", routes![pomo_command]).launch();
}
