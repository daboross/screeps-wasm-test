//! Automatically generated rust module for 'everything.proto' file

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
pub struct World<'a> {
    pub spawns: Vec<spawn::StructureSpawn<'a>>,
    pub creeps: Vec<creep::Creep<'a>>,
    pub rooms: Vec<room::Room<'a>>,
}

impl<'a> World<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spawns.push(r.read_message(bytes, spawn::StructureSpawn::from_reader)?),
                Ok(18) => msg.creeps.push(r.read_message(bytes, creep::Creep::from_reader)?),
                Ok(26) => msg.rooms.push(r.read_message(bytes, room::Room::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for World<'a> {
    fn get_size(&self) -> usize {
        0
        + self.spawns.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.creeps.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.rooms.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.spawns { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.creeps { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.rooms { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

