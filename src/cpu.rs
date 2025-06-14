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
