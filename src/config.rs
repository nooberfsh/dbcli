use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
    pub jump_server: JumpServer,
    pub mysql_dbs: Option<Vec<MySqlConfig>>,
    pub mongo_dbs: Option<Vec<MongoConfig>>,
}

impl DBConfig {
    pub fn find_mysql(&self, name: &str) -> Option<MySqlConfig> {
        for db in self.mysql_dbs.as_ref()? {
            if db.db == name {
                return Some(db.clone())
            }
        }
        None
    }

    pub fn find_mongo(&self, name: &str) -> Option<MongoConfig> {
        for db in self.mongo_dbs.as_ref()? {
            if db.db == name {
                return Some(db.clone())
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
    pub password: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String
}
