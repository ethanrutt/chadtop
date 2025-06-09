use std::collections::HashMap;
use std::fs;

pub struct Proc {
    pub name: String,
    /// real uid, i.e. the first number in the 4 numbers
    pub uid: i32,
    /// real gid, i.e. the first number in the 4 numbers
    pub gid: i32,
    pub pid: i32,
    pub ppid: i32,
    pub threads: i32,
    /// virtual memsize in megabytes
    pub mem: i32,
    pub cpu_usage: i32,
}

pub fn read_procs() -> Vec<Proc> {
    let pids = get_all_pids();

    let mut ret: Vec<Proc> = Vec::new();
    for pid in pids {
        ret.push(parse_proc(pid).unwrap());
    }

    ret
}

fn get_all_pids() -> Vec<i32> {
    let mut pids = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(pid) = file_name.parse::<i32>() {
                    pids.push(pid);
                }
            }
        }
    }

    pids
}

/// parse from /proc/pid/status
fn parse_proc(pid: i32) -> Option<Proc> {
    let path = format!("/proc/{pid}/status");
    let content = fs::read_to_string(path).ok()?;
    let mut proc_map: HashMap<String, String> = HashMap::new();
    for line in content.lines() {
        let mut bruh: Vec<&str> = line.split(":").collect();
        bruh[1] = bruh[1].trim();
        proc_map.insert(String::from(bruh[0]), String::from(bruh[1]));
    }

    let path = format!("/proc/{pid}/stat");
    let content = fs::read_to_string(path).ok()?;

    for (i, s) in content.split_whitespace().enumerate() {
        // man proc_pid_stat
        // starts at 1, 14 is the utime, 15 is stime, 22 is starttime, but we are
        // enumerating so we start at 0
        if i == 13 {
            proc_map.insert(String::from("utime"), String::from(s));
        } else if i == 14 {
            proc_map.insert(String::from("stime"), String::from(s));
        } else if i == 21 {
            proc_map.insert(String::from("starttime"), String::from(s));
        }
    }

    let path = format!("/proc/uptime");
    let content = fs::read_to_string(path).ok()?;
    for (i, s) in content.split_whitespace().enumerate() {
        // man proc_uptime
        // just want the first number
        if i == 0 {
            proc_map.insert(String::from("uptime"), String::from(s));
        }
    }

    let utime: i32 = proc_map.get("utime").unwrap().parse().unwrap();
    let stime: i32 = proc_map.get("stime").unwrap().parse().unwrap();
    let starttime: i32 = proc_map.get("starttime").unwrap().parse().unwrap();
    let uptime: f32 = proc_map
        .get("uptime")
        .unwrap()
        .parse::<f32>()
        .unwrap()
        .round();
    let uptime: i32 = uptime as i32;

    // 100 for approximation
    let cpu_usage = calculate_cpu_usage(utime, stime, starttime, uptime, 100);

    Some(Proc {
        name: proc_map
            .get("Name")
            .expect("can't find name in proc_map")
            .to_string(),
        pid: pid,
        ppid: proc_map
            .get("PPid")
            .expect("can't find ppid in proc_map")
            .parse()
            .unwrap(),
        threads: proc_map
            .get("Threads")
            .expect("can't find threads in proc_map")
            .parse()
            .unwrap(),
        uid: proc_map
            .get("Uid")
            .expect("can't find uid in proc_map")
            .to_string()
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        gid: proc_map
            .get("Gid")
            .expect("can't find gid in proc_map")
            .to_string()
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        mem: proc_map
            .get("VmSize")
            .expect("can't find VmSize in proc_map")
            .to_string()
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
            / 1000,
        cpu_usage: cpu_usage,
    })
}

fn calculate_cpu_usage(utime: i32, stime: i32, starttime: i32, uptime: i32, hertz: i32) -> i32 {
    let total_time = utime + stime;
    let seconds = uptime - (starttime / hertz);

    if seconds <= 0 {
        return 0;
    }

    let cpu_usage = 100 * (total_time / hertz) / seconds;
    cpu_usage
}

