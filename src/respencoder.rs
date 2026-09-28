use crate::resp::RespProtocolDataType::{self, BulkString, SimpleString};
use std::todo;

const CRLF: &[u8] = b"\r\n";

pub struct RespEncoder {}

#[derive(Debug)]
pub enum EncoderError {
    Unknown
}

impl RespEncoder {
    pub fn encode(&mut self, value: RespProtocolDataType) -> Result<Vec<u8>, EncoderError> {
        match value {
            BulkString(val) => return Ok(Self::encode_bulk_string(val)),
            SimpleString(val) => return Ok(Self::encode_simple_string(val)),
            _ => return Err(EncoderError::Unknown)
        }
    }

    pub fn encode_bulk_string(msg: Vec<u8>) -> Vec<u8> {

        
        let mut out = b"$".to_vec();

        out.extend_from_slice(msg.len().to_string().as_bytes());
        out.extend_from_slice(CRLF);
        out.extend_from_slice(&msg);
        out.extend_from_slice(CRLF);
        out
    }

    pub fn encode_simple_string(msg: Vec<u8>) -> Vec<u8> {

        let mut out = b"+".to_vec();

        out.extend_from_slice(&msg);
        out.extend_from_slice(CRLF);
        out
    }
}