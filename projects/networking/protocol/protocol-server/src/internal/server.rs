use std::io::ErrorKind::{Interrupted, WouldBlock};
use std::{net::TcpListener, time::Duration};

use super::connection::RealClientConnection;

pub struct NotchianServer {
    tcp_listener: TcpListener,
    handshaking_connections: Vec<RealClientConnection>,
}

impl NotchianServer {
    pub fn new() -> anyhow::Result<Self> {
        let tcp_listener = TcpListener::bind("0.0.0.0:25565")?;
        tcp_listener.set_nonblocking(true)?;

        Ok(Self {
            tcp_listener,
            handshaking_connections: Vec::new(),
        })
    }

    pub fn tick(&mut self, duration: Duration) {
        self.handle_new_connection();
        self.empty_sockets();
    }

    fn handle_new_connection(&mut self) {
        let accept_result = self.tcp_listener.accept();
        match accept_result {
            Ok((stream, addr)) => {
                println!("New connection {addr}");
                stream.set_nonblocking(true).unwrap();
                stream.set_nodelay(true).unwrap();

                self.handshaking_connections
                    .push(RealClientConnection::new(stream));
            }
            Err(e) => {
                if e.kind() != WouldBlock {
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    fn empty_sockets(&mut self) {
        self.handshaking_connections.retain_mut(|connection| {
            connection.empty_tcp_socket().unwrap_or_else(|e| {
                if e.kind() == WouldBlock || e.kind() == Interrupted {
                    true
                } else {
                    println!("{e}");
                    false
                }
            })
        });
    }
}
