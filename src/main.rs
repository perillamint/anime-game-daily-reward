// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use clap::Parser;
use reqwest::{header, Client};

mod config;

use config::read_config;

#[macro_use]
extern crate lazy_static;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, short = 'c', value_name = "CONFIG")]
    config: String,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

static API_URL: &str =
    "https://hk4e-api-os.hoyoverse.com/event/sol/sign?act_id=e202102251931481&lang=ko-kr";

#[tokio::main]
async fn main() {
    let cfg = read_config(&ARGS.config);
    let client = Client::builder()
        .user_agent(cfg.http.user_agent)
        .build()
        .unwrap();

    for session in cfg.sessions {
        let mut headers = header::HeaderMap::new();
        let cookie_str = format!("ltoken={}; ltuid={}", session.ltoken, session.ltuid);
        let cookie = header::HeaderValue::from_str(&cookie_str).unwrap();
        headers.insert("Cookie", cookie);

        let msg = client.post(API_URL).headers(headers).send().await.unwrap();
        println!(
            "Query result of {}: {}",
            session.ltuid,
            msg.text().await.unwrap()
        );
    }
}
