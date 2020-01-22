use std::path::PathBuf;

pub struct VmConfig {
    pub boot_vcpus: u8,
    pub max_vcpus: u8,
    pub memory_size: u64,
    pub kernel_path: PathBuf,
    //pub kernel_args: CString,
    pub disk_path: PathBuf,
}

impl VmConfig {
    pub fn new(
        cpus: u8,
        mem: u64,
        kernel_path: PathBuf,
        //kernel_args: CString,
        disk_path: PathBuf,
    ) -> Self {
        VmConfig {
            boot_vcpus: cpus,
            max_vcpus: cpus,
            memory_size: mem,
            kernel_path,
            //kernel_args,
            disk_path,
        }
    }
}
