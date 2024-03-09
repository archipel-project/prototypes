mod connection;

use crate::connection::RealClientConnection;
use std::io::ErrorKind::{Interrupted, WouldBlock};
use std::io::Read;
use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use valence_protocol::packets::handshaking::HandshakeC2s;
use valence_protocol::{Decode, PacketDecoder, MINECRAFT_VERSION};

struct MinecraftServer {
    tcp_listener: TcpListener,
    handshaking_connections: Vec<RealClientConnection>,
    decoder: PacketDecoder,
}
impl MinecraftServer {
    fn new() -> Self {
        let tcp_listener = TcpListener::bind("127.0.0.1:25565").unwrap();
        tcp_listener.set_nonblocking(true).unwrap();
        MinecraftServer {
            tcp_listener,
            handshaking_connections: vec![],
            decoder: PacketDecoder::new(),
        }
    }

    fn tick(&mut self, duration: Duration) {
        self.handle_new_connection();
        self.empty_sockets();
        self.process_handshaking_connection();
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

    fn process_handshaking_connection(&mut self) {
        for mut connection in &mut self.handshaking_connections {
            for packet in connection.connection_handler.received_packets() {
                let packet = packet.decode::<HandshakeC2s>();
                if packet.is_ok() {
                    println!("got an handshake packet : {:?}", packet.unwrap())
                }
            }
        }
    }
}

fn main() {
    println!("{}", MINECRAFT_VERSION);

    let mut server = MinecraftServer::new();

    loop {
        server.tick(Duration::from_millis(50));
        sleep(Duration::from_millis(50));
    }
}
