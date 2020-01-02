#[macro_use]
extern crate clap;
use clap::App;
use std::*;

mod config;
mod vm;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    println!("config: {:?}", matches.value_of("config"));

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            println!("run");

            let cpus = run_matches.value_of("cpus").unwrap().parse::<u8>().unwrap();
            let mem = run_matches.value_of("mem").unwrap().parse::<u64>().unwrap();

            let vm_config = config::VmConfig::new(cpus, mem);

            vm::Vm::run_vm(vm_config);
        }
        ("pause", Some(pause_matches)) => {
            let name = pause_matches.value_of("name").unwrap();
            vm::Vm::pause_vm(name);
        }
        ("resume", Some(resume_matches)) => {
            let name = resume_matches.value_of("name").unwrap();
            vm::Vm::resume_vm(name);
        }
        ("stop", Some(stop_matches)) => {
            let name = stop_matches.value_of("name").unwrap();
            vm::Vm::stop_vm(name);
        }
        ("", None) => {}
        _ => {}
    }
}
