use anyhow::{Context, Ok};

pub struct Config {
    pub db_url: String,
    pub listen_port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let port: u16 = std::env::var("PORT")
            .context("PORT must be set")?
            .parse()
            .context("PORT must be a valid u16 number")?;
        Ok(Config {
            db_url,
            listen_port: port,
        })
    }
}
