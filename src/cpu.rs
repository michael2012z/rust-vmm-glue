use crate::error::*;
use std::fs::File;
use vm_memory::GuestAddress;

pub struct VmCpu {}

/// Wrapper over KVM vCPU ioctls.
pub struct VcpuFd {
    vcpu: File,
}

pub struct Vcpu {
    fd: VcpuFd,
    id: u8,
}

impl VmCpu {
    pub fn new() -> Result<Self> {
        Ok(VmCpu {})
    }

    fn create_vcpus(&self, vcpu_count: u64, entry_addr: GuestAddress) -> Result<Vec<Vcpu>> {
        let mut vcpus = Vec::with_capacity(vcpu_count as usize);

        Ok(vcpus)
    }

    fn start_vcpus(&mut self, mut vcpus: Vec<Vcpu>) -> Result<()> {
        //Vcpu::register_kick_signal_handler();
        Ok(())
    }
}
