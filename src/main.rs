use redis::Commands;
use serde::Deserialize;
use std::env::{args, current_dir};
use std::fs::read;
use std::path::PathBuf;
use std::str::from_utf8;
use tide::{Request, Response};

#[derive(Deserialize, Clone)]
struct Config {
    redis_addr: String,
    listen_addr: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config: Config;
    {
        let mut config_file_path = current_dir()?.join("config.toml");
        if let Some(arg) = args().nth(1) {
            if arg == "-h" || arg == "--help" {
                println!(
                    "Usage: {} [/path/to/config/file.toml]",
                    args().nth(0).unwrap()
                );
                return Ok(());
            }
            config_file_path = PathBuf::from(arg);
        }
        config = toml::from_str(from_utf8(&read(config_file_path)?)?)?;
    }

    let client = redis::Client::open(config.redis_addr.as_str())?;
    let mut app = tide::with_state(client);
    app.at("/:mode/:username")
        .get(get_ranking_for_mode);
    app.listen(config.listen_addr).await?;
    Ok(())
}

async fn get_ranking_for_mode(req: Request<redis::Client>) -> tide::Result {
    let mode = req.param("mode")?;
    let username = req.param("username")?;
    let client = req.state();
    let mut connection = client.get_connection()?;
    let result = connection
        .get(format!("{mode}|{username}"))
        .unwrap_or("".to_string());
    if result == "" {
        Ok(Response::builder(404).build())
    } else {
        Ok(Response::builder(200)
            .body(result)
            .content_type("application/json")
            .build())
    }
}
