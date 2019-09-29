mod config;
mod tunnel;

use std::env::args;
use std::process::{Command, Stdio};

use crate::config::{JumpServer, MongoConfig, MySqlConfig};

#[derive(Debug, Clone)]
pub struct HostPort {
    host: String,
    port: u16,
}

fn with_tunnel<T, F: FnOnce(&tunnel::Tunnel) -> T>(
    target: HostPort,
    js: JumpServer,
    f: F,
) -> Result<T, tunnel::TunnelError> {
    println!("create tunnel");
    let tunnel = tunnel::Tunnel::make(target, js)?;
    let ret = f(&tunnel);
    println!("close tunnel");
    tunnel.close()?;
    Ok(ret)
}

fn main() {
    if args().len() != 2 {
        println!("Usage: dbcli <db_name>");
        return;
    }
    let db = args().nth(1).unwrap();

    let mut config_path = dirs::home_dir().expect("can not find home dir");
    config_path.push(".dbcli");
    let config = match config::parse(&config_path) {
        Ok(d) => d,
        Err(config::Error::IO(e)) => {
            panic!("parse config: {:?} failed, reason: {:?}", config_path, e)
        }
        Err(config::Error::Toml(e)) => {
            panic!("parse config: {:?} failed, reason: {:?}", config_path, e)
        }
    };

    let js = config.jump_server.clone();

    if let Some(db) = config.find_mysql(&db) {
        let hp = HostPort {
            host: db.host.clone(),
            port: db.port,
        };
        let cli = match config.client.map(|r| r.mysql.clone()) {
            Some(Some(d)) => d,
            _ => "mysql".into(),
        };
        with_tunnel(hp, js, |tunnel| handle_mysql(tunnel, db, cli)).unwrap()
    } else if let Some(db) = config.find_mongo(&db) {
        let hp = HostPort {
            host: db.host.clone(),
            port: db.port,
        };
        let cli = match config.client.map(|r| r.mongo.clone()) {
            Some(Some(d)) => d,
            _ => "mongo".into(),
        };
        with_tunnel(hp, js, |tunnel| handle_mongo(tunnel, db, cli)).unwrap()
    } else {
        println!("can not find db: {} in config file!", db)
    }
}

fn handle_mysql(tunnel: &tunnel::Tunnel, config: MySqlConfig, cli: String) {
    let proxy = tunnel.tunnel();
    Command::new(cli)
        .arg(format!("-P{}", proxy.port))
        .arg(format!("-h{}", proxy.host))
        .arg(format!("-u{}", &config.username))
        .arg(format!("-p{}", &config.password))
        .arg(format!("-D{}", &config.db))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("execute mysql failed");
}

fn handle_mongo(tunnel: &tunnel::Tunnel, config: MongoConfig, cli: String) {
    let proxy = tunnel.tunnel();
    let db = format!("{}:{}/{}", proxy.host, proxy.port, config.db);
    Command::new(cli)
        .arg(db)
        .arg("-u")
        .arg(config.username)
        .arg("-p")
        .arg(config.password)
        .arg("--authenticationDatabase")
        .arg("admin")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("execute mongo failed");
}
