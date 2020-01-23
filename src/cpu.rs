use crate::error::*;
use crate::regs;
use kvm_bindings;
use kvm_ioctls::{VcpuFd, VmFd};
use std::sync::{Arc, Barrier};
use std::thread;
use vm_memory::{GuestAddress, GuestMemoryMmap};

pub struct Vcpu {
    fd: VcpuFd,
    id: u8,
    mpidr: u64,
}

impl Vcpu {
    pub fn new(id: u8, vm_fd: &VmFd) -> Result<Self> {
        let kvm_vcpu = vm_fd.create_vcpu(id).unwrap(); //.map_err(Error::X)?;
        Ok(Vcpu {
            fd: kvm_vcpu,
            id,
            mpidr: 0,
        })
    }

    pub fn configure(
        &mut self,
        vm_fd: &VmFd,
        guest_mem: &GuestMemoryMmap,
        kernel_load_addr: GuestAddress,
    ) -> Result<()> {
        let mut kvi: kvm_bindings::kvm_vcpu_init = kvm_bindings::kvm_vcpu_init::default();

        // This reads back the kernel's preferred target type.
        vm_fd.get_preferred_target(&mut kvi).unwrap();
        // We already checked that the capability is supported.
        kvi.features[0] |= 1 << kvm_bindings::KVM_ARM_VCPU_PSCI_0_2;
        // Non-boot cpus are powered off initially.
        if self.id > 0 {
            kvi.features[0] |= 1 << kvm_bindings::KVM_ARM_VCPU_POWER_OFF;
        }

        self.fd.vcpu_init(&kvi).unwrap();
        regs::setup_regs(&self.fd, self.id, kernel_load_addr.0, guest_mem).unwrap();

        self.mpidr = regs::read_mpidr(&self.fd).unwrap();

        Ok(())
    }

    /*
        pub fn run(&self) -> Result<()> {
            Ok(())
        }
    */

    pub fn fake_run() -> Result<()> {
        Ok(())
    }
}

pub struct VmCpu {
    cpus: Option<Vec<Vcpu>>,
    cpu_count: usize,
}

impl VmCpu {
    pub fn new() -> Result<Self> {
        Ok(VmCpu {
            cpus: None,
            cpu_count: 0,
        })
    }

    pub fn create_vcpus(
        &mut self,
        vm_fd: &VmFd,
        vcpu_count: u64,
        entry_addr: GuestAddress,
        guest_mem: &GuestMemoryMmap,
    ) -> Result<()> {
        let mut vcpus = Vec::with_capacity(vcpu_count as usize);

        for cpu_index in 0..vcpu_count {
            let mut vcpu;
            vcpu = Vcpu::new(cpu_index as u8, vm_fd)?;

            vcpu.configure(vm_fd, guest_mem, entry_addr)?;

            vcpus.push(vcpu);
        }

        self.cpus = Some(vcpus);
        self.cpu_count = vcpu_count as usize;

        Ok(())
    }

    pub fn start_vcpus(&mut self) -> Result<()> {
        // TODO
        // Totally not ready.
        let vcpu_thread_barrier = Arc::new(Barrier::new(self.cpu_count));
        for cpu in self.cpus.as_ref().unwrap() {
            let vcpu_thread_barrier = vcpu_thread_barrier.clone();
            let _handle = Some(
                thread::Builder::new()
                    .name(format!("vcpu{}", cpu.id))
                    .spawn(move || {
                        // Block until all CPUs are ready.
                        vcpu_thread_barrier.wait();
                        loop {
                            match Vcpu::fake_run() {
                                Err(_e) => {
                                    break;
                                }
                                Ok(()) => {}
                            }
                        }
                    }),
            );
        }
        vcpu_thread_barrier.wait();
        Ok(())
    }
}
