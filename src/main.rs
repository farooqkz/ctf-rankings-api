use std::env::{args, current_dir};
use anyhow::Result;
use std::fs::read;
use std::path::PathBuf;
use std::str::from_utf8;
use tide::Request;
use tide::prelude::*;

struct Config {
    redis_addr: String,
    listen_addr: String,
}

fn main() -> anyhow::Result<()> {
    let config: Config;
    {
        let mut config_file_path = current_dir()?.join("config.toml");
        if let Some(arg) = args().nth(1) {
            if arg == "-h" || arg == "--help" {
                println!("Usage: {} [/path/to/config/file.toml]", args().nth(0).unwrap());
                return Ok(());
            }
            config_file_path = PathBuf::from(arg);
        }
        config = toml::from_str(from_utf8(&read(config_file_path)?)?)?;
    }

    let mut app = tide::new();
    let client = redis::Client::open(config.redis_addr.as_str())?;
    app.at("/:mode/:username").get(|req| { get_ranking_for_mode(req, client.get_connection()?) });
    app.at("/:username").get(|req| { get_ranking_all_modes(req, client.get_connection()?) });
    app.listen(config.listen_addr)?;
}

async fn get_ranking_for_mode(req: Request<()>, connection: _) -> tide::Result<String> {

}

async fn get_ranking_all_modes(req: Request<()>, connection: _) -> tide::Result<String> {

}
