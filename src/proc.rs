use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System};

pub fn read_procs() -> Vec<&Process> {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let ret = Vec::<&Process>::new();
    for (pid, process) in sys.processes() {
        ret.push(process);
    }

    ret
}
