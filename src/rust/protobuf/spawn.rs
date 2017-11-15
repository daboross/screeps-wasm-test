//! Automatically generated rust module for 'spawn.proto' file

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
pub struct StructureSpawn<'a> {
    pub pos: pos::RoomPosition<'a>,
    pub hitsMax: i32,
    pub hits: i32,
    pub id: Cow<'a, str>,
    pub energy: i32,
    pub energyCapacity: i32,
    pub name: Cow<'a, str>,
    pub spawning: bool,
}

impl<'a> StructureSpawn<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = r.read_message(bytes, pos::RoomPosition::from_reader)?,
                Ok(16) => msg.hitsMax = r.read_int32(bytes)?,
                Ok(24) => msg.hits = r.read_int32(bytes)?,
                Ok(34) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.energy = r.read_int32(bytes)?,
                Ok(48) => msg.energyCapacity = r.read_int32(bytes)?,
                Ok(58) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.spawning = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StructureSpawn<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.pos).get_size())
        + 1 + sizeof_varint(*(&self.hitsMax) as u64)
        + 1 + sizeof_varint(*(&self.hits) as u64)
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_varint(*(&self.energy) as u64)
        + 1 + sizeof_varint(*(&self.energyCapacity) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.spawning) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.pos))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.hitsMax))?;
        w.write_with_tag(24, |w| w.write_int32(*&self.hits))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(40, |w| w.write_int32(*&self.energy))?;
        w.write_with_tag(48, |w| w.write_int32(*&self.energyCapacity))?;
        w.write_with_tag(58, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(64, |w| w.write_bool(*&self.spawning))?;
        Ok(())
    }
}

