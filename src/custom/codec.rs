use bincode;
use bytes::buf::BufMut;
use bytes::BytesMut;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    Connect { name: String },
    Ping,
    Message { msg: String },
}

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
}

fn parsing_error(error: &bincode::Error) -> tokio::io::Error {
    tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, format!("{:?}", error))
}

impl tokio::codec::Decoder for Codec {
    type Item = Packet;
    type Error = tokio::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, tokio::io::Error> {
        match bincode::deserialize::<Self::Item>(&buf[..]) {
            Ok(item) => {
                let parsed_size = match bincode::serialized_size(&item) {
                    Ok(parsed_size) => parsed_size,
                    Err(e) => return Err(parsing_error(&e)),
                };

                buf.split_to(parsed_size as usize);
                Ok(Some(item))
            }
            Err(e) => match *e {
                bincode::ErrorKind::Io(_err) => Ok(None),
                // TODO: Check for other interesting errors
                _ => Err(parsing_error(&e)),
            },
        }
    }
}

impl tokio::codec::Encoder for Codec {
    type Item = Packet;
    type Error = tokio::io::Error;

    fn encode(&mut self, packet: Self::Item, buf: &mut BytesMut) -> Result<(), tokio::io::Error> {
        match bincode::serialize::<Self::Item>(&packet) {
            Ok(vec) => {
                buf.reserve(vec.len());
                buf.put(vec);
                Ok(())
            }
            Err(e) => Err(parsing_error(&e)),
        }
    }
}
