use std::ops::{Deref, DerefMut};

use wasm_embedded_spec::{Error, spi::Spi, api::types};

use super::{Context, Engine};

use wiggle::GuestPtr;

impl <E: Engine> wasm_embedded_spec::api::spi::Spi for Context<E> {
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

        Spi::init(&mut self.engine, dev, baud, mosi, miso, sck, cs)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        log::debug!("Closing SPI handle: {}", handle);

        Spi::deinit(&mut self.engine, handle)
    }

    fn read<'a>(&mut self, handle: i32, data: &types::Rbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let mut d1 = d.as_slice_mut().unwrap();

        log::debug!("SPI Read {}", handle);

        Spi::read(&mut self.engine, handle, d1.deref_mut())
    }

    fn write<'a>(&mut self, handle: i32, data: &types::Wbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let d1 = d.as_slice().unwrap();

        log::debug!("SPI Write {} data: {:02x?}", handle, d1.deref());

        Spi::write(&mut self.engine, handle, d1.deref())
    }

    fn transfer<'a>(&mut self, handle: i32, read: &types::Rbytes<'a>, write: &types::Wbytes<'a>) -> Result<(), Error> {
        let r = read.ptr.as_array(read.len);
        let mut r1 = r.as_slice_mut().unwrap();

        let w = write.ptr.as_array(write.len);
        let w1 = w.as_slice().unwrap();

        log::debug!("SPI Transfer {} data: {:02x?}", handle, w1.deref());

        Spi::transfer(&mut self.engine, handle, r1.deref_mut(), w1.deref())
    }

    fn transfer_inplace<'a>(&mut self, handle: i32, data: &types::Rbytes<'a>) -> Result<(), Error> {
        let d = data.ptr.as_array(data.len);
        let mut d1 = d.as_slice_mut().unwrap();

        log::debug!("SPI TransferInplace {} data: {:02x?}", handle, d1.deref());

        Spi::transfer_inplace(&mut self.engine, handle, d1.deref_mut())
    }

    fn exec<'a>(&mut self, _handle: i32, ops: &GuestPtr<'a, [types::Op]>) -> Result<(), Error> {
        let _num_ops = ops.len();
        // TODO: idk yet how guest types etc. are going to work here
        todo!()
    }
}