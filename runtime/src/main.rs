use std::env;
use std::time::SystemTime;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{Map, to_string, json};

use wasmer::{Store, Module, Instance, NativeFunc};
use wasmer_wasi::WasiState;

use wasmer_compiler_singlepass::Singlepass;
use wasmer_compiler_cranelift::Cranelift;
// use wasmer_compiler_llvm::LLVM;

use wasmer_engine_universal::Universal;


/// Type aliases for brevity
type Err = Box<dyn std::error::Error>;
type Mod = NativeFunc<i64, i64>;


/// Instantiate Wasmer Compiler and Engine
fn make_store(compiler_name: &String, _engine: &String) -> Store {

    let engine = match compiler_name.as_str() {
        "singlepass" => { Universal::new(Singlepass::default()) },
        "cranelift" => { Universal::new(Cranelift::default()) },
        // "llvm" => { Universal::new(LLVM::default()) },
        _ => { Universal::new(Cranelift::default()) }
    };

    Store::new(&engine.engine())
}


/// Create native callable from WASM function
fn make_callable(store: &Store, target: &str) -> Result<Mod, Err> {

    let module = Module::from_file(&store, target)?;
    let mut wasi_env = WasiState::new("benchmark-runtime").finalize()?;
    let import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;
    let benchmark = instance.exports.get_function("benchmark")?;
    let benchmark_native = benchmark.native::<i64, i64>()?;

    Ok(benchmark_native)
}


/// Storage for benchmark context and results
#[derive(Serialize, Deserialize)]
pub struct BenchmarkResult {
    compile: i64,
    runtime: Vec<i64>
}

pub struct Benchmark {
    name: String,
    callable: Mod,
    repeat: i64,
    iters: i64,
    result: BenchmarkResult,
}

impl Benchmark {
    pub fn from_descriptor(store: &Store, arg: &String) -> Result<Benchmark, Err> {
        let split = arg.split(":").collect::<Vec<&str>>();
        let target = split[0];

        let time = SystemTime::now();
        let callable = make_callable(store, target)?;
        let compile = time.elapsed()?.as_nanos();

        Ok(Benchmark {
            name: String::from(arg),
            callable: callable,
            repeat: split[1].parse()?,
            iters: split[2].parse()?,
            result: BenchmarkResult {
                compile: compile as i64,
                runtime: Vec::new()
            }
        })
    }
    pub fn execute(&mut self) -> Result<(), Err> {
        self.result.runtime.push(self.callable.call(self.iters)?);
        Ok(())
    }
}


/// Storage for benchmarks
pub struct Benchmarks {
    contents: Vec<Benchmark>,
    ordering: Vec<usize>
}

impl Benchmarks {
    pub fn new(contents: Vec<Benchmark>) -> Benchmarks {
        let mut ordering = Vec::new();
        for (i, b) in &mut contents.iter().enumerate() {
            for _ in 0..b.repeat { ordering.push(i); }
        }

        let mut rng = rand::thread_rng();
        ordering.shuffle(&mut rng);
    
        Benchmarks {
            contents: contents,
            ordering: ordering
        }
    }
    pub fn from_args() -> Result<Benchmarks, Err> {
        let args: Vec<String> = env::args().collect();
        let store = make_store(&args[1], &args[2]);

        let mut contents = Vec::new();
        for arg in env::args().skip(3) {
            contents.push(Benchmark::from_descriptor(&store, &arg)?);
        }
        Ok(Benchmarks::new(contents))
    }
    pub fn execute(&mut self) -> Result<(), Err> {
        for i in &self.ordering {
            self.contents[*i].execute()?;
        }
        Ok(())
    }
    pub fn as_json(&mut self) -> Result<(), Err> {
        let mut data = Map::new();
        for b in &self.contents {
            data.insert(String::from(&b.name), json!(b.result));
        }
        println!("{}", to_string(&data)?);
        Ok(())
    }
}


/// tmp
fn main() -> Result<(), Err> {

    let mut benchmarks = Benchmarks::from_args()?;
    benchmarks.execute()?;

    benchmarks.as_json()?;
    Ok(())

}
