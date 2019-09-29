use std::fs::File;
use std::io::{self, Read};

use serde_derive::Deserialize;
use std::path::Path;
use toml::de::Error as TomlError;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub jump_server: JumpServer,
    pub mysql: Option<Vec<MySqlConfig>>,
    pub mongo: Option<Vec<MongoConfig>>,
}

impl Config {
    pub fn find_mysql(&self, name: &str) -> Option<MySqlConfig> {
        for db in self.mysql.as_ref()? {
            if db.db == name {
                return Some(db.clone());
            }
        }
        None
    }

    pub fn find_mongo(&self, name: &str) -> Option<MongoConfig> {
        for db in self.mongo.as_ref()? {
            if db.db == name {
                return Some(db.clone());
            }
        }
        None
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JumpServer {
    pub username: String,
    pub host: String,
    pub port: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MySqlConfig {
    pub db: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Toml(TomlError),
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let ret = toml::from_slice(&buf)?;
    Ok(ret)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<TomlError> for Error {
    fn from(e: TomlError) -> Self {
        Error::Toml(e)
    }
}
