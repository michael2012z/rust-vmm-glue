use crate::config::VmConfig;
use kvm_ioctls::Kvm;

/*
pub enum Error {
    VmRun(kvm_ioctls::Error),
    VmPause(kvm_ioctls::Error),
    VmResume(kvm_ioctls::Error),
    VmStop(kvm_ioctls::Error),
}
 */

pub struct Vm {}

impl Vm {
    pub fn run_vm(vm_config: VmConfig) {
        println!(
            "run_vm: cpus: {}, mem: {} MB",
            vm_config.boot_vcpus, vm_config.memory_size
        );
        let kvm = Kvm::new().unwrap();
        kvm.create_vm().unwrap();
    }

    pub fn pause_vm(name: &str) {
        println!("VM {} paused", name);
    }

    pub fn resume_vm(name: &str) {
        println!("VM {} resumed", name);
    }

    pub fn stop_vm(name: &str) {
        println!("VM {} stopped", name);
    }
}
