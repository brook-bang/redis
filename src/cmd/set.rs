use crate::cmd::{Parse, ParseError};
use crate::{Connection, Db, Frame};

use bytes::Bytes;
use std::time::Duration;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn new(key: impl ToString, value: Bytes, expire: Option<Duration>) -> Set {
        Set {
            key: key.to_string(),
            value,
            expire,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }    

    pub fn value(&self) -> &Bytes {
        &self.value
    }

    pub fn expire(&self) -> Option<Duration> {
        self.expire
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Set> {
        use ParseError::EndOfStream;
        let key = parse.next_string()?;
        let value = parse.next_bytes()?;
        let mut expire = None;
        match parse.next_string() {
            Ok(s) if s.to_uppercase() == "EX" => {
                
            }
        }
    }

}