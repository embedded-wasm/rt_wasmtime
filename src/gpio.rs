
use wasm_embedded_spec::{self as spec};

use spec::gpio::Gpio;
use spec::api::{types, UserErrorConversion};

use embedded_hal::digital::{PinState};

use super::{Driver, Context};
use wasmtime::*;

/// Wrapper for wiggle-generated GPIO api
impl <D: Driver> spec::api::gpio::Gpio for Context<D> {
    /// Initialise the provided GPIO pin in input or output mode
    fn init(&mut self, port: u32, pin: u32, mode: types::Mode) -> Result<i32, spec::Error> {
        log::debug!("GPIO init port: {} pin: {} mode: {:?}", port, pin, mode);

        Gpio::init(&mut self.driver, port, pin, mode == types::Mode::Output)
    }

    /// Deinitialise the specified GPIO pin
    fn deinit(&mut self, dev: i32) -> Result<(), spec::Error> {
        log::debug!("GPIO deinit handle: {}", dev);

        Gpio::deinit(&mut self.driver, dev)
    }

    /// Write to a GPIO pin
    fn set(&mut self, dev: i32, value: types::Value) -> Result<(), spec::Error> {
        log::debug!("GPIO write handle: {} val: {:?}", dev, value);

        let state = match value {
            types::Value::High => PinState::High,
            types::Value::Low => PinState::Low,
        };

        Gpio::set(&mut self.driver, dev, state)
    }

    // Read from a GPIO pin
    fn get(&mut self, dev: i32) -> Result<types::Value, spec::Error> {
        log::debug!("GPIO read handle: {}", dev);

        let r = Gpio::get(&mut self.driver, dev)?;

        match r {
            PinState::High => Ok(types::Value::High),
            PinState::Low => Ok(types::Value::Low),
        }
    }
}
