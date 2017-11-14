//! Automatically generated rust module for 'pos.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RoomPosition<'a> {
    pub x: i32,
    pub y: i32,
    pub roomName: Cow<'a, str>,
}

impl<'a> RoomPosition<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = r.read_int32(bytes)?,
                Ok(16) => msg.y = r.read_int32(bytes)?,
                Ok(26) => msg.roomName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RoomPosition<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.x) as u64)
        + 1 + sizeof_varint(*(&self.y) as u64)
        + 1 + sizeof_len((&self.roomName).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.x))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.y))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.roomName))?;
        Ok(())
    }
}

