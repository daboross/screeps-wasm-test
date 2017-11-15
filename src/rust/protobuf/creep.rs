//! Automatically generated rust module for 'creep.proto' file

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
pub struct CreepCarry<'a> {
    pub resourceType: Cow<'a, str>,
    pub amount: i32,
}

impl<'a> CreepCarry<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.resourceType = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.amount = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreepCarry<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.resourceType).len())
        + 1 + sizeof_varint(*(&self.amount) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.resourceType))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.amount))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Creep<'a> {
    pub pos: pos::RoomPosition<'a>,
    pub carry: Vec<CreepCarry<'a>>,
    pub carryCapacity: i32,
    pub fatigue: i32,
    pub hits: i32,
    pub hitsMax: i32,
    pub id: Cow<'a, str>,
    pub my: bool,
    pub name: Cow<'a, str>,
    pub spawning: bool,
    pub ticksToLive: i32,
}

impl<'a> Creep<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = r.read_message(bytes, pos::RoomPosition::from_reader)?,
                Ok(18) => msg.carry.push(r.read_message(bytes, CreepCarry::from_reader)?),
                Ok(24) => msg.carryCapacity = r.read_int32(bytes)?,
                Ok(32) => msg.fatigue = r.read_int32(bytes)?,
                Ok(40) => msg.hits = r.read_int32(bytes)?,
                Ok(48) => msg.hitsMax = r.read_int32(bytes)?,
                Ok(58) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.my = r.read_bool(bytes)?,
                Ok(74) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.spawning = r.read_bool(bytes)?,
                Ok(88) => msg.ticksToLive = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Creep<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.pos).get_size())
        + self.carry.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.carryCapacity) as u64)
        + 1 + sizeof_varint(*(&self.fatigue) as u64)
        + 1 + sizeof_varint(*(&self.hits) as u64)
        + 1 + sizeof_varint(*(&self.hitsMax) as u64)
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_varint(*(&self.my) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.spawning) as u64)
        + 1 + sizeof_varint(*(&self.ticksToLive) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.pos))?;
        for s in &self.carry { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_with_tag(24, |w| w.write_int32(*&self.carryCapacity))?;
        w.write_with_tag(32, |w| w.write_int32(*&self.fatigue))?;
        w.write_with_tag(40, |w| w.write_int32(*&self.hits))?;
        w.write_with_tag(48, |w| w.write_int32(*&self.hitsMax))?;
        w.write_with_tag(58, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(64, |w| w.write_bool(*&self.my))?;
        w.write_with_tag(74, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(80, |w| w.write_bool(*&self.spawning))?;
        w.write_with_tag(88, |w| w.write_int32(*&self.ticksToLive))?;
        Ok(())
    }
}

