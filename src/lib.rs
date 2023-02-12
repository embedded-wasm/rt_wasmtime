//! Wasmtime embedded wasm runtime support

pub use wasm_embedded_spec::{self as spec};

use spec::{
    Engine,
    wiggle::api,
};

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};


//mod error;

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

impl <E: Engine + 'static> WasmtimeRuntime<E> {
    /// Create a new WasmtimeRuntime with the provided engine and application
    pub fn new(mut engine: E, bin: &[u8]) -> anyhow::Result<Self> {
        // Create new linker with the provided engine
        let wasm_engine = wasmtime::Engine::default();
        let mut linker = Linker::new(&wasm_engine);

        // Bind WASI
        // TODO: make WASI an optional feature?
        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut Context<E>| &mut ctx.wasi )?;

        // Bind drivers
        if engine.gpio().is_some() {
            api::gpio::add_to_linker(&mut linker, move |c: &mut Context<E>| c.engine.gpio().unwrap() )?;
        }
        if engine.spi().is_some() {
            api::spi::add_to_linker(&mut linker, move |c: &mut Context<E>| c.engine.spi().unwrap() )?;
        }
        if engine.i2c().is_some() {
            api::i2c::add_to_linker(&mut linker, move |c: &mut Context<E>| c.engine.i2c().unwrap() )?;
        }
        if engine.uart().is_some() {
            api::uart::add_to_linker(&mut linker, move |c: &mut Context<E>| c.engine.uart().unwrap() )?;
        }
        
        // Setup store and context
        let context = Context::new(engine);
        let mut store = Store::new(&wasm_engine, context);

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
            .typed::<(), _>(&self.store)?
            .call(&mut self.store, ())?;

        Ok(())
    }
}
