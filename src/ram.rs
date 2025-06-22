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

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_read_memory_returns_nonzero_total() {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let ram = read_memory(&mut sys);

        assert!(
            ram.total > 0,
            "Expected total memory to be greater than zero"
        );
    }

    #[test]
    fn test_memory_values_make_sense() {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let ram = read_memory(&mut sys);

        assert!(
            ram.free <= ram.total,
            "Free memory cannot exceed total memory"
        );
        assert!(
            ram.used <= ram.total,
            "Used memory cannot exceed total memory"
        );
        assert!(
            ram.available <= ram.total,
            "Available memory cannot exceed total memory"
        );
    }

    #[test]
    fn test_swap_values_make_sense() {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let ram = read_memory(&mut sys);

        assert!(
            ram.free_swap <= ram.total_swap,
            "Free swap cannot exceed total swap"
        );
        assert!(
            ram.used_swap <= ram.total_swap,
            "Used swap cannot exceed total swap"
        );
        assert_eq!(
            ram.total_swap,
            ram.free_swap + ram.used_swap,
            "Swap total should equal used + free"
        );
    }

    #[test]
    fn test_ram_new_is_zeroed() {
        let ram = Ram::new();

        assert_eq!(ram.total, 0);
        assert_eq!(ram.free, 0);
        assert_eq!(ram.available, 0);
        assert_eq!(ram.used, 0);
        assert_eq!(ram.total_swap, 0);
        assert_eq!(ram.free_swap, 0);
        assert_eq!(ram.used_swap, 0);
    }
}
