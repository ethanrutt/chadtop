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
    let processes = sys.processes();
    let mut ret: Vec<Proc> = Vec::with_capacity(processes.len());

    for (pid, proc) in processes {
        let name: Option<String> = proc.name().to_str().map(str::to_string);

        let cmd: Option<String> = proc
            .cmd()
            .iter()
            .map(|s| s.to_str())
            .collect::<Option<Vec<&str>>>()
            .map(|v| v.join(" "));

        let exe: Option<String> = proc.exe().and_then(|s| s.to_str()).map(str::to_string);

        let cwd: Option<String> = proc.cwd().and_then(|s| s.to_str()).map(str::to_string);

        let disk_usage: DiskUsage = proc.disk_usage();

        let user = proc
            .user_id()
            .and_then(|u| users.get_user_by_id(u))
            .map(|u| u.name().to_string());

        ret.push(Proc {
            name,
            cmd,
            exe,
            pid: pid.as_u32(),
            cwd,
            memory: proc.memory(),
            ppid: proc.parent().map(|p| p.as_u32()),
            start_time: proc.start_time(),
            run_time: proc.run_time(),
            cpu_usage: proc.cpu_usage(),
            disk_usage_read: disk_usage.total_read_bytes,
            disk_usage_written: disk_usage.total_written_bytes,
            user,
            open_files: proc.open_files(),
            open_files_limit: proc.open_files_limit(),
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
