mod config;
mod tunnel;

use std::env::args;
use std::process::{Command, Stdio};

use crate::config::{JumpServer, MySqlConfig};

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
        Err(config::Error::IO(e)) => panic!("parse config: {:?} failed, reason: {:?}", config_path, e),
        Err(config::Error::Toml(e)) => panic!("parse config: {:?} failed, reason: {:?}", config_path, e),
    };

    let js = config.jump_server.clone();

    if let Some(db) = config.find_mysql(&db) {
        let hp = HostPort {
            host: db.host.clone(),
            port: db.port,
        };
        with_tunnel(hp, js, |tunnel| handle_mysql(tunnel, db)).unwrap()
    } else if let Some(_db) = config.find_mongo(&db) {
    } else {
        println!("can not find db: {} in config file!", db)
    }
}

fn handle_mysql(tunnel: &tunnel::Tunnel, config: MySqlConfig) {
    let proxy = tunnel.tunnel();
    Command::new("mycli")
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
