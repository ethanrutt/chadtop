pub mod cpu;
pub mod gpu;
pub mod hdd;
pub mod proc;
pub mod ram;
pub mod state;
pub mod ui;

use std::{
    io::{self, Result},
    time::Duration,
};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use state::State;
use ui::ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = State::new();
    let _ = run(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    while !state.exit {
        terminal.draw(|f| ui(f, state))?;

        if event::poll(Duration::new(1, 0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                state.handle_key(&key);
            }
        }

        state.refresh_procs();
    }

    Ok(())
}
