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

        let vm = Vm::new(&self.kvm, vm_config)?;

        vm.start()
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

    /*
    fn load_kernel(&mut self) -> std::result::Result<GuestAddress, StartMicrovmError> {
        use StartMicrovmError::*;

        // Trying to load kernel before initialzing guest memory is a programming error.
        let vm_memory = self
            .vm
            .memory()
            .expect("Cannot load kernel prior allocating memory!");

        // This is the easy way out of consuming the value of the kernel_cmdline.
        let kernel_config = self.kernel_config.as_mut().ok_or(MissingKernelConfig)?;

        let entry_addr = kernel_loader::load_kernel(
            vm_memory,
            &mut kernel_config.kernel_file,
            arch::get_kernel_start(),
        )
        .map_err(KernelLoader)?;

        Ok(entry_addr)
    }
     */
}
