use std::ops::{Deref, DerefMut};

use wasm_embedded_spec::{Error, i2c::I2c, api::types};

use super::{Context, Engine};

/// Wrapper for wiggle-generated I2C api
impl <E: Engine> wasm_embedded_spec::api::i2c::I2c for Context<E> {
    fn init(&mut self, port: u32, baud: u32, sda: i32, scl: i32) -> Result<i32, Error> {
        log::debug!(
            "Opening I2C port: {} (baud: {} sda: {} scl: {})",
            port,
            baud,
            sda,
            scl
        );

        I2c::init(&mut self.engine, port, baud, sda, scl)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing I2C handle: {}", handle);

        I2c::deinit(&mut self.engine, handle)
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

        I2c::write(&mut self.engine, handle, addr, d1.deref())
    }

    /// Read from an I2c device
    fn read(&mut self, handle: i32, addr: u16, buff: &types::Wbytes) -> Result<(), Error> {
        let b = buff.ptr.as_array(buff.len);
        let mut b1 = b.as_slice_mut().unwrap();

        log::debug!("I2C read handle: {} addr: {}", handle, addr);

        I2c::read(&mut self.engine, handle, addr, b1.deref_mut())
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

        I2c::write_read(&mut self.engine, handle, addr, d1.deref(), b1.deref_mut())
    }
}
