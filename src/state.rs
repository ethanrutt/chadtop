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

pub struct State {
    pub exit: bool,
    pub processes: Vec<Proc>,
    pub processes_state: TableState,
    pub current_screen: CurrentScreen,
}

impl State {
    pub fn new() -> State {
        State {
            exit: false,
            processes: read_procs(),
            processes_state: TableState::default(),
            current_screen: CurrentScreen::Main,
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
                if i >= self.processes.len() - 1 {
                    0
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

    pub fn handle_key(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('j') => self.next_row(),
            KeyCode::Char('k') => self.previous_row(),
            KeyCode::Char('g') => self.first(),
            KeyCode::Char('G') => self.last(),
            KeyCode::Esc => self.select_none(),
            _ => {}
        }
    }
}
