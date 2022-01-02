use std::ops::{Deref, DerefMut};
use wasm_embedded_spec::{self as spec};

use spec::Error;
use spec::api::{types, UserErrorConversion};
use embedded_hal::digital::{PinState};

use super::Context;
use wasmtime::*;

impl spec::api::i2c::I2c for Context {
    fn init(&mut self, port: u32, baud: u32, sda: i32, scl: i32) -> Result<i32, Error> {
        log::debug!(
            "Opening I2C port: {} (baud: {} sda: {} scl: {})",
            port,
            baud,
            sda,
            scl
        );
        
        let i2c = match self.i2c.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        i2c.init(port, baud, sda, scl)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing I2C handle: {}", handle);

        let i2c = match self.i2c.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        i2c.deinit(handle)
    }

    /// Write to an I2c device
    fn write(&mut self, handle: i32, addr: u16, data: &types::Rbytes) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice_mut().unwrap();

        log::debug!(
            "I2C write handle: {} addr: {} data: {:02x?}",
            handle,
            addr,
            d1.deref()
        );

        let i2c = match self.i2c.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        i2c.write(handle, addr, d1.deref())
    }

    /// Read from an I2c device
    fn read(&mut self, handle: i32, addr: u16, buff: &types::Wbytes) -> Result<(), Error> {
        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap();

        log::debug!("I2C read handle: {} addr: {}", handle, addr);

        let i2c = match self.i2c.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        i2c.read(handle, addr, b1.deref_mut())
    }

    /// Write to and read from an I2c device on the specified peripheral
    fn write_read(
        &mut self,
        handle: i32,
        addr: u16,
        data: &types::Rbytes,
        buff: &types::Wbytes,
    ) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice().unwrap();

        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap();

        log::debug!(
            "I2C write_read dev: {} addr: {} write: {:02x?}",
            handle,
            addr,
            d1.deref()
        );

        let i2c = match self.i2c.as_deref_mut() {
            Some(d) => d,
            None => return Err(spec::Error::Unsupported),
        };

        i2c.write_read(handle, addr, d1.deref(), b1.deref_mut())
    }
}