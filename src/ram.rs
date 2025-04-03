use std::fs;

pub struct RamInfo {
    pub total: f64,
    pub free: f64,
    pub available: f64,
    pub cached: f64
}

fn trim_units(s: &str) -> String {
    let ret = String::from(s);
    let ret = ret.trim();

    String::from(&ret[0..ret.find(" ").expect("no space in unit string")])
}

pub fn read_raminfo() -> RamInfo {
    let contents = fs::read_to_string("/proc/meminfo")
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut ram_info = RamInfo {
        total: -1.0,
        free: -1.0,
        available: -1.0,
        cached: -1.0,
    };

    for line in lines {
        let cols = line.split(":").collect::<Vec<_>>();

        if cols[0].trim() == "MemTotal" {
            ram_info.total = trim_units(cols[1]).parse().expect("Can't parse into MemTotal");
            ram_info.total /= 1000000.0
        }
        else if cols[0].trim() == "MemFree" {
            ram_info.free = trim_units(cols[1]).parse().expect("Can't parse into MemFree");
            ram_info.free /= 1000000.0
        }
        else if cols[0].trim() == "MemAvailable" {
            ram_info.available = trim_units(cols[1]).parse().expect("Can't parse into MemAvailable");
            ram_info.available /= 1000000.0
        }
        else if cols[0].trim() == "Cached" {
            ram_info.cached = trim_units(cols[1]).parse().expect("Can't parse into Cached");
            ram_info.cached /= 1000000.0
        }
    }

    ram_info
}
