//! Automatically generated rust module for 'room.proto' file

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
pub struct XYPos {
    pub x: u32,
    pub y: u32,
}

impl XYPos {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = r.read_uint32(bytes)?,
                Ok(16) => msg.y = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for XYPos {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.x) as u64)
        + 1 + sizeof_varint(*(&self.y) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.x))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.y))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Room<'a> {
    pub name: Cow<'a, str>,
    pub energyAvailable: i32,
    pub energyCapacityAvailable: i32,
    pub sources: Vec<XYPos>,
}

impl<'a> Room<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.energyAvailable = r.read_int32(bytes)?,
                Ok(24) => msg.energyCapacityAvailable = r.read_int32(bytes)?,
                Ok(34) => msg.sources.push(r.read_message(bytes, XYPos::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Room<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.energyAvailable) as u64)
        + 1 + sizeof_varint(*(&self.energyCapacityAvailable) as u64)
        + self.sources.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.energyAvailable))?;
        w.write_with_tag(24, |w| w.write_int32(*&self.energyCapacityAvailable))?;
        for s in &self.sources { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

