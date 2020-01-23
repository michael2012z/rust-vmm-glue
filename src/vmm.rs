use crate::config::VmConfig;
use crate::error::*;
use crate::vm::Vm;
use kvm_ioctls::Kvm;

/*
pub enum Error {
    VmRun(kvm_ioctls::Error),
    VmPause(kvm_ioctls::Error),
    VmResume(kvm_ioctls::Error),
    VmStop(kvm_ioctls::Error),
}
 */

pub struct Vmm {
    kvm: Kvm,
}

impl Vmm {
    pub fn new() -> Result<Self> {
        let kvm = Kvm::new().unwrap();
        Ok(Vmm { kvm })
    }

    pub fn run_vm(&self, vm_config: VmConfig) -> Result<()> {
        println!(
            "run_vm: cpus: {}, mem: {} MB",
            vm_config.boot_vcpus, vm_config.memory_size
        );

        let mut vm = Vm::new(&self.kvm, vm_config)?;

        vm.boot()
    }

    pub fn pause_vm(&self, name: &str) {
        println!("VM {} paused", name);
    }

    pub fn resume_vm(&self, name: &str) {
        println!("VM {} resumed", name);
    }

    pub fn stop_vm(&self, name: &str) {
        println!("VM {} stopped", name);
    }
}
