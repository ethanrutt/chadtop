use std::{
    cmp::Reverse,
    fmt::{self, Display},
};
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

#[derive(Clone, Debug)]
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
            ProcessSortStrategy::CpuUsage => {
                self.processes.sort_by(|a, b| {
                    // compare b to a because we want reverse sorting i.e. most cpu usage goes
                    // first or descending order
                    b.cpu_usage
                        .partial_cmp(&a.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Greater)
                });
            }
            ProcessSortStrategy::Memory => {
                // also want descending order, most memory usage goes first
                self.processes.sort_by_key(|p| Reverse(p.memory));
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyCode;
    use ratatui::crossterm::event::KeyEvent;

    #[test]
    fn test_state_initializes_correctly() {
        let state = State::new();

        assert!(
            !state.processes.is_empty(),
            "Expected some processes to be loaded"
        );
        assert!(!state.cpus.is_empty(), "Expected CPU info to be loaded");
        assert!(state.ram.total > 0, "Expected non-zero total RAM");
        assert!(
            !state.info.kernel_long_version.is_empty(),
            "Expected OS info to be loaded"
        );
    }

    #[test]
    fn test_sort_strategy_cycles_through_all_variants() {
        let mut state = State::new();
        let original = &state.process_sort_strategy;

        let mut seen = std::collections::HashSet::new();
        seen.insert(format!("{}", original));

        for _ in 0..10 {
            state.next_sort_strategy();
            seen.insert(format!("{}", state.process_sort_strategy));
        }

        assert_eq!(seen.len(), 6);
    }

    #[test]
    fn test_quit_key_sets_exit_true() {
        let mut state = State::new();
        assert!(!state.exit);
        state.handle_key(&KeyEvent::from(KeyCode::Char('q')));
        assert!(state.exit, "Expected 'q' to set exit flag");
    }

    #[test]
    fn test_handle_key_s_changes_sort_strategy() {
        let mut state = State::new();
        let before = format!("{}", state.process_sort_strategy);
        state.handle_key(&KeyEvent::from(KeyCode::Char('s')));
        let after = format!("{}", state.process_sort_strategy);
        assert_ne!(before, after, "Sort strategy should change on 's' key");
    }

    #[test]
    fn test_sysinfo_screen() {
        let mut state = State::new();
        let before = state.current_screen.clone();
        assert!(matches!(before, CurrentScreen::Main));
        state.handle_key(&KeyEvent::from(KeyCode::Char('i')));
        let after = state.current_screen.clone();
        assert!(matches!(after, CurrentScreen::SysInfo));
    }

    #[test]
    fn test_help_screen() {
        let mut state = State::new();
        let before = state.current_screen.clone();
        assert!(matches!(before, CurrentScreen::Main));
        state.handle_key(&KeyEvent::from(KeyCode::Char('h')));
        let after = state.current_screen.clone();
        assert!(matches!(after, CurrentScreen::Help));
    }
}
