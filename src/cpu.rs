use sysinfo::System;

pub struct CpuUsage {
    pub name: String,
    pub usage: f32,
}

/// before calling this function make sure to refresh sys with the cpu values
pub fn read_cpus(sys: &mut System) -> Vec<CpuUsage> {
    let mut ret: Vec<CpuUsage> = Vec::new();

    ret.push(CpuUsage {
        name: String::from("overall"),
        usage: sys.global_cpu_usage(),
    });

    for cpu in sys.cpus() {
        ret.push(CpuUsage {
            name: String::from(cpu.name()),
            usage: cpu.cpu_usage(),
        });
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_read_cpus_returns_at_least_one_entry() {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();

        let cpus = read_cpus(&mut sys);

        assert!(!cpus.is_empty(), "Expected at least one CPU usage entry");
    }

    #[test]
    fn test_first_cpu_is_overall() {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();

        let cpus = read_cpus(&mut sys);

        assert_eq!(
            cpus[0].name, "overall",
            "First CPU usage entry should be 'overall'"
        );
    }

    #[test]
    fn test_each_cpu_has_valid_usage() {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();

        let cpus = read_cpus(&mut sys);

        for cpu in cpus {
            assert!(
                (0.0..=100.0).contains(&cpu.usage),
                "CPU usage out of expected range: {} ({})",
                cpu.name,
                cpu.usage
            );
        }
    }

    #[test]
    fn test_logical_cpu_count_matches() {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();

        let cpus = read_cpus(&mut sys);

        // There should be exactly 1 (overall) + N (logical CPUs)
        let expected = 1 + sys.cpus().len();
        assert_eq!(
            cpus.len(),
            expected,
            "Expected {} CPU entries, got {}",
            expected,
            cpus.len()
        );
    }
}
