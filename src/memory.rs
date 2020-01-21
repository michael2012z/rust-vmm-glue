//      ==== Address map in use in ARM development systems today ====
//
//              - 32-bit -              - 36-bit -          - 40-bit -
//1024GB    +                   +                      +-------------------+     <- 40-bit
//          |                                           | DRAM              |
//          ~                   ~                       ~                   ~
//          |                                           |                   |
//          |                                           |                   |
//          |                                           |                   |
//          |                                           |                   |
//544GB     +                   +                       +-------------------+
//          |                                           | Hole or DRAM      |
//          |                                           |                   |
//512GB     +                   +                       +-------------------+
//          |                                           |       Mapped      |
//          |                                           |       I/O         |
//          ~                   ~                       ~                   ~
//          |                                           |                   |
//256GB     +                   +                       +-------------------+
//          |                                           |       Reserved    |
//          ~                   ~                       ~                   ~
//          |                                           |                   |
//64GB      +                   +-----------------------+-------------------+   <- 36-bit
//          |                   |                   DRAM                    |
//          ~                   ~                   ~                       ~
//          |                   |                                           |
//          |                   |                                           |
//34GB      +                   +-----------------------+-------------------+
//          |                   |                  Hole or DRAM             |
//32GB      +                   +-----------------------+-------------------+
//          |                   |                   Mapped I/O              |
//          ~                   ~                       ~                   ~
//          |                   |                                           |
//16GB      +                   +-----------------------+-------------------+
//          |                   |                   Reserved                |
//          ~                   ~                       ~                   ~
//4GB       +-------------------+-----------------------+-------------------+   <- 32-bit
//          |           2GB of DRAM                                         |
//          |                                                               |
//2GB       +-------------------+-----------------------+-------------------+
//          |                           Mapped I/O                          |
//1GB       +-------------------+-----------------------+-------------------+
//          |                          ROM & RAM & I/O                      |
//0GB       +-------------------+-----------------------+-------------------+   0
//              - 32-bit -              - 36-bit -              - 40-bit -
//
// Taken from (http://infocenter.arm.com/help/topic/com.arm.doc.den0001c/DEN0001C_principles_of_arm_memory_maps.pdf).

use crate::error::*;
use std::sync::Arc;
use vm_memory::{GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

pub struct VmLayout {}

impl VmLayout {
    /// Start of RAM on 64 bit ARM.
    pub const DRAM_MEM_START: u64 = 0x8000_0000; // 2 GB.
    /// The maximum addressable RAM address.
    pub const DRAM_MEM_END: u64 = 0x00FF_8000_0000; // 1024 - 2 = 1022 GB.
    /// The maximum RAM size.
    pub const DRAM_MEM_MAX_SIZE: u64 = VmLayout::DRAM_MEM_END - VmLayout::DRAM_MEM_START;

    /// Kernel command line maximum size.
    /// As per `arch/arm64/include/uapi/asm/setup.h`.
    pub const CMDLINE_MAX_SIZE: usize = 2048;

    /// Maximum size of the device tree blob as specified in https://www.kernel.org/doc/Documentation/arm64/booting.txt.
    pub const FDT_MAX_SIZE: usize = 0x20_0000;

    // As per virt/kvm/arm/vgic/vgic-kvm-device.c we need
    // the number of interrupts our GIC will support to be:
    // * bigger than 32
    // * less than 1023 and
    // * a multiple of 32.
    // We are setting up our interrupt controller to support a maximum of 128 interrupts.
    /// First usable interrupt on aarch64.
    pub const IRQ_BASE: u32 = 32;

    /// Last usable interrupt on aarch64.
    pub const IRQ_MAX: u32 = 159;

    /// Below this address will reside the GIC, above this address will reside the MMIO devices.
    pub const MAPPED_IO_START: u64 = (1 << 30); // 1 GB

    // Auxiliary function to get the address where the device tree blob is loaded.
    pub fn get_fdt_addr(mem: &GuestMemoryMmap) -> u64 {
        // If the memory allocated is smaller than the size allocated for the FDT,
        // we return the start of the DRAM so that
        // we allow the code to try and load the FDT.
        /*
                if let Some(offset) = mem.end_addr().checked_sub(layout::FDT_MAX_SIZE as u64) {
                    if mem.address_in_range(offset) {
                        return offset.raw_value();
                    }
                }
        */
        VmLayout::DRAM_MEM_START
    }
}

#[derive(PartialEq)]
pub enum RegionType {
    /// RAM type
    Ram,
}

pub struct VmMemory {
    pub guest_mem: GuestMemoryMmap,
}

impl VmMemory {
    /// Start of RAM on 64 bit ARM.
    pub const DRAM_MEM_START: u64 = 0x8000_0000; // 2 GB.
    /// The maximum addressable RAM address.
    pub const DRAM_MEM_END: u64 = 0x00FF_8000_0000; // 1024 - 2 = 1022 GB.
    /// The maximum RAM size.
    pub const DRAM_MEM_MAX_SIZE: u64 = VmMemory::DRAM_MEM_END - VmMemory::DRAM_MEM_START;

    pub fn new(mem_size_mib: usize) -> Result<Self> {
        let mem_size_bytes = mem_size_mib << 20;

        let arch_mem_regions = VmMemory::arch_memory_regions(mem_size_bytes);

        let ram_regions: Vec<(GuestAddress, usize)> = arch_mem_regions
            .iter()
            .filter(|r| r.2 == RegionType::Ram)
            .map(|r| (r.0, r.1))
            .collect();

        let mut mem_regions = Vec::new();
        for region in ram_regions.iter() {
            let mem_region = Arc::new(
                GuestRegionMmap::new(MmapRegion::new(region.1).unwrap(), region.0).unwrap(),
            );
            mem_regions.push(mem_region);
        }

        let guest_mem = GuestMemoryMmap::from_arc_regions(mem_regions.clone()).unwrap();

        Ok(VmMemory { guest_mem })
    }

    pub fn arch_memory_regions(size: usize) -> Vec<(GuestAddress, usize, RegionType)> {
        vec![(GuestAddress(0), size, RegionType::Ram)]
    }
}
