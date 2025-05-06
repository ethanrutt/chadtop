use std::process::Command;
use tui::widgets::ListItem;


pub struct Proc {
    pub uid: String,
    pub pid: String,
    pub ppid: String,
    pub c: String,
    pub stime: String,
    pub tty: String,
    pub time: String,
    pub comm: String,
    pub cmd: String,
}

pub fn read_and_convert_procs() -> Vec<ListItem<'static>> {
    let procs = read_procs();
    convert_to_list_item(procs)
}

pub fn read_procs() -> Vec<Proc> {
    let mut processes = Vec::<Proc>::new();
    let process = Command::new("ps")
        .arg("-eo")
        .arg("uid,pid,ppid,c,stime,tty,time,comm,cmd")
        .output()
        .ok()
        .expect("Failed to execute");

    let contents = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");

    let lines = contents.split("\n");

    for line in lines {
        let cols = line.split_whitespace().collect::<Vec<_>>();

        if !cols.is_empty() {
            processes.push(
                Proc {
                    uid: String::from(cols[0]),
                    pid: String::from(cols[1]),
                    ppid: String::from(cols[2]),
                    c: String::from(cols[3]),
                    stime: String::from(cols[4]),
                    tty: String::from(cols[5]),
                    time: String::from(cols[6]),
                    comm: String::from(cols[7]),
                    cmd: cols[8..].join(" "),
                }
            )
        }
    }

    if processes.is_empty() {
        panic!("no processes found");
    }

    // remove header since it's not a process
    processes.remove(0);
    processes
}

fn convert_to_list_item(proc_list: Vec<Proc>) -> Vec<ListItem<'static>> {
    let mut converted = Vec::<ListItem>::new();

    for p in proc_list {
        let s = String::from(format!("{}", p.comm));
        let l = ListItem::new(s);
        converted.push(l);
    }

    converted
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_procs() {
        let proc_info = read_procs();

        assert!(!proc_info.is_empty());

        for proc in proc_info {
            assert!(!proc.uid.is_empty());
            assert!(!proc.pid.is_empty());
            assert!(!proc.ppid.is_empty());
            assert!(!proc.c.is_empty());
            assert!(!proc.stime.is_empty());
            assert!(!proc.tty.is_empty());
            assert!(!proc.time.is_empty());
            assert!(!proc.comm.is_empty());
            assert!(!proc.cmd.is_empty());
        }
    }
}
