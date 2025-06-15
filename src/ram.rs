use sysinfo::System;

pub struct Ram {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub used: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            total: 0,
            free: 0,
            available: 0,
            used: 0,
            total_swap: 0,
            free_swap: 0,
            used_swap: 0,
        }
    }
}

pub fn read_memory(sys: &mut System) -> Ram {
    let total = sys.total_memory();
    let free = sys.free_memory();
    let available = sys.available_memory();
    let used = sys.used_memory();
    let total_swap = sys.total_swap();
    let free_swap = sys.free_swap();
    let used_swap = sys.used_swap();
    Ram {
        total,
        free,
        available,
        used,
        total_swap,
        free_swap,
        used_swap,
    }
}
