use crate::error::*;
use vm_memory::{GuestMemoryMmap, GuestRegionMmap};

pub struct VmMemory {
    guest_mem: Option<GuestMemoryMmap>,
}

impl VmMemory {
    /// Start of RAM on 64 bit ARM.
    pub const DRAM_MEM_START: u64 = 0x8000_0000; // 2 GB.
    /// The maximum addressable RAM address.
    pub const DRAM_MEM_END: u64 = 0x00FF_8000_0000; // 1024 - 2 = 1022 GB.
    /// The maximum RAM size.
    pub const DRAM_MEM_MAX_SIZE: u64 = VmMemory::DRAM_MEM_END - VmMemory::DRAM_MEM_START;

    pub fn new() -> Result<Self> {
        Ok(VmMemory { guest_mem: None })
    }

    pub fn memory_regions(&self, size: usize) -> Vec<GuestRegionMmap> {
        vec![]
    }

    pub fn memory_regions_init(&self, guest_mem: &GuestMemoryMmap) -> Result<()> {
        Ok(())
    }

    pub fn vm_memory_init(&mut self, mem_size_mib: usize) -> Result<()> {
        let mem_size_bytes = mem_size_mib << 20;
        let mem_regions = self.memory_regions(mem_size_bytes);
        self.guest_mem = Some(GuestMemoryMmap::from_regions(mem_regions).unwrap());
        if let Some(guest_mem) = &self.guest_mem {
            self.memory_regions_init(guest_mem)
        } else {
            Err(Error::Generic)
        }
    }
}
