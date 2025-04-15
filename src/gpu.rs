use std::process::Command;

pub struct GpuInfo {
    pub name: String,
    pub mem_size: String,
}

pub fn read_gpuinfo() -> GpuInfo {
    let process = Command::new("glxinfo")
        .output()
        .ok()
        .expect("Failed to execute");
    let contents = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");

    let lines = contents.split("\n");

    let mut gpu_info: GpuInfo = GpuInfo {
        name: String::from("not parsed yet"),
        mem_size: String::from("-1"),
    };

    for line in lines {
        let cols = line.split(":").collect::<Vec<_>>();

        if cols[0].trim() == "Device" {
            gpu_info.name = cols[1].trim().parse().expect("device name not a string");
        }
        else if cols[0].trim() == "Video memory" {
            gpu_info.mem_size = cols[1].trim().parse().expect("video memory not a string");
        }
    }

    gpu_info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_gpu() {
        let gpu_info = read_gpuinfo();

        assert_ne!(gpu_info.name, "not parsed yet");
        assert_ne!(gpu_info.mem_size, "-1");
    }
}
