use protocol_lib::PacketFrame;
use std::{
    io::{self, Read},
    net::TcpStream,
};

pub struct RealClientConnection {
    pub stream: TcpStream,
    pub connection_handler: ConnectionHandler,
}

impl RealClientConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            connection_handler: ConnectionHandler::new(),
        }
    }

    /// Empty the tcp socket and fill the decoder, return false if this connection should be discarded
    pub fn empty_tcp_socket(&mut self) -> io::Result<bool> {
        let dec = &mut self.connection_handler.dec;
        let mut buff = [0u8, 4096];
        loop {
            let n = self.stream.read(&mut buff)?;
            if n == 0 {
                return Ok(false);
            }
            dec.que
        }
    }
}

/// High level interface to get and pass deserialized minecraft packet
pub struct ConnectionHandler {
    enc: PacketEncoder,
    dec: PacketDecoder,
}

impl ConnectionHandler {
    fn new() -> Self {
        Self {
            enc: PacketEncoder::default(),
            dec: PacketDecoder::default(),
        }
    }

    pub fn received_packets(&mut self) -> IncomingPacketIterator {
        IncomingPacketIterator { handler: self }
    }
}

pub struct IncomingPacketIterator<'a> {
    handler: &'a mut ConnectionHandler,
}

impl Iterator for IncomingPacketIterator<'_> {
    type Item = PacketFrame;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.handler.dec.try_next_packet();
            if let Ok(frame) = frame {
                return Some(frame);
            }

            let error = frame.err().unwrap();
            println!(
                "encoded an error trying to deserialize a packet {}, ignoring",
                error.root_cause()
            )
        }
    }
}
