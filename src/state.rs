use std::fmt::{self, Display};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::Color,
    widgets::TableState,
};
use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System};

pub enum CurrentScreen {
    Main,
    Kill,
    KillConfirm,
}

pub enum ProcessSortStrategy {
    Uid,
    Pid,
    Ppid,
    Stime,
    Time,
    Alphabetical,
}

impl Display for ProcessSortStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ProcessSortStrategy::Uid => "uid",
            ProcessSortStrategy::Pid => "pid",
            ProcessSortStrategy::Ppid => "ppid",
            ProcessSortStrategy::Stime => "stime",
            ProcessSortStrategy::Time => "time",
            ProcessSortStrategy::Alphabetical => "proc",
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
            ProcessSortStrategy::Stime => Color::Blue,
            ProcessSortStrategy::Time => Color::Green,
            ProcessSortStrategy::Alphabetical => Color::LightRed,
        }
    }
}

pub struct State {
    pub exit: bool,
    pub sys: System,
    pub processes: Vec<&'a Process>,
    pub processes_state: TableState,
    pub process_sort_strategy: ProcessSortStrategy,
    pub current_screen: CurrentScreen,
}

impl State {
    pub fn new() -> State {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let procs = Vec::<&Process>::new();
        for (pid, process) in sys.processes() {
            procs.push(process);
        }

        let mut new = State {
            exit: false,
            sys: sys,
            processes: procs,
            processes_state: TableState::default(),
            process_sort_strategy: ProcessSortStrategy::Time,
            current_screen: CurrentScreen::Main,
        };

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
            ProcessSortStrategy::Ppid => self.process_sort_strategy = ProcessSortStrategy::Stime,
            ProcessSortStrategy::Stime => self.process_sort_strategy = ProcessSortStrategy::Time,
            ProcessSortStrategy::Time => {
                self.process_sort_strategy = ProcessSortStrategy::Alphabetical
            }
            ProcessSortStrategy::Alphabetical => {
                self.process_sort_strategy = ProcessSortStrategy::Uid
            }
        }
    }

    pub fn refresh_procs(&mut self) {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        self.processes.clear();
        for (pid, process) in self.sys.processes() {
            self.processes.push(process);
        }

        match self.process_sort_strategy {
            ProcessSortStrategy::Uid => self.processes.sort_by_key(|p| p.user_id()),
            ProcessSortStrategy::Pid => self.processes.sort_by_key(|p| p.pid()),
            ProcessSortStrategy::Ppid => self.processes.sort_by_key(|p| p.parent()),
            ProcessSortStrategy::Stime => self.processes.sort_by_key(|p| p.start_time()),
            ProcessSortStrategy::Time => self.processes.sort_by_key(|p| p.run_time()),
            ProcessSortStrategy::Alphabetical => self.processes.sort_by_key(|p| p.name()),
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
