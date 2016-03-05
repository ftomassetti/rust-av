#![cfg_attr(feature = "assignment_operators", feature(augmented_assignments, op_assign_traits))]
#[macro_use]
extern crate bitflags;

use std::sync::Arc;
use std::io::{Read, Write};
use std::unimplemented;

// use data::SideData;

bitflags! {
    flags PacketFlags: u32 {
        const KEY     = 0b0001,
        const CORRUPT = 0b0010,
    }
}

pub struct Packet {
    data : Arc<&[u8]>,
    pts : i64,
    dts : i64,
    pos : i64,
    stream_index : isize,

    // side_data : SideData;

    flags : PacketFlags,
}

impl Packet {
    pub fn new(len: usize) -> Self {
        unimplemented!();
    }
    pub fn from_slice(slice : &mut[u8]) -> Self {
        unimplemented!();
    }
    pub fn truncate(&mut self, len: usize) {
        unimplemented!();
    }
}

pub trait ReadPacket: Read {
    pub fn get_packet(&mut self, len: usize) -> Result<Packet> {
        let mut buffer = [0; len];
        try!(self.read(&mut buffer[..]);
        Ok(Packet::from_slice(&mut buffer[]))
    }
}
