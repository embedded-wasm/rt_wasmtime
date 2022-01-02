/// Wasmtime runtime support


pub use wasm_embedded_spec::{self as spec};

use spec::Error;
use spec::api::{types, UserErrorConversion, types::Errno};
use embedded_hal::digital::{PinState};

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};


mod gpio;
//mod spi;
//mod i2c;

pub struct WasmtimeRuntime<D> {
    engine: Engine,
    linker: Linker<Context<D>>,
    store: Store<Context<D>>,
    module: Module,
}

pub struct Context<D> {
    wasi: WasiCtx,
    driver: D,
}

pub trait Driver: spec::gpio::Gpio {}

impl <T> Driver for T where
    T: spec::gpio::Gpio,
{
}

impl <D: Driver> Context<D> {
    pub fn new(driver: D) -> Self {

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build();

        Context {
            wasi,
            driver
        }
    }
}

impl <D: Driver> UserErrorConversion for Context<D> {
    fn errno_from_error(&mut self, e: spec::Error) -> Result<spec::api::types::Errno, wiggle::Trap> {
        match e {
            Error::InvalidArg => Ok(Errno::InvalidArg),
            Error::Unexpected => Ok(Errno::Unexpected),
            Error::Failed => Ok(Errno::Failed),
            Error::NoDevice => Ok(Errno::NoDevice),
            Error::Unsupported => Ok(Errno::Unsupported),
            _ => todo!()
        }
    }
}

impl <D: Driver + 'static> WasmtimeRuntime<D> {
    pub fn new(driver: D, bin: &[u8]) -> anyhow::Result<Self> {
        // Create new linker with the provided engine
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        // Setup store and context
        let context = Context::new(driver);
        let mut store = Store::new(&engine, context);

        // Bind WASI
        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut Context<D>| &mut ctx.wasi )?;

        // Bind drivers
        spec::api::gpio::add_to_linker(&mut linker, move |c: &mut Context<D>| c)?;

        // Load module from file
        let module = Module::from_binary(&engine, bin)?;
        linker.module(&mut store, "", &module)?;

        Ok(Self{ engine, linker, store, module })
    }

    pub fn bind_all(&mut self, driver: impl Driver) {
        todo!()
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let Self{linker, ..} = self;

        linker
            .get_default(&mut self.store, "")?
            .typed::<(), (), _>(&self.store)?
            .call(&mut self.store, ())?;

        Ok(())
    }
}
