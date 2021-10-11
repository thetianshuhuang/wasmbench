use std::env;
use std::time::SystemTime;

use wasmer::{Store, Module, Instance};
use wasmer_wasi::WasiState;

use wasmer_compiler_singlepass::Singlepass;
// use wasmer_compiler_cranelift::Cranelift;
// use wasmer_compiler_llvm::LLVM;

use wasmer_engine_universal::Universal;


fn make_store(_compiler: &String, _engine: &String) -> Store {

    let compiler = Singlepass::default();
    Store::new(&Universal::new(compiler).engine())

}


fn execute(store: &Store, target: &String) -> Result<(), Box<dyn std::error::Error>> {

    let time = SystemTime::now();

    let module = Module::from_file(&store, target)?;

    let mut wasi_env = WasiState::new("benchmark-runtime").finalize()?;
    
    let import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;

    let start = instance.exports.get_function("_start")?;
    match time.elapsed() {
        Ok(elapsed) => { println!("{}", elapsed.as_nanos()); }
        Err(_e) => { println!("Error: Timer"); }
    }
    start.call(&[])?;
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let store = make_store(&args[1], &args[2]);

    for arg in env::args().skip(3) {
        execute(&store, &arg)?;
    }
    Ok(())
}
