#![no_main]
use libfuzzer_sys::fuzz_target;
use steel::steel_vm::engine::Engine;

fuzz_target!(|data: String| {
    let mut vm = Engine::new();
    vm.compile_and_run_raw_program(&data).unwrap();
});