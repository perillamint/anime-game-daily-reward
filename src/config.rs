// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct HttpConfig {
    pub user_agent: String,
}

#[derive(Deserialize)]
pub(crate) struct Session {
    pub svctype: String,
    pub ltoken: String,
    pub ltuid: i64,
}

#[derive(Deserialize)]
pub(crate) struct Config {
    pub http: HttpConfig,
    pub sessions: Vec<Session>,
}

pub(crate) fn parse_toml(tomlstr: &str) -> Config {
    let cfg: Config = toml::from_str(tomlstr).expect("Invalid config file");

    // TODO: Sanitize the value?

    cfg
}

pub(crate) fn read_config(cfgpath: &str) -> Config {
    match fs::read_to_string(cfgpath) {
        Ok(x) => parse_toml(&x),
        Err(_) => panic!("Config file not found!"),
    }
}

pub(crate) enum SVCType {
    Genshin,
    Honkai,
    GenshinCN,
    HonkaiCN,
}

pub(crate) fn convert_svctype(svctype: &str) -> Result<SVCType, String> {
    match svctype {
        "genshin" => Ok(SVCType::Genshin),
        "honkai" => Ok(SVCType::Honkai),
        "genshin_cn" => Ok(SVCType::GenshinCN),
        "honkai_cn" => Ok(SVCType::HonkaiCN),
        _ => Err("Invalid svctype".to_string()),
    }
}
