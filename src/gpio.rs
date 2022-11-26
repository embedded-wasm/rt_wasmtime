

use wasm_embedded_spec::{Error, gpio::Gpio, api::types};

use embedded_hal::digital::{PinState};

use super::{Engine, Context};

/// Wrapper for wiggle-generated GPIO api
impl <E: Engine> wasm_embedded_spec::api::gpio::Gpio for Context<E> {
    /// Initialise the provided GPIO pin in input or output mode
    fn init(&mut self, port: i32, pin: i32, mode: types::Mode) -> Result<i32, Error> {
        log::debug!("GPIO init port: {} pin: {} mode: {:?}", port, pin, mode);

        Gpio::init(&mut self.engine, port, pin, mode == types::Mode::Output)
    }

    /// Deinitialise the specified GPIO pin
    fn deinit(&mut self, dev: i32) -> Result<(), Error> {
        log::debug!("GPIO deinit handle: {}", dev);

        Gpio::deinit(&mut self.engine, dev)
    }

    /// Write to a GPIO pin
    fn set(&mut self, dev: i32, value: types::Value) -> Result<(), Error> {
        log::debug!("GPIO write handle: {} val: {:?}", dev, value);

        let state = match value {
            types::Value::High => PinState::High,
            types::Value::Low => PinState::Low,
        };

        Gpio::set(&mut self.engine, dev, state)
    }

    // Read from a GPIO pin
    fn get(&mut self, dev: i32) -> Result<types::Value, Error> {
        log::debug!("GPIO read handle: {}", dev);

        let r = Gpio::get(&mut self.engine, dev)?;

        match r {
            PinState::High => Ok(types::Value::High),
            PinState::Low => Ok(types::Value::Low),
        }
    }
}
