use crate::config::VmConfig;
use crate::cpu::VmCpu;
use crate::error::*;
use crate::memory::VmLayout;
use crate::memory::VmMemory;
use kvm_ioctls::Kvm;
use kvm_ioctls::VmFd;
use linux_loader::loader;
use linux_loader::loader::KernelLoader;
use std::fs::File;
use vm_memory::GuestAddress;

pub struct Vm {
    fd: VmFd,
    memory: VmMemory,
    cpus: VmCpu,
    config: VmConfig,
}

impl Vm {
    pub fn new(kvm: &Kvm, vm_config: VmConfig) -> Result<Self> {
        // Create VM.
        let fd = kvm.create_vm().unwrap();

        // Setup memory.
        let memory = VmMemory::new(vm_config.memory_size as usize)?;

        let cpus = VmCpu::new()?;

        Ok(Vm {
            fd,
            memory,
            cpus,
            config: vm_config,
        })
    }

    pub fn start(&self) -> Result<()> {
        // Setup CPUs
        let entry_addr = self.load_kernel().unwrap();

        /*
        let vcpus = self.create_vcpus(entry_addr)?;

        self.setup_irqchip()?;

        // Setup devices.

        // Start.
        self.start_vcpus(vcpus)
         */

        Ok(())
    }

    fn load_kernel(&self) -> Result<GuestAddress> {
        let mem = &self.memory.guest_mem;
        let mut kernel = File::open(&self.config.kernel_path).unwrap();
        let entry_addr = loader::Arm64Pe::load(
            mem,
            None,
            &mut kernel,
            Some(GuestAddress(VmLayout::get_kernel_start())),
        )
        .unwrap();
        let load_addr = entry_addr.kernel_load;

        Ok(load_addr)
    }
}
