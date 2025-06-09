use std::fmt::{self, Display};

use crate::proc::{read_procs, Proc};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    widgets::TableState,
};

pub enum CurrentScreen {
    Main,
    Kill,
    KillConfirm,
}

pub enum ProcessSortStrategy {
    Uid,
    Pid,
    Ppid,
    Guid,
    Threads,
    CpuUsage,
    Alphabetical,
    Memory,
}

impl Display for ProcessSortStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ProcessSortStrategy::Uid => "uid",
            ProcessSortStrategy::Pid => "pid",
            ProcessSortStrategy::Ppid => "ppid",
            ProcessSortStrategy::Guid => "guid",
            ProcessSortStrategy::Threads => "threads",
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
            ProcessSortStrategy::Uid => Color::Cyan,
            ProcessSortStrategy::Pid => Color::Magenta,
            ProcessSortStrategy::Ppid => Color::Yellow,
            ProcessSortStrategy::Guid => Color::LightGreen,
            ProcessSortStrategy::Threads => Color::Blue,
            ProcessSortStrategy::CpuUsage => Color::Green,
            ProcessSortStrategy::Alphabetical => Color::LightRed,
            ProcessSortStrategy::Memory => Color::LightMagenta,
        }
    }
}

pub struct State {
    pub exit: bool,
    pub processes: Vec<Proc>,
    pub processes_state: TableState,
    pub process_sort_strategy: ProcessSortStrategy,
    pub current_screen: CurrentScreen,
}

impl State {
    pub fn new() -> State {
        let mut new = State {
            exit: false,
            processes: read_procs().expect("couldn't read procs"),
            processes_state: TableState::default(),
            process_sort_strategy: ProcessSortStrategy::CpuUsage,
            current_screen: CurrentScreen::Main,
        };
        new.refresh_procs();
        new
    }

    pub fn handle_key(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('j') => self.next_row(),
            KeyCode::Char('k') => self.previous_row(),
            KeyCode::Char('g') => self.first(),
            KeyCode::Char('G') => self.last(),
            KeyCode::Char('s') => self.next_sort_strategy(),
            KeyCode::Esc => self.select_none(),
            _ => {}
        }
    }

    pub fn next_sort_strategy(&mut self) {
        match self.process_sort_strategy {
            ProcessSortStrategy::Uid => self.process_sort_strategy = ProcessSortStrategy::Pid,
            ProcessSortStrategy::Pid => self.process_sort_strategy = ProcessSortStrategy::Ppid,
            ProcessSortStrategy::Ppid => self.process_sort_strategy = ProcessSortStrategy::Guid,
            ProcessSortStrategy::Guid => self.process_sort_strategy = ProcessSortStrategy::Threads,
            ProcessSortStrategy::Threads => {
                self.process_sort_strategy = ProcessSortStrategy::CpuUsage
            }
            ProcessSortStrategy::CpuUsage => {
                self.process_sort_strategy = ProcessSortStrategy::Alphabetical
            }
            ProcessSortStrategy::Alphabetical => {
                self.process_sort_strategy = ProcessSortStrategy::Memory
            }
            ProcessSortStrategy::Memory => self.process_sort_strategy = ProcessSortStrategy::Uid,
        }
    }

    pub fn refresh_procs(&mut self) {
        self.processes = read_procs().expect("couldn't read procs");

        match self.process_sort_strategy {
            ProcessSortStrategy::Uid => self.processes.sort_by_key(|p| p.uid),
            ProcessSortStrategy::Guid => self.processes.sort_by_key(|p| p.gid),
            ProcessSortStrategy::Pid => self.processes.sort_by_key(|p| p.pid),
            ProcessSortStrategy::Ppid => self.processes.sort_by_key(|p| p.ppid),
            ProcessSortStrategy::Threads => self.processes.sort_by_key(|p| p.threads),
            ProcessSortStrategy::CpuUsage => self.processes.sort_by_key(|p| p.cpu_usage),
            ProcessSortStrategy::Memory => self.processes.sort_by_key(|p| p.mem),
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
