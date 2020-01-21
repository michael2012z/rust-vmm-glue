use crate::error::*;
use std::sync::Arc;
use vm_memory::{GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

#[derive(PartialEq)]
pub enum RegionType {
    /// RAM type
    Ram,

    /// SubRegion memory region.
    /// A SubRegion is a memory region sub-region, allowing for a region
    /// to be split into sub regions managed separately.
    /// For example, the x86 32-bit memory hole is a SubRegion.
    SubRegion,

    /// Reserved type.
    /// A Reserved memory region is one that should not be used for memory
    /// allocation. This type can be used to prevent the VMM from allocating
    /// memory ranges in a specific address range.
    Reserved,
}

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

    pub fn arch_memory_regions(&self, size: usize) -> Vec<(GuestAddress, usize, RegionType)> {
        vec![(GuestAddress(0), size, RegionType::Ram)]
    }

    fn create_ram_region(
        &self,
        start_addr: GuestAddress,
        size: usize,
    ) -> Result<Arc<GuestRegionMmap>> {
        Ok(Arc::new(
            GuestRegionMmap::new(MmapRegion::new(size).unwrap(), start_addr).unwrap(),
        ))
    }

    pub fn vm_memory_init(&mut self, mem_size_mib: usize) -> Result<()> {
        let mem_size_bytes = mem_size_mib << 20;

        let arch_mem_regions = self.arch_memory_regions(mem_size_bytes);

        let ram_regions: Vec<(GuestAddress, usize)> = arch_mem_regions
            .iter()
            .filter(|r| r.2 == RegionType::Ram)
            .map(|r| (r.0, r.1))
            .collect();

        let mut mem_regions = Vec::new();
        for region in ram_regions.iter() {
            mem_regions.push(self.create_ram_region(region.0, region.1)?);
        }

        let guest_memory = GuestMemoryMmap::from_arc_regions(mem_regions.clone()).unwrap();
        self.guest_mem = Some(guest_memory);

        Ok(())
    }
}
