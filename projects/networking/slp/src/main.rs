use {
    byteorder::{BigEndian, ReadBytesExt, WriteBytesExt},
    slp::{
        types::{
            vars::{Var, VarInt},
            McStr
        },
        Codec, DecodeError, EncodeError, Sizeable
    },
    std::{
        ascii,
        error::Error,
        io::Take,
        net::{TcpListener, TcpStream}
    }
};

pub trait Encoder: std::io::Write {
    fn encode<T: Codec>(&mut self, data: T) -> Result<(), EncodeError> {
        data.encode_to(self)
    }
}

pub trait Decoder: std::io::Read {
    fn decode<T: Codec>(&mut self) -> Result<T, DecodeError> {
        T::decode_from(self)
    }
}

use std::io::Read;

impl<T: Read> Decoder for Take<T> {}

impl Encoder for TcpStream {}
impl Decoder for TcpStream {}

fn main() -> Result<(), Box<dyn Error>> {
    let srv = TcpListener::bind("127.0.0.1:25565")?;
    let (mut stream, _) = srv.accept()?;

    let mut state = 0; // HANDSHAKE

    loop {
        let Var(len) = stream.decode::<VarInt>()?;

        let mut packet = stream.by_ref().take(len as u64);
        let Var(id) = packet.decode::<VarInt>()?;

        println!("received {len}-byte long packet with id: {id}");

        if state == 0 && id == 0 {
            println!("protocol version: {}", stream.decode::<VarInt>()?.0);
            println!("host name: {}", &*stream.decode::<McStr>()?);
            println!("port: {}", stream.read_u16::<BigEndian>()?);

            state = stream.decode::<VarInt>()?.0;
            println!("next state: {state}");

            continue;
        }

        if state == 1 && id == 0 {
            use std::io::Write;

            println!("sending status response");

            let status: McStr = include_str!("status.json").into();

            stream.encode(Var(status.size() + 1))?;
            stream.write_u8(0)?;

            stream.encode(status)?;

            stream.flush()?;

            continue;
        }

        for b in stream.by_ref().bytes() {
            if let Ok(byte) = b {
                println!("{}", ascii::escape_default(byte));
            }
        }
    }
}
