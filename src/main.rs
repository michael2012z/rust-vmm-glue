#[macro_use]
extern crate clap;
use clap::App;
use std::path::PathBuf;
use std::*;

mod config;
mod cpu;
mod error;
mod memory;
mod regs;
mod vm;
mod vmm;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    println!("config: {:?}", matches.value_of("config"));

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            println!("run");

            let cpus = run_matches.value_of("cpus").unwrap().parse::<u8>().unwrap();
            let mem = run_matches.value_of("mem").unwrap().parse::<u64>().unwrap();
            let kernel_path = run_matches.value_of("kernel_path").unwrap();
            let kernel_path = PathBuf::from(kernel_path);
            //let kernel_args = run_matches.value_of("kernel_args").unwrap();
            let disk_path = run_matches.value_of("disk_path").unwrap();
            let disk_path = PathBuf::from(disk_path);

            let vm_config = config::VmConfig::new(cpus, mem, kernel_path, disk_path);

            vmm::Vmm::new().unwrap().run_vm(vm_config).unwrap();
        }
        ("pause", Some(pause_matches)) => {
            let name = pause_matches.value_of("name").unwrap();
            vmm::Vmm::new().unwrap().pause_vm(name);
        }
        ("resume", Some(resume_matches)) => {
            let name = resume_matches.value_of("name").unwrap();
            vmm::Vmm::new().unwrap().resume_vm(name);
        }
        ("stop", Some(stop_matches)) => {
            let name = stop_matches.value_of("name").unwrap();
            vmm::Vmm::new().unwrap().stop_vm(name);
        }
        ("", None) => {}
        _ => {}
    }
}
