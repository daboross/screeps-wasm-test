use quick_protobuf::errors::Error as QpError;

use protobuf::execution::BodyPart;

use {decoding, utils};

pub fn debug_room_position(input: &[u8]) -> Result<(), QpError> {
    let pos = decoding::read_protobuf_room_position(input)?;

    utils::print(format!("debug_room_position: found {:#?}", pos));

    Ok(())
}

pub fn operate_world(input: &[u8]) -> Result<(), QpError> {
    let world = decoding::read_protobuf_world(input)?;

    for spawn in world.spawns {
        if !spawn.spawning && spawn.energy == spawn.energyCapacity {
            let result = utils::spawn_spawn_creep(
                &spawn.name,
                vec![
                    BodyPart::MOVE,
                    BodyPart::CARRY,
                    BodyPart::WORK,
                    BodyPart::MOVE,
                ],
                "hello",
            )?;
            if result != 0 {
                utils::print(format!(
                    "unknown result from {}.spawnCreep: {}",
                    spawn.name,
                    result
                ));
            }
        }
    }

    Ok(())
}
