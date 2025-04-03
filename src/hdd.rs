use std::process::Command;

pub struct HddInfo {
    pub name: String,
    pub mount: String,
    pub used: String,
    pub avail: String,
}

pub fn read_hddinfo() -> HddInfo {
    let process = Command::new("df")
        .arg("-h")
        .output()
        .ok()
        .expect("Failed to execute");

    let contents = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");

    let lines = contents.split("\n");

    let mut hdd_info: HddInfo = HddInfo {
        name: String::from("not parsed yet"),
        mount: String::from("not parsed yet"),
        used: String::from("not parsed yet"),
        avail: String::from("not parsed yet"),
    };

    for line in lines {
        let cols = line.split_whitespace().collect::<Vec<_>>();
        if cols.len() >= 6 && cols[5] == "/" {
            hdd_info.name = cols[0].trim().parse().expect("filesystem not a string");
            hdd_info.mount = cols[5].trim().parse().expect("filesystem not a string");
            hdd_info.used = cols[2].trim().parse().expect("filesystem not a string");
            hdd_info.avail = cols[3].trim().parse().expect("filesystem not a string");
        }
    }

    return hdd_info
}
