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
        let vm_fd = kvm.create_vm().unwrap();

        // Setup memory.
        let vm_memory = VmMemory::new(vm_config.memory_size as usize)?;

        let vm_cpu = VmCpu::new()?;

        Ok(Vm {
            fd: vm_fd,
            memory: vm_memory,
            cpus: vm_cpu,
            config: vm_config,
        })
    }

    pub fn boot(&mut self) -> Result<()> {
        // Setup CPUs
        let entry_addr = self.load_kernel().unwrap();
        self.cpus
            .create_vcpus(
                &self.fd,
                self.config.boot_vcpus as u64,
                entry_addr,
                &self.memory.guest_mem,
            )
            .unwrap();
        self.cpus.start_vcpus().unwrap();

        /*
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
