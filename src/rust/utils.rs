use quick_protobuf::errors::Error as QpError;
use quick_protobuf::{self, MessageWrite};

use imports;

use protobuf::execution;

pub fn print<T: AsRef<str>>(thing: T) {
    let bytes = thing.as_ref().as_bytes();
    let ptr = bytes.as_ptr() as *mut u8;
    let len = bytes.len();
    unsafe {
        imports::print_str(ptr, len);
    }
}

// TODO: allow &[execution::BodyPart] if we modify pb-rs to have Cow<'a, []> members.
pub fn spawn_spawn_creep<T: AsRef<str>, U: AsRef<str>>(
    spawn_name: T,
    body: Vec<execution::BodyPart>,
    name: U,
) -> Result<i32, QpError> {
    let execution = execution::CreepSpawn {
        spawn_name: spawn_name.as_ref().into(),
        body: body,
        creep_name: name.as_ref().into(),
    };

    let mut buf = Vec::<u8>::new();
    execution.write_message(&mut quick_protobuf::Writer::new(&mut buf))?;

    let result = unsafe {
        let ptr = buf.as_slice().as_ptr() as *mut u8;
        let len = buf.len();

        imports::execute_spawn_spawn_creep(ptr, len)
    };

    Ok(result)
}
