use crate::server::Example;
use anyhow::{ensure, Ok};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::preview2::WasiCtx;
use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime_wasi::preview2::WasiView;

mod server {
    wasmtime::component::bindgen!({
        path: "wit/server/world.wit",
    });
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
}

impl WasiView for SimplePluginCtx {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

#[test]
fn main() -> Result<(), anyhow::Error> {
    let mut engine_config = wasmtime::Config::new();
    engine_config.wasm_component_model(true);
    engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    engine_config.debug_info(true);

    let engine = wasmtime::Engine::new(&engine_config).unwrap();

    let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);

    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker).unwrap();

    let table = ResourceTable::new();

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdin()
        .inherit_stdout()
        .build();

    let mut store = Store::new(
        &engine,
        SimplePluginCtx {
            table,
            context: wasi_ctx,
        },
    );

    let component =
        Component::from_file(&engine, "target/wasm32-wasi/debug/wasmtime_bug_report.wasm")?;

    let (plugin, _instance) = Example::instantiate(&mut store, &component, &linker)?;

    let res1_1 = plugin
        .component_bug_report_resource1_interface()
        .call_create(&mut store)?;

    let res1_2 = plugin
        .component_bug_report_resource1_interface()
        .call_create(&mut store)?;

    ensure!(res1_1 != res1_2);

    let res2_1 = plugin
        .component_bug_report_resource2_interface()
        .call_create_resource2(&mut store, res1_1)?;

    let res2_2 = plugin
        .component_bug_report_resource2_interface()
        .call_create_resource2(&mut store, res1_1)?;

    ensure!(res2_1 != res2_2);

    plugin
        .component_bug_report_resource2_interface()
        .call_do_stuff(&mut store, res1_1, res2_1)?;

    Ok(())
}
