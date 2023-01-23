use std::fmt::Display;

use bytes::{Buf, Bytes};
use tokio::net::TcpStream;

#[macro_use]
mod macros;
mod message_type;
use message_type::MessageType;

const MESSAGE_TYPE_BYTE_LENGTH: usize = 1;
const MESSAGE_USERNAME_LENGTH_BYTE_LENGTH: usize = 1;
const MESSAGE_BODY_LENGTH_BYTE_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub description: String,
}

impl Error {
    pub fn new(description: &str) -> Self {
        Error {
            description: String::from(description),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

/// representing single message
///
/// constructed from TcpStream use [`Message::from_tcp_stream`]
pub struct Message {
    pub message_type: MessageType,
    username_length: u8,
    pub username: String,
    body_length: u32,
}

impl Message {

    pub fn new(message_type: MessageType, username: String) -> Self {
        let username_length = username.len() as u8;
        let body_length = message_type.body_length();

        Message {
            message_type,
            username,
            username_length,
            body_length
        }
    }

    pub async fn from_tcp_stream(tcp_stream: &mut TcpStream) -> Result<Self> {
        let message_type = read_from_reader!(MESSAGE_TYPE_BYTE_LENGTH, tcp_stream, "type").await?;

        let username_length =
            read_from_reader!(MESSAGE_USERNAME_LENGTH_BYTE_LENGTH, tcp_stream, "length").await?;

        let username = read_from_reader!(username_length.len(), tcp_stream, "username").await?;

        let body_length =
            read_from_reader!(MESSAGE_BODY_LENGTH_BYTE_LENGTH, tcp_stream, "body length").await?;

        let body = read_from_reader!(body_length.len(), tcp_stream, "body").await?;

        Ok(Message {
            message_type: Message::get_message_type(message_type, body),
            username_length: Message::get_username_length(username_length),
            username: Message::get_username(username),
            body_length: Message::get_body_length(body_length),
        })
    }

    fn get_message_type(byte: Bytes, body: Bytes) -> MessageType {
        let message_type = MessageType::parse(*byte.first().unwrap(), body).unwrap();

        message_type
    }

    fn get_username_length(bytes: Bytes) -> u8 {
        let mut b = bytes;
        b.get_u8()
    }

    fn get_username(bytes: Bytes) -> String {
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn get_body_length(bytes: Bytes) -> u32 {
        let mut b = bytes;
        b.get_u32()
    }
}
