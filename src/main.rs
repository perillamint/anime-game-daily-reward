// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use clap::Parser;
use reqwest::{header, Client};

mod config;

use config::{read_config, SVCType};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, short = 'c', value_name = "CONFIG")]
    config: String,
}

#[derive(Serialize, Deserialize)]
struct ApiPayload {
    act_id: String,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn get_svcpair(svctype: &SVCType) -> (String, String, String) {
    match svctype {
        SVCType::Genshin => (
            "sg-hk4e-api".to_owned(),
            "sol".to_owned(),
            "e202102251931481".to_owned(),
        ),
        SVCType::Honkai => (
            "sg-public-api".to_owned(),
            "mani".to_owned(),
            "e202110291205111".to_owned(),
        ),
        SVCType::HonkaiStarRail => (
            "sg-public-api".to_owned(),
            "luna/os".to_owned(),
            "e202303301540311".to_owned(),
        ),
        SVCType::GenshinCN => (
            "hk4e-api-os".to_owned(),
            "sol".to_owned(),
            "e202009291139501".to_owned(),
        ),
        SVCType::HonkaiCN => (
            "sg-public-api".to_owned(),
            "mani".to_owned(),
            "e202006291139501".to_owned(),
        ),
    }
}

#[tokio::main]
async fn main() {
    let cfg = read_config(&ARGS.config);
    let client = Client::builder()
        .user_agent(cfg.http.user_agent)
        .build()
        .unwrap();

    for session in cfg.sessions {
        let svctype = config::convert_svctype(&session.svctype).unwrap();
        let svcpair = get_svcpair(&svctype);
        let url = format!(
            "https://{}.hoyoverse.com/event/{}/sign?lang=ko-kr",
            svcpair.0, svcpair.1
        );
        let mut headers = header::HeaderMap::new();
        let cookie_str = format!("ltoken={}; ltuid={}", session.ltoken, session.ltuid);
        let cookie = header::HeaderValue::from_str(&cookie_str).unwrap();
        headers.insert("Cookie", cookie);

        let api_payload = ApiPayload {
            act_id: svcpair.2.clone(),
        };

        let msg = client
            .post(url)
            .headers(headers)
            .json(&api_payload)
            .send()
            .await
            .unwrap();
        println!(
            "Query result of {}: {}",
            session.ltuid,
            msg.text().await.unwrap()
        );
    }
}
