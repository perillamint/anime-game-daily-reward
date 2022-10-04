// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use clap::Parser;
use reqwest::{header, Client};

mod config;

use config::{read_config, SVCType};

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

fn get_actid(svctype: SVCType) -> String {
    match svctype {
        SVCType::Genshin => "e202102251931481".to_owned(),
        SVCType::Honkai => "e202110291205111".to_owned(),
        SVCType::GenshinCN => "e202009291139501".to_owned(),
        SVCType::HonkaiCN => "e202006291139501".to_owned(),
    }
}

static API_SERVER: &str = "https://hk4e-api-os.hoyoverse.com";

#[tokio::main]
async fn main() {
    let cfg = read_config(&ARGS.config);
    let client = Client::builder()
        .user_agent(cfg.http.user_agent)
        .build()
        .unwrap();

    for session in cfg.sessions {
        let svctype = config::convert_svctype(&session.svctype).unwrap();
        let actid = get_actid(svctype);
        let url = format!("{}/event/sol/sign?act_id={}&lang=ko-kr", API_SERVER, actid);
        let mut headers = header::HeaderMap::new();
        let cookie_str = format!("ltoken={}; ltuid={}", session.ltoken, session.ltuid);
        let cookie = header::HeaderValue::from_str(&cookie_str).unwrap();
        headers.insert("Cookie", cookie);

        let msg = client.post(url).headers(headers).send().await.unwrap();
        println!(
            "Query result of {}: {}",
            session.ltuid,
            msg.text().await.unwrap()
        );
    }
}
