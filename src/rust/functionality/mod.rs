use quick_protobuf::errors::Error as QpError;

use {decoding, utils};

pub fn debug_room_position(input: &[u8]) -> Result<(), QpError> {
    let pos = decoding::read_protobuf_room_position(input)?;

    utils::print(format!("debug_room_position: found {:#?}", pos));

    Ok(())
}
