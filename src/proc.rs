use std::process::Command;

pub struct Proc {
    pub uid: String,
    pub pid: String,
    pub ppid: String,
    pub c: String,
    pub stime: String,
    pub tty: String,
    pub time: String,
    pub cmd: String,
}

pub fn read_procs() -> Vec<Proc> {
    let process = Command::new("ps")
        .arg("-ef")
        .output()
        .ok()
        .expect("Failed to execute");

    let contents = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");

    let lines = contents.split("\n");

    return Vec::<Proc>::new();
}
