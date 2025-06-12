use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Clear, HighlightSpacing, Paragraph, Row, Table},
    Frame,
};

use crate::state::{CurrentScreen, State};

/// handles ui for chadtop
pub fn ui(frame: &mut Frame, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(frame.area());

    render_title(frame, &chunks);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(chunks[1]);

    render_proc_list(frame, body_chunks[1], state);

    match state.current_screen {
        CurrentScreen::Kill => {
            // FIXME: make this a list instead of a table
            let proc_idx = state
                .processes
                .iter()
                .position(|p| p.pid == state.current_pid_watch.expect("no pid watching"));

            let proc = &state.processes[proc_idx.expect("no proc index")];

            let row = [
                proc.pid.to_string(),
                proc.start_time.to_string(),
                proc.run_time.to_string(),
                proc.disk_usage_read.to_string(),
                proc.disk_usage_written.to_string(),
                proc.open_files.unwrap_or(0).to_string(),
                proc.open_files_limit.unwrap_or(0).to_string(),
                proc.cwd.clone().unwrap_or(String::from("n/a")),
                proc.exe.clone().unwrap_or(String::from("n/a")),
                proc.cmd.clone().unwrap_or(String::from("n/a")),
            ];

            let row = row.into_iter().map(|r| Cell::new(r)).collect::<Row>();

            let proc_table_header = [
                "pid",
                "start_time",
                "run_time",
                "disk_usage_read",
                "disk_usage_written",
                "open_files",
                "open_files_limit",
                "cwd",
                "exe",
                "cmd",
            ]
            .into_iter()
            .map(|h| Cell::new(h))
            .collect::<Row>()
            .style(Style::default().fg(Color::Blue))
            .bold()
            .height(1);

            let popup_block = Block::default()
                .borders(Borders::NONE)
                .title(proc.name.clone().expect("no proc name"))
                .style(Style::default().bg(Color::Black));

            let area = centered_rect(80, 50, frame.area());

            let t = Table::new(
                std::iter::once(row),
                [
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(7),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Length(20),
                ],
            )
            .header(proc_table_header)
            .block(popup_block);

            frame.render_widget(Clear, area);
            frame.render_widget(t, area);
        }
        _ => {}
    }
}

/// renders the title of chadtop
///
/// # Assumptions
/// We assume that the `chunks` parameter is a horizontal layout split into two parts, the first
/// part being 33% and the second part being 67% of the screen
fn render_title(frame: &mut Frame, chunks: &Rc<[Rect]>) {
    let title_chunks = Layout::default()
        .flex(Flex::Center)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .split(chunks[0]);

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "

┏┣┓┏┓┏┫╋┏┓┏┓
┗┛┗┗┻┗┻┗┗┛┣┛
        ",
        Style::default().fg(Color::LightBlue),
    ))
    .centered()
    .block(title_block);

    let keybinds_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let keybinds = Paragraph::new(Text::styled(
        "(q) quit | (j) navigate down processes | (k) navigate up processes
(g) go to first process | (G) go to last process
(s) cycle next sort strategy
(d) bring up signals menu for current selected process
        ",
        Style::default(),
    ))
    .left_aligned()
    .block(keybinds_block);

    let gigachad_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let gigachad_art = Paragraph::new(Text::styled(
        "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣶⣤⣄⡠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣻⣿⣿⣿⣿⣿⣿⣿⠉⠉⠛⠛⠛⠛⠿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠲⠶⢤⡄⠀⠘⢿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⡿⠁⣂⡀⠀⠀⠀⠀⠀⠦⠤⣽⠷⣾⢷⢮⣀⠀⢻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⡷⠟⠉⠀⠀⢀⣠⣶⣶⣶⣤⣠⣤⡟⠈⢻⡿⠀⠘⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀⠠⠼⠿⢻⣿⣻⣿⣿⣿⣿⣿⣷⣾⣣⣇⡀⢻⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⢿⡿⣿⣿⣿⠁⠀⠀⠀⠲⣶⣷⣿⣿⣿⣿⣿⣿⣿⠟⢻⣟⣿⣿⣿⣾⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⠃⡼⣁⢀⣿⠁⠀⠀⠀⠀⠀⠉⠙⢿⣿⣿⣿⣿⣿⣿⠀⢸⣿⣿⣿⣿⣭⣛⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⡟⠻⡟⣿⣷⡆⠀⠀⠀⠀⠀⢀⣠⣬⣿⣿⣿⣿⡋⠀⠈⢻⣟⣿⣌⣻⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣿⢷⣤⢁⣿⡿⣿⣀⣀⢀⡀⣼⠿⠟⢋⣿⣿⣿⣯⢉⣓⣂⠈⣿⠙⣿⠉⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠡⣤⡷⣾⣿⠷⢻⣿⣷⣿⣿⡟⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⡟⣿⣼⣷⠀⣼⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢀⣿⠛⣷⣴⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠗⢿⣽⡏⣸⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⢸⣿⣟⠙⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡻⢶⣿⣏⣙⡋⢳⣦⣿⣵⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣭⣽⣿⣿⡊⢹⣿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠉⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣾⣿⡃⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠈⠹⠻⣿⣿⣿⣿⢟⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣠⣤⠶⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠒⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⣀⣤⡶⠖⠛⠛⠉⠉⣁⣤⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⣿⣿⡿⠉⠁⢸⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢀⣤⣶⠿⠛⠉⠀⠀⠀⠀⠀⠀⣼⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡀⠀⠀⢰⣾⣿⡷⠀⠀⣾⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠟⠁⠀⠀⠃⢀⠈⠉⢁⣨⣭⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⡇⠀⣀⣿⣿⡏⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡀⠀⠠⠆⠒⣿⣶⣾⣿⣿⣿⡿⡂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣷⠀⣿⣿⡏⠀⢸⠁⠀⡙⠷⢦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣶⣦⣶⣿⣿⣿⣿⣿⣿⣿⣧⣥⣄⡀⠀⠀⠀⣄⣀⠀⠀⡀⠀⠀⠀⣿⣿⣿⢿⣿⢀⣴⡻⠀⠀⣧⠀⠀⠈⠛⠶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣷⣄⠀⠀⠀⠈⠉⠉⠉⠉⠛⠛⠛⠛⠿⠿⠿⣿⣷⣶⣿⣄⠀⢰⣿⢻⣿⣸⣿⣾⠁⠁⠇⠀⢿⣷⡤⣄⣀⠀⠈⠻⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠈⠻⠿⣿⣧⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⣀⠀⠈⠉⠙⠿⠷⣿⣏⠀⣿⣿⣿⡇⠀⠈⡀⠀⠈⠻⣷⡈⠻⢿⣤⠀⠈⠻⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠉⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡋⠀⠉⠠⠀⠈⠻⣄⢹⣿⡟⢀⣀⡴⢿⣶⠒⠺⠟⠛⠂⠀⢻⡆⢰⣶⣬⣍⣉⠓⠶⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣷⠀⠀⠂⠶⢤⣄⡙⠀⠻⠁⣾⣿⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣾⣿⣿⡟⠋⠙⠓⠂⠙⠓⠒⠶⢤⣤⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⠿⠶⠀⠈⠀⠒⠀⢺⣿⣷⣿⣙⡻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣷⣀⢤⡴⢄⠀⠀⠀⠀⠀⠀⠉⠳⠦⣄⠀⠀⠀
⠈⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⠀⠘⣿⣿⡿⣿⡟⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣯⣷⣘⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣦⡀
⠴⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠳⣄⠀⢸⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻
⠀⠀⠐⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠄⠀⠀⠙⠀⢸⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⣹⣿⠷⣦⣁⠂⠀⠀⠀⠀⠀⠀
⠤⠄⠀⠀⠀⣀⡀⢀⡀⠤⠀⠉⢀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣦⠈⠛⢷⣄⠀⠀⠀⠀⠀
⠛⠀⠀⠀⠀⠀⠠⠄⢀⡀⢀⣤⢠⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⠿⣿⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣦⠀⠈⠙⠀⠀⠀⠀⠀
⠀⠀⠀⣐⣂⣀⣀⠀⣶⣶⣾⢉⣴⢾⣿⣷⣤⣤⣤⣤⣠⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡄⢀⣀⠀⠄⠈⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣦⡀⣀⣀⣀⣀⠀⢀
",
        Style::default(),
    ))
    .centered()
    .block(gigachad_block);

    frame.render_widget(title, title_chunks[0]);
    frame.render_widget(keybinds, title_chunks[1]);
    frame.render_widget(gigachad_art, title_chunks[2]);
}

/// renders the process list and handles state for drawing selector, sorting differently, etc.
///
/// # Assumptions
/// We assume that the `chunks` parameter is a horizontal layout split into two
/// parameters, the first part being 33% and the second part being 67% of the screen
/// This second part is then split into the bottom 80% of the screen
///
/// We also assume that the argument `state` is already initialized
fn render_proc_list(frame: &mut Frame, chunk: Rect, state: &mut State) {
    // right 67% of the screen chunk, split into the bottom 80% of the screen
    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(Color::Blue);
    let selected_col_style = Style::default().fg(Color::DarkGray);
    let selected_cell_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(Color::DarkGray);

    let process_block = Block::new()
        .title(Line::raw("processes").centered())
        .title(
            Line::raw(format!("{}", state.process_sort_strategy))
                .centered()
                .style(Style::default().fg(state.process_sort_strategy.get_color())),
        )
        .borders(Borders::TOP);

    let process_table_header = ["pid", "name", "memory", "cpu usage", "uid", "ppid"]
        .into_iter()
        .map(|h| Cell::new(h))
        .collect::<Row>()
        .style(Style::default().fg(Color::Blue))
        .bold()
        .height(1);

    let rows = state.processes.iter().map(|process| {
        // bytes to mb
        let memory = process.memory / 1000000;
        let row = [
            process.pid.to_string(),
            process.name.clone().unwrap_or(String::from("n/a")),
            memory.to_string() + " mb",
            process.cpu_usage.to_string() + "%",
            match process.uid {
                Some(x) => x.to_string(),
                None => String::from("n/a"),
            },
            match process.ppid {
                Some(x) => x.to_string(),
                None => String::from("n/a"),
            },
        ];

        row.into_iter()
            .map(|p| Cell::new(p))
            .collect::<Row>()
            .style(Style::default().bg(Color::DarkGray))
            .height(1)
    });

    let t = Table::new(
        rows,
        [
            Constraint::Length(7),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(6),
            Constraint::Length(9),
            Constraint::Length(6),
        ],
    )
    .header(process_table_header)
    .row_highlight_style(selected_row_style)
    .column_highlight_style(selected_col_style)
    .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from(vec![" > ".into()]))
    .highlight_spacing(HighlightSpacing::Always)
    .block(process_block);

    frame.render_stateful_widget(t, chunk, &mut state.processes_state);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
/// taken from ratatui.rs json editor
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
