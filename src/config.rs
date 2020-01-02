pub struct VmConfig {
    pub boot_vcpus: u8,
    pub max_vcpus: u8,
    pub memory_size: u64,
    /*
        pub kernel_path: PathBuf,
        pub kernel_args: String,
        pub disk_path: PathBuf,
    */
}

impl VmConfig {
    pub fn new(cpus: u8, mem: u64) -> Self {
        VmConfig {
            boot_vcpus: cpus,
            max_vcpus: cpus,
            memory_size: mem,
        }
    }
}
