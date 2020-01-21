use crate::config::VmConfig;
use crate::cpu::VmCpu;
use crate::error::*;
use crate::memory::VmMemory;
use kvm_ioctls::Kvm;
use kvm_ioctls::VmFd;

pub struct Vm {
    fd: VmFd,
    memory: VmMemory,
    cpus: VmCpu,
}

impl Vm {
    pub fn new(kvm: &Kvm, vm_config: VmConfig) -> Result<Self> {
        // Create VM.
        let fd = kvm.create_vm().unwrap();

        // Setup memory.
        let mut memory = VmMemory::new()?;
        memory.vm_memory_init(vm_config.memory_size as usize);

        let cpus = VmCpu::new()?;

        Ok(Vm { fd, memory, cpus })

        /*
        // Setup CPUs
        let entry_addr = self.load_kernel()?;
        let vcpus = self.create_vcpus(entry_addr)?;

        self.setup_irqchip()?;

        // Setup devices.

        // Start.
        self.start_vcpus(vcpus)
         */
    }

    pub fn start(&self) -> Result<()> {
        Ok(())
    }
}
