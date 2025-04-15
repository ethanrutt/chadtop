use std::fs;

pub struct CpuCore {
    pub processor_number: i8,
    pub ghz: f64,
    pub usage: i8,
}

pub struct CpuInfo {
    pub name: String,
    pub cores: Vec<CpuCore>,
}


pub fn read_cpuinfo() -> CpuInfo {
    let contents = fs::read_to_string("/proc/cpuinfo")
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut core_info = CpuInfo {
        name: String::from("not yet parsed"),
        cores: Vec::<CpuCore>::new(),
    };

    let mut curr_core = CpuCore {
        processor_number: -1,
        ghz: -1.0,
        usage: -1,
    };

    for line in lines {
        let cols = line.split(":").collect::<Vec<_>>();

        if cols[0].trim() == "model name" {
            core_info.name = cols[1].trim().parse().expect("mode name not a string");
        }
        else if cols[0].trim() == "processor" {
            curr_core.processor_number = cols[1].trim().parse().expect("processor not a number");
        }
        else if cols[0].trim() == "cpu MHz" {
            curr_core.ghz = cols[1].trim().parse().expect("cpu MHz not a number");
            curr_core.ghz /= 1000.0;
        }
        else if line.is_empty() {
            if curr_core.processor_number != -1 {
                core_info.cores.push(curr_core);
            }
            curr_core = CpuCore {
                processor_number: -1,
                ghz: -1.0,
                usage: -1,
            };
        }
    }

    core_info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_cpu() {
        let core_info = read_cpuinfo();

        assert_ne!(core_info.name, "not yet parsed");

        for core in core_info.cores {
            assert_ne!(core.processor_number, -1);
            assert_ne!(core.ghz, -1.0);
        }

        println!("NOTE: prob need to do some calculations for usage?");
    }
}
