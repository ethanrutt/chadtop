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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_info_sanity() {
        let info = read_info();

        assert!(
            !info.kernel_long_version.is_empty(),
            "kernel_long_version should not be empty"
        );

        assert!(!info.cpu_arch.is_empty(), "cpu_arch should not be empty");

        // Optional fields should be Some(...) and non-empty when present
        if let Some(ref os) = info.long_os_version {
            assert!(
                !os.is_empty(),
                "long_os_version should not be an empty string"
            );
        }

        if let Some(ref host) = info.host_name {
            assert!(!host.is_empty(), "host_name should not be an empty string");
        }

        if let Some(cores) = info.physical_core_count {
            assert!(
                cores > 0,
                "physical_core_count should be greater than zero if present"
            );
        }
    }
}
