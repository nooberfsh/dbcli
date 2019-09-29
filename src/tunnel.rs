use crate::config::JumpServer;
use crate::HostPort;

use std::io;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};

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
    BindPort(u16)
}

fn get_available_port(base: u16) -> u16 {
    // TODO:
    base
}

fn is_listen(addr: &SocketAddr) -> bool {
    match TcpStream::connect(addr) {
        Ok(_) => true,
        Err(_) => false
    }
}

impl Tunnel {
    pub fn make(target: HostPort, jump_server: JumpServer) -> Result<Tunnel, TunnelError> {
        let new_port = get_available_port(target.port);
        let map = format!("{}:{}:{}", new_port, target.host, target.port);
        let jump = format!("{}@{}", jump_server.username, jump_server.host);

        println!("cmd: ssh   -N -L {} {} ", map, jump);

        let child = Command::new("ssh")
            .arg("-N")
            .arg("-L")
            .arg(map)
            .arg(jump)
            .spawn()?;

        let ip = Ipv4Addr::new(127, 0, 0,1);
        let addr = SocketAddr::new(IpAddr::V4(ip), new_port);
        let mut count = 50;
        while  count > 0 && !is_listen(&addr) {
            println!("wait for tunnel process to start...");
            sleep(Duration::from_millis(200));
            count -= 1;
        }

        if count == 0 {
            return Err(TunnelError::BindPort(new_port))
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
