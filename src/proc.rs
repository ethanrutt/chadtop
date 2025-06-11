use sysinfo::{DiskUsage, System};

pub struct Proc {
    pub name: Option<String>,
    pub cmd: Option<String>,
    pub exe: Option<String>,
    pub pid: u32,
    pub cwd: Option<String>,
    pub memory: u64,
    pub ppid: Option<u32>,
    pub start_time: u64,
    pub run_time: u64,
    pub cpu_usage: i32,
    pub disk_usage_read: u64,
    pub disk_usage_written: u64,
    pub uid: Option<u32>,
    pub open_files: Option<usize>,
    pub open_files_limit: Option<usize>,
}

pub fn read_procs(sys: &mut System) -> Vec<Proc> {
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let mut ret: Vec<Proc> = Vec::new();

    for (pid, proc) in sys.processes() {
        let name: Option<String> = proc.name().to_str().and_then(|s| Some(String::from(s)));
        let cmd: Option<String> = proc
            .cmd()
            .iter()
            .map(|s| s.to_str())
            .collect::<Option<Vec<&str>>>()
            .map(|v| v.join(" "));
        let exe: Option<String> = proc
            .exe()
            .and_then(|s| s.to_str())
            .and_then(|s| Some(String::from(s)));
        let pid: u32 = pid.as_u32();
        let cwd: Option<String> = proc
            .cwd()
            .and_then(|s| s.to_str())
            .and_then(|s| Some(String::from(s)));
        let memory: u64 = proc.memory();
        let ppid: Option<u32> = proc.parent().and_then(|n| Some(n.as_u32()));
        let start_time: u64 = proc.start_time();
        let run_time: u64 = proc.run_time();
        let cpu_usage: i32 = proc.cpu_usage() as i32;
        let disk_usage: DiskUsage = proc.disk_usage();
        let disk_usage_read: u64 = disk_usage.total_read_bytes;
        let disk_usage_written: u64 = disk_usage.total_written_bytes;
        let uid: Option<u32> = proc.user_id().and_then(|u| {
            Some(
                u.to_string()
                    .parse::<u32>()
                    .expect("unable to parse user id"),
            )
        });
        let open_files: Option<usize> = proc.open_files();
        let open_files_limit: Option<usize> = proc.open_files_limit();

        ret.push(Proc {
            name,
            cmd,
            exe,
            pid,
            cwd,
            memory,
            ppid,
            start_time,
            run_time,
            cpu_usage,
            disk_usage_read,
            disk_usage_written,
            uid,
            open_files,
            open_files_limit,
        })
    }

    ret
}
