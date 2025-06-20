pub mod cpu;
pub mod info;
pub mod proc;
pub mod ram;
pub mod state;
pub mod ui;

use std::io::{self, Result};
use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event},
    prelude::Backend,
    Terminal,
};
use state::State;
use ui::ui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut state = State::new();
    let _ = run(&mut terminal, &mut state);

    ratatui::restore();

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    let mut elapsed = Instant::now();
    let refresh_interval = Duration::new(1, 0);

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

        if elapsed.elapsed() >= refresh_interval {
            state.refresh();
            elapsed = Instant::now();
        }
    }

    Ok(())
}
