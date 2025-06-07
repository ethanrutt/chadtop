use std::fmt::{self, Display};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::TableState,
};

use crate::proc::{read_procs, Proc};

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
            processes: read_procs(),
            processes_state: TableState::default(),
            process_sort_strategy: ProcessSortStrategy::Time,
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
        let mut procs = read_procs();

        match self.process_sort_strategy {
            ProcessSortStrategy::Uid => procs.sort_by_key(|p| p.uid.clone()),
            ProcessSortStrategy::Pid => procs.sort_by_key(|p| p.pid.clone()),
            ProcessSortStrategy::Ppid => procs.sort_by_key(|p| p.ppid.clone()),
            ProcessSortStrategy::Stime => procs.sort_by_key(|p| p.stime.clone()),
            ProcessSortStrategy::Time => {
                procs.sort_by_key(|p| p.time.clone());
                procs.reverse();
            },
            ProcessSortStrategy::Alphabetical => procs.sort_by_key(|p| p.comm.clone()),
        }

        self.processes = procs;
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
