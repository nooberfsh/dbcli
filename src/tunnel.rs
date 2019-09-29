use crate::config::JumpServer;
use crate::HostPort;

use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Tunnel {
    target: HostPort,
    jump_server: JumpServer,
    tunnel: HostPort,
    child: Child,
}

#[derive(Debug)]
pub enum TunnelError {
    IO(io::Error),
    CannotFindAvailablePort,
    BindPort(u16),
}

fn get_available_port(base: u16, ip: IpAddr) -> Option<u16> {
    let mut port = base;
    while port < std::u16::MAX {
        let addr = SocketAddr::new(ip, port);
        if !is_listen(&addr) {
            return Some(port);
        }
        println!("port {} is  used, use next port", port);
        port += 1
    }
    None
}

fn is_listen(addr: &SocketAddr) -> bool {
    match TcpStream::connect(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl Tunnel {
    pub fn make(target: HostPort, jump_server: JumpServer) -> Result<Tunnel, TunnelError> {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let new_port = match get_available_port(target.port, ip) {
            Some(d) => d,
            None => return Err(TunnelError::CannotFindAvailablePort),
        };
        let map = format!("{}:{}:{}", new_port, target.host, target.port);
        let jump = format!("{}@{}", jump_server.username, jump_server.host);

        println!("making tunnel: ssh   -N -L {} {} ", map, jump);

        let child = Command::new("ssh")
            .arg("-N")
            .arg("-L")
            .arg(map)
            .arg(jump)
            .spawn()?;

        let addr = SocketAddr::new(ip, new_port);
        let mut count = 50;
        while count > 0 && !is_listen(&addr) {
            println!("wait for tunnel process to start...");
            sleep(Duration::from_millis(200));
            count -= 1;
        }

        if count == 0 {
            return Err(TunnelError::BindPort(new_port));
        }

        let ret = Tunnel {
            target,
            jump_server,
            child,
            tunnel: HostPort {
                host: "127.0.0.1".to_string(),
                port: new_port,
            },
        };
        Ok(ret)
    }

    pub fn tunnel(&self) -> HostPort {
        self.tunnel.clone()
    }

    pub fn close(mut self) -> Result<(), TunnelError> {
        self.child.kill()?;
        Ok(())
    }
}

impl From<io::Error> for TunnelError {
    fn from(e: io::Error) -> Self {
        TunnelError::IO(e)
    }
}
