pub mod cpu;
pub mod gpu;
pub mod ram;
pub mod hdd;
pub mod proc;

fn main() {
    let core_info: cpu::CpuInfo = cpu::read_cpuinfo();
    for core in core_info.cores {
        println!("{}: {}", core.processor_number, core.ghz);
    }

    let gpu_info: gpu::GpuInfo = gpu::read_gpuinfo();

    println!("{}", gpu_info.name);
    println!("{}", gpu_info.mem_size);

    let ram_info: ram::RamInfo = ram::read_raminfo();

    println!("{}", ram_info.total);
    println!("{}", ram_info.free);
    println!("{}", ram_info.available);
    println!("{}", ram_info.cached);

    let hdd_info: hdd::HddInfo = hdd::read_hddinfo();

    println!("{}", hdd_info.name);
    println!("{}", hdd_info.mount);
    println!("{}", hdd_info.used);
    println!("{}", hdd_info.avail);

    let procs = proc::read_procs();
}
