use sysinfo::System;

pub struct Info {
    pub long_os_version: Option<String>,
    pub kernel_long_version: String,
    pub host_name: Option<String>,
    pub cpu_arch: String,
    pub physical_core_count: Option<usize>,
}

/// This should only be called once as this information shouldn't change and frankly if you change
/// this while you have chadtop open you should just restart chadtop and probably your computer
pub fn read_info() -> Info {
    let long_os_version = System::long_os_version();
    let host_name = System::host_name();
    let kernel_long_version = System::kernel_long_version();
    let cpu_arch = System::cpu_arch();
    let physical_core_count = System::physical_core_count();

    Info {
        long_os_version,
        kernel_long_version,
        host_name,
        cpu_arch,
        physical_core_count,
    }
}
