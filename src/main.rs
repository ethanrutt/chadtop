pub mod cpu;
pub mod gpu;
pub mod hdd;
pub mod proc;
pub mod ram;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List},
    Terminal,
};

fn draw_screen(term: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> crossterm::Result<()> {
    term.draw(|f| {
        let size = f.size();
        let proc_list_widget = List::new(proc::read_and_convert_procs())
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_widget(proc_list_widget, size);
    })?;

    Ok(())
}

fn run_chadtop() -> crossterm::Result<()> {
    loop {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    // clean up threads?

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _ = draw_screen(&mut terminal);

    let _ = run_chadtop();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
