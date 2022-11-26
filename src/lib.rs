/// Wasmtime runtime support


pub use wasm_embedded_spec::{self as spec};

use spec::Error;
use spec::api::{UserErrorConversion, types::Errno};

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};


mod gpio;
mod spi;
mod i2c;
mod uart;

/// Wasmtime runtime object
pub struct WasmtimeRuntime<E> {
    _engine: wasmtime::Engine,
    linker: Linker<Context<E>>,
    store: Store<Context<E>>,
}

struct Context<E> {
    wasi: WasiCtx,
    engine: E,
}

pub trait Engine: spec::gpio::Gpio + spec::i2c::I2c + spec::spi::Spi + spec::uart::Uart {}

impl <T> Engine for T where
    T: spec::gpio::Gpio + spec::i2c::I2c + spec::spi::Spi + spec::uart::Uart,
{
}

impl <E: Engine> Context<E> {
    pub fn new(engine: E) -> Self {

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build();

        Context {
            wasi,
            engine
        }
    }
}

impl <E: Engine> UserErrorConversion for Context<E> {
    fn errno_from_error(&mut self, e: spec::Error) -> Result<spec::api::types::Errno, anyhow::Error> {
        match e {
            Error::InvalidArg => Ok(Errno::InvalidArg),
            Error::Unexpected => Ok(Errno::Unexpected),
            Error::Failed => Ok(Errno::Failed),
            Error::NoDevice => Ok(Errno::NoDevice),
            Error::Unsupported => Ok(Errno::Unsupported),
        }
    }
}

impl <E: Engine + 'static> WasmtimeRuntime<E> {
    /// Create a new WasmtimeRuntime with the provided engine and application
    pub fn new(engine: E, bin: &[u8]) -> anyhow::Result<Self> {
        // Create new linker with the provided engine
        let wasm_engine = wasmtime::Engine::default();
        let mut linker = Linker::new(&wasm_engine);

        // Setup store and context
        let context = Context::new(engine);
        let mut store = Store::new(&wasm_engine, context);

        // Bind WASI
        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut Context<E>| &mut ctx.wasi )?;

        // Bind drivers
        spec::api::gpio::add_to_linker(&mut linker, move |c: &mut Context<E>| c)?;
        spec::api::spi::add_to_linker(&mut linker, move |c: &mut Context<E>| c)?;
        spec::api::i2c::add_to_linker(&mut linker, move |c: &mut Context<E>| c)?;
        spec::api::uart::add_to_linker(&mut linker, move |c: &mut Context<E>| c)?;

        // Load module from file
        let module = Module::from_binary(&wasm_engine, bin)?;
        linker.module(&mut store, "", &module)?;

        Ok(Self{ _engine: wasm_engine, linker, store })
    }

    /// Run the loaded application
    pub fn run(&mut self) -> anyhow::Result<()> {
        let Self{linker, ..} = self;

        linker
            .get_default(&mut self.store, "")?
            .typed::<(), (), _>(&self.store)?
            .call(&mut self.store, ())?;

        Ok(())
    }
}
