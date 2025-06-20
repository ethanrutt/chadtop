use std::fmt::{self, Display};
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System, UpdateKind,
    Users,
};

use crate::{
    cpu::{read_cpus, CpuUsage},
    info::{read_info, Info},
    proc::{read_procs, Proc},
    ram::{read_memory, Ram},
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    widgets::TableState,
};

pub enum CurrentScreen {
    Main,
    ProcInfo,
    Filter,
    SysInfo,
    Help,
    KillConfirm,
}

pub enum ProcessSortStrategy {
    User,
    Pid,
    Ppid,
    CpuUsage,
    Alphabetical,
    Memory,
}

impl Display for ProcessSortStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ProcessSortStrategy::User => "user",
            ProcessSortStrategy::Pid => "pid",
            ProcessSortStrategy::Ppid => "ppid",
            ProcessSortStrategy::CpuUsage => "cpu usage",
            ProcessSortStrategy::Alphabetical => "name",
            ProcessSortStrategy::Memory => "memory",
        };

        write!(f, "{}", text)
    }
}

impl ProcessSortStrategy {
    pub fn get_color(&self) -> Color {
        match self {
            ProcessSortStrategy::User => Color::Cyan,
            ProcessSortStrategy::Pid => Color::Magenta,
            ProcessSortStrategy::Ppid => Color::Yellow,
            ProcessSortStrategy::CpuUsage => Color::Green,
            ProcessSortStrategy::Alphabetical => Color::LightRed,
            ProcessSortStrategy::Memory => Color::LightMagenta,
        }
    }
}

pub struct State {
    pub exit: bool,
    pub sys: System,
    pub users: Users,
    pub processes: Vec<Proc>,
    pub cpus: Vec<CpuUsage>,
    pub ram: Ram,
    pub info: Info,
    pub processes_state: TableState,
    pub process_sort_strategy: ProcessSortStrategy,
    pub current_screen: CurrentScreen,
    pub current_pid_watch: Option<u32>,
    pub filter: String,
}

impl State {
    pub fn new() -> State {
        let mut new = State {
            exit: false,
            sys: System::new_with_specifics(get_refresh_kind()),
            users: Users::new_with_refreshed_list(),
            processes: Vec::new(),
            cpus: Vec::new(),
            ram: Ram::new(),
            info: read_info(),
            processes_state: TableState::default(),
            process_sort_strategy: ProcessSortStrategy::CpuUsage,
            current_screen: CurrentScreen::Main,
            current_pid_watch: None,
            filter: String::new(),
        };
        new.refresh();
        new
    }

    pub fn handle_key(&mut self, key: &KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('j') => self.next_row(),
                KeyCode::Char('k') => self.previous_row(),
                KeyCode::Char('g') => self.first(),
                KeyCode::Char('G') => self.last(),
                KeyCode::Char('s') => self.next_sort_strategy(),
                KeyCode::Esc => self.select_none(),
                KeyCode::Char('d') => {
                    match self.processes_state.selected() {
                        Some(idx) => {
                            self.current_pid_watch = Some(self.processes[idx].pid);
                            self.current_screen = CurrentScreen::ProcInfo;
                        }
                        None => {
                            self.current_pid_watch = None;
                            self.current_screen = CurrentScreen::Main;
                        }
                    };
                }
                KeyCode::Char('K') => {
                    match self.processes_state.selected() {
                        Some(idx) => {
                            self.current_pid_watch = Some(self.processes[idx].pid);
                            self.current_screen = CurrentScreen::KillConfirm;
                        }
                        None => {
                            self.current_pid_watch = None;
                            self.current_screen = CurrentScreen::Main;
                        }
                    };
                }
                KeyCode::Char('i') => self.current_screen = CurrentScreen::SysInfo,
                KeyCode::Char('h') => self.current_screen = CurrentScreen::Help,
                KeyCode::Char('f') => self.current_screen = CurrentScreen::Filter,
                KeyCode::Backspace => {
                    self.filter.clear();
                    self.refresh_procs();
                }
                _ => {}
            },
            CurrentScreen::ProcInfo => match key.code {
                KeyCode::Char('d') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            CurrentScreen::Filter => match key.code {
                KeyCode::Esc | KeyCode::Enter => self.current_screen = CurrentScreen::Main,
                KeyCode::Char(value) => {
                    self.filter.push(value);
                    self.refresh_procs();
                }
                KeyCode::Backspace => {
                    self.filter.pop();
                    self.refresh_procs();
                }
                _ => {}
            },
            CurrentScreen::SysInfo => match key.code {
                KeyCode::Esc | KeyCode::Char('i') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            CurrentScreen::Help => match key.code {
                KeyCode::Esc | KeyCode::Char('h') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            CurrentScreen::KillConfirm => match key.code {
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.current_screen = CurrentScreen::Main
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => match self.current_pid_watch {
                    Some(pid) => {
                        self.sys
                            .process(Pid::from_u32(pid))
                            .and_then(|p| Some(p.kill()));

                        self.refresh_procs();
                        self.current_pid_watch = None;
                        self.current_screen = CurrentScreen::Main;
                    }
                    None => {
                        self.current_screen = CurrentScreen::Main;
                    }
                },
                _ => {}
            },
        }
    }

    pub fn next_sort_strategy(&mut self) {
        match self.process_sort_strategy {
            ProcessSortStrategy::User => self.process_sort_strategy = ProcessSortStrategy::Pid,
            ProcessSortStrategy::Pid => self.process_sort_strategy = ProcessSortStrategy::Ppid,
            ProcessSortStrategy::Ppid => self.process_sort_strategy = ProcessSortStrategy::CpuUsage,
            ProcessSortStrategy::CpuUsage => {
                self.process_sort_strategy = ProcessSortStrategy::Alphabetical
            }
            ProcessSortStrategy::Alphabetical => {
                self.process_sort_strategy = ProcessSortStrategy::Memory
            }
            ProcessSortStrategy::Memory => self.process_sort_strategy = ProcessSortStrategy::User,
        };
        self.refresh_procs();
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_specifics(get_refresh_kind());
        self.refresh_procs();
        self.ram = read_memory(&mut self.sys);
        self.cpus = read_cpus(&mut self.sys);
    }

    fn refresh_procs(&mut self) {
        self.processes = read_procs(&mut self.sys, &mut self.users);

        if self.filter.len() > 0 {
            self.processes.retain(|p| {
                let name = p.name.clone().unwrap_or(String::new());
                name.starts_with(&self.filter)
            });
        }

        match self.process_sort_strategy {
            ProcessSortStrategy::User => self.processes.sort_by_key(|p| p.user.clone()),
            ProcessSortStrategy::Pid => self.processes.sort_by_key(|p| p.pid),
            ProcessSortStrategy::Ppid => self.processes.sort_by_key(|p| p.ppid),
            ProcessSortStrategy::CpuUsage => self.processes.sort_by(|a, b| {
                a.cpu_usage
                    .partial_cmp(&b.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Greater)
            }),
            ProcessSortStrategy::Memory => self.processes.sort_by_key(|p| p.memory),
            ProcessSortStrategy::Alphabetical => self.processes.sort_by_key(|p| p.name.clone()),
        }
    }

    fn select_none(&mut self) {
        self.processes_state.select(None);
    }

    fn next_row(&mut self) {
        let i = match self.processes_state.selected() {
            Some(i) => {
                if i >= self.processes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.processes_state.select(Some(i));
    }

    fn previous_row(&mut self) {
        let i = match self.processes_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.processes_state.select(Some(i));
    }

    fn first(&mut self) {
        self.processes_state.select(Some(0));
    }

    fn last(&mut self) {
        self.processes_state.select(Some(self.processes.len() - 1));
    }
}

fn get_refresh_kind() -> RefreshKind {
    RefreshKind::nothing()
        .with_memory(MemoryRefreshKind::everything())
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
        .with_processes(
            ProcessRefreshKind::nothing()
                .with_cmd(UpdateKind::Always)
                .with_exe(UpdateKind::Always)
                .with_cwd(UpdateKind::Always)
                .with_memory()
                .with_cpu()
                .with_disk_usage()
                .with_user(UpdateKind::Always)
                .without_tasks(),
        )
}
