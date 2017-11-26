use std::slice;

use quick_protobuf::BytesReader;
use quick_protobuf::errors::Error as QpError;

use protobuf::pos;


pub unsafe fn read_ptr<'a>(ptr: *mut u8, len: usize) -> &'a [u8] {
    slice::from_raw_parts(ptr, len)
}

pub unsafe fn consume_ptr(ptr: *mut u8, len: usize) -> Vec<u8> {
    Vec::from_raw_parts(ptr, len, len)
}

pub fn read_protobuf_room_position(input: &[u8]) -> Result<pos::RoomPosition, QpError> {
    // TODO: figure out if quick_protobuf is willing to add a read trait so this isn't duplicated
    // https://github.com/tafia/quick-protobuf/issues/78
    let mut r = BytesReader::from_bytes(input);

    r.read_message(input, pos::RoomPosition::from_reader)
}
