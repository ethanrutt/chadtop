use sysinfo::{DiskUsage, System, Users};

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
    pub cpu_usage: f32,
    pub disk_usage_read: u64,
    pub disk_usage_written: u64,
    pub user: Option<String>,
    pub open_files: Option<usize>,
    pub open_files_limit: Option<usize>,
}

/// read procs
/// before calling this make sure to refresh the System argument and make sure the Users argument
/// is populated
pub fn read_procs(sys: &mut System, users: &mut Users) -> Vec<Proc> {
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
        let cpu_usage: f32 = proc.cpu_usage();
        let disk_usage: DiskUsage = proc.disk_usage();
        let disk_usage_read: u64 = disk_usage.total_read_bytes;
        let disk_usage_written: u64 = disk_usage.total_written_bytes;

        users.refresh();
        let user = proc
            .user_id()
            .and_then(|u| users.get_user_by_id(u))
            .and_then(|u| Some(String::from(u.name())));

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
            user,
            open_files,
            open_files_limit,
        })
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::{System, Users};

    #[test]
    fn test_read_procs_returns_some_procs() {
        let mut sys = System::new_all();
        let mut users = Users::new_with_refreshed_list();

        sys.refresh_all();

        let procs = read_procs(&mut sys, &mut users);

        assert!(!procs.is_empty());
    }

    #[test]
    fn test_procs_have_usernames_when_possible() {
        let mut sys = System::new_all();
        let mut users = Users::new_with_refreshed_list();

        sys.refresh_all();

        let procs = read_procs(&mut sys, &mut users);

        let some_have_usernames = procs.iter().any(|p| p.user.is_some());

        assert!(
            some_have_usernames,
            "Expected at least one proc with user info"
        );
    }
}
