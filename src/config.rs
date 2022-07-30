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
