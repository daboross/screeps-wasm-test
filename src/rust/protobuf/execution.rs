//! Automatically generated rust module for 'execution.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BodyPart {
    WORK = 1,
    MOVE = 2,
    CARRY = 3,
    ATTACK = 4,
    RANGED_ATTACK = 5,
    HEAL = 6,
    TOUGH = 7,
    CLAIM = 8,
}

impl Default for BodyPart {
    fn default() -> Self {
        BodyPart::WORK
    }
}

impl From<i32> for BodyPart {
    fn from(i: i32) -> Self {
        match i {
            1 => BodyPart::WORK,
            2 => BodyPart::MOVE,
            3 => BodyPart::CARRY,
            4 => BodyPart::ATTACK,
            5 => BodyPart::RANGED_ATTACK,
            6 => BodyPart::HEAL,
            7 => BodyPart::TOUGH,
            8 => BodyPart::CLAIM,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreepSpawn<'a> {
    pub spawn_name: Cow<'a, str>,
    pub body: Vec<BodyPart>,
    pub creep_name: Cow<'a, str>,
}

impl<'a> CreepSpawn<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spawn_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.body.push(r.read_enum(bytes)?),
                Ok(26) => msg.creep_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CreepSpawn<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.spawn_name).len())
        + self.body.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
        + 1 + sizeof_len((&self.creep_name).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.spawn_name))?;
        for s in &self.body { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        w.write_with_tag(26, |w| w.write_string(&**&self.creep_name))?;
        Ok(())
    }
}

