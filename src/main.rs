pub mod cpu;

fn main() {
    let core_info: cpu::CpuInfo = cpu::read_cpuinfo();
    for core in core_info.cores {
        println!("{}: {}", core.processor_number, core.ghz);
    }
}
