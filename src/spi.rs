use std::ops::{Deref, DerefMut};

use wasm_embedded_spec::{self as spec};

use spec::Error;
use spec::api::{types, UserErrorConversion};

use embedded_hal::digital::{PinState};

use super::Context;

use wasmtime::*;
use wiggle::GuestPtr;

impl spec::api::spi::Spi for Context {
    fn init(
        &mut self,
        dev: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    ) -> Result<i32, Error> {
        log::debug!(
            "Opening SPI device: {} (baud: {} mosi: {} miso: {} sck: {} cs: {})",
            dev,
            baud,
            mosi,
            miso,
            sck,
            cs
        );
        
        let spi = match self.spi.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        spi.init(dev, baud, mosi, miso, sck, cs)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing SPI handle: {}", handle);

        let spi = match self.spi.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        spi.deinit(handle)
    }

    fn write<'a>(&mut self, handle: i32, data: &types::Wbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice().unwrap();

        log::debug!("Write SPI {} data: {:02x?}", handle, d1.deref());

        let spi = match self.spi.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        spi.write(handle, d1.deref())
    }

    fn transfer<'a>(&mut self, handle: i32, data: &types::Rbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let mut d1 = d.as_slice_mut().unwrap();

        log::debug!("Transfer SPI {} data: {:02x?}", handle, d1.deref());

        let spi = match self.spi.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        spi.transfer(handle, d1.deref_mut())
    }

    fn exec<'a>(&mut self, _handle: i32, ops: &GuestPtr<'a, [types::Op]>) -> Result<(), Error> {
        let _num_ops = ops.len();
        // TODO: idk yet how guest types etc. are going to work here
        todo!()
    }
}