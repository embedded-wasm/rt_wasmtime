
use wiggle::GuestPtr;

use wasm_embedded_spec::{
    Error,
    api::{
        core::Core,
        types::{Rbytes, Wbytes},
    },
};

use super::{Engine, Context};

/// Wrapper for wiggle-generated core api
impl <E: Engine> wasm_embedded_spec::api::core::Core for Context<E> {
    fn exec(&mut self, cla: u32, flags: u32, ins: u32, handle: i32, cmd: &GuestPtr<u8>, cmd_len: u32, resp: &GuestPtr<u8>, resp_len: u32) -> Result<i32, wasm_embedded_spec::Error> { 
        log::debug!("Core::exec CLA: {:08x} INS: {:08x} flags: {:08x} handle: {}", cla, ins, flags, handle);


        todo!()
    }
}
