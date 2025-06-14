use std::fmt::{self, Display};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, Users};

use crate::{
    cpu::{read_cpus, CpuUsage}, info::{read_info, Info}, proc::{read_procs, Proc}, ram::{read_memory, Ram}
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    widgets::TableState,
};

// FIXME: add `Filter` to CurrentScreen option, this will send all key presses to state.filter and
// will also filter the processes
pub enum CurrentScreen {
    Main,
    ProcInfo,
    Filter,
    SysInfo,
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

// FIXME: add filter member that handles the filter text
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
            sys: System::new_all(),
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
                KeyCode::Char('i') => self.current_screen = CurrentScreen::SysInfo,
                KeyCode::Char('f') => self.current_screen = CurrentScreen::Filter,
                _ => {}
            },
            CurrentScreen::ProcInfo => match key.code {
                KeyCode::Char('d') => self.current_screen = CurrentScreen::Main,
                _ => {}
            },
            CurrentScreen::Filter => match key.code {
                KeyCode::Esc => self.current_screen = CurrentScreen::Main,
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
                KeyCode::Char('i') => self.current_screen = CurrentScreen::Main,
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
        // FIXME: initialize and make sure we only have what we need
        self.sys.refresh_all();
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
            ProcessSortStrategy::CpuUsage => self.processes.sort_by_key(|p| p.cpu_usage),
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
