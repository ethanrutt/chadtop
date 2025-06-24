use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::Title, Block, Borders, Cell, Clear, HighlightSpacing, List, ListItem, Padding,
        Paragraph, Row, Table,
    },
    Frame,
};

use crate::state::{CurrentScreen, State};

/// handles ui for chadtop
pub fn ui(frame: &mut Frame, state: &mut State) {
    if state.debug {
        return ui_debug(frame, state);
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(frame.area());

    render_title(frame, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Percentage(90),
        ])
        .split(chunks[1]);

    render_filter(frame, body_chunks[1], state);

    render_proc_list(frame, body_chunks[2], state);

    match state.current_screen {
        CurrentScreen::ProcInfo => render_proc_info_popup(frame, state),
        CurrentScreen::SysInfo => render_sysinfo(frame, state),
        CurrentScreen::Help => render_help(frame),
        CurrentScreen::KillConfirm => render_killconfirm(frame, state),
        _ => {}
    }
}

pub fn ui_debug(frame: &mut Frame, state: &mut State) {
    let i = Instant::now();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
        .split(frame.area());

    render_title(frame, chunks[0]);

    let title_elapsed = i.elapsed();
    let i = Instant::now();

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Percentage(90),
        ])
        .split(chunks[1]);

    render_filter(frame, body_chunks[1], state);

    let filter_elapsed = i.elapsed();
    let i = Instant::now();

    render_proc_list(frame, body_chunks[2], state);

    let proc_list_elapsed = i.elapsed();
    let i = Instant::now();

    match state.current_screen {
        CurrentScreen::ProcInfo => render_proc_info_popup(frame, state),
        CurrentScreen::SysInfo => render_sysinfo(frame, state),
        CurrentScreen::Help => render_help(frame),
        CurrentScreen::KillConfirm => render_killconfirm(frame, state),
        _ => {}
    }

    let popup_elapsed = i.elapsed();

    render_debug(
        frame,
        state,
        title_elapsed,
        filter_elapsed,
        proc_list_elapsed,
        popup_elapsed,
        body_chunks[0],
    );
}

/// renders the title of chadtop
///
/// # Assumptions
/// We assume that the `chunks` parameter is a horizontal layout split into two parts
fn render_title(frame: &mut Frame, chunk: Rect) {
    let title_chunks = Layout::default()
        .flex(Flex::Center)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Max(4),
            Constraint::Fill(1),
        ])
        .split(chunk);

    let no_border_block = Block::default().borders(Borders::NONE);

    let title = Paragraph::new(Text::styled(
        "
┏┣┓┏┓┏┫╋┏┓┏┓
┗┛┗┗┻┗┻┗┗┛┣┛
        ",
        Style::default().fg(Color::LightBlue),
    ))
    .centered()
    .block(no_border_block.clone());

    let welcome = Paragraph::new(Text::raw("welcome to chadtop\npress [h]elp for keybinds"))
        .centered()
        .block(no_border_block.clone());

    let gigachad_art = Paragraph::new(Text::raw(if chunk.height < 30 && chunk.width < 45 {
        "
⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣶⣦⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢰⣿⣿⣿⠟⠁⠀⠉⢩⡯⡉⢣⡀⠀⠀⠀⠀⠀
⠀⠀⠀⣾⣿⣿⡿⠒⢀⣠⣬⣬⣽⣹⠆⢇⠀⠀⠀⠀⠀
⠀⠀⠀⡻⡿⡿⠁⠀⠽⣾⣿⣿⡟⢿⣷⣾⡄⠀⠀⠀⠀
⠀⠀⠀⣿⡓⣷⡄⠀⢀⣬⣿⣿⢇⠸⣿⣿⠁⠀⠀⠀⠀
⠀⠀⠀⠡⢳⣿⣻⣿⣯⣴⣿⣿⣿⣷⣷⢠⠇⠀⠀⠀⠀
⠀⠀⠀⠀⢸⡎⣿⣿⣿⣿⣿⣺⣿⡵⣾⠏⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠈⠻⢿⣿⣿⣿⣿⣿⣿⣯⡟⠀⠀⠀⠀⠀⠀
⠀⠀⢠⠀⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⡥⠃⠀⠀⠀⠀⠀
⠒⣊⠁⠀⠀⠀⠀⡀⠀⢹⡿⠛⠛⠋⠁⠀⠀⠀⠀⠀⠀
⡼⠃⠀⠀⠀⠀⠀⡆⢀⣿⢃⢸⡀⠀⠀⠀⠀⠀⠀⠀⠀
⣧⣄⣀⣀⡀⡀⠀⣿⣾⣧⠆⢰⡉⠒⢤⡀⠀⠀⠀⠀⠀
"
    } else if chunk.height < 30 {
        "
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣾⣿⣿⣽⢿⡆⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⡿⢋⣝⠛⠿⠿⣿⣿⠛⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⡿⠛⣥⣿⣥⢰⠃⠀⠈⡿⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣟⣿⠣⣤⣿⣿⣿⣿⣷⣷⣤⡇⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣆⠙⣿⣮⣉⣿⣿⠛⠛⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⣿⢻⣿⣿⣿⣿⣿⣿⣿⡂⠎⠀⠀⠀⠀⠀
⢤⣶⣶⣦⣤⣶⣶⠿⠻⢭⣻⡟⠁⢸⡿⠁⣝⢿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀
⠟⣹⣴⣿⣿⣿⣿⡿⣾⣿⡏⠠⢰⢸⣧⢀⣼⡿⠟⠛⠛⡛⠫⢀⠀⠀⠀⠀⠀⠀
⡏⢛⣽⣿⣿⣿⣿⣇⣻⣿⣧⣀⢸⠘⣧⡟⣧⣠⠄⠀⢠⣿⣶⣄⠑⢄⠀⠀⠀⠀
⢀⣾⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⢸⣿⣗⣿⣟⡴⠂⣿⡿⠟⠳⠷⢄⠁⡀⠀⠀
⡏⢙⡟⣿⣿⣿⣿⣿⣿⣿⣿⣿⠻⣿⣿⣟⡁⣑⣤⠀⠀⢰⠀⠠⠀⠀⠁⠈⠂⢄
⣿⣘⣫⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣿⣧⣬⣿⣿⣯⣧⣤⣟⣥⣔⣀⣀⣀⣀⣀⣀
"
    } else if chunk.width < 45 {
        "
⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⣶⣤⣤⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄⠀⠀⠀⠀⠀
⠀⠀⢀⣾⣿⣿⣿⣿⣿⡿⠋⠉⠛⠛⠛⠿⣿⠿⠿⢿⣇⠀⠀⠀⠀
⠀⠀⣾⣿⣿⣿⣿⣿⠟⠀⠀⠀⠀⠀⡀⢀⣽⣷⣆⡀⠙⣧⠀⠀⠀
⠀⢰⣿⣿⣿⣿⣿⣷⠶⠋⠀⠀⣠⣤⣤⣉⣉⣿⠙⣿⠀⢸⡆⠀⠀
⠀⢸⣿⣿⣿⣿⣿⠁⠀⠀⠴⡟⣻⣿⣿⣿⣿⣿⣶⣿⣦⡀⣇⠀⠀
⠀⢨⠟⡿⠻⣿⠃⠀⠀⠀⠻⢿⣿⣿⣿⣿⣿⠏⢹⣿⣿⣿⢿⡇⠀
⠀⣿⣼⣷⡶⣿⣄⠀⠀⠀⠀⠀⢉⣿⣿⣿⡿⠀⠸⣿⣿⡿⣷⠃⠀
⠀⢻⡿⣦⢀⣿⣿⣄⡀⣀⣰⠾⠛⣻⣿⣿⣟⣲⡀⢸⡿⡟⠹⡆⠀
⠀⠀⢰⠞⣾⣿⡛⣿⣿⣿⣿⣰⣾⣿⣿⣿⣿⣿⣿⣿⣿⡇⢰⡇⠀
⠀⠀⠘⠀⣿⡽⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⠿⣍⣿⣧⡏⠀⠀
⠀⠀⠀⠀⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣷⣮⣽⣿⣷⣙⣿⡟⠀⠀⠀
⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⣹⡿⠇⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡧⣦⠀⠀⠀
⢠⡆⠀⠀⠀⠀⠀⠀⠀⠉⠻⣿⣿⣾⣿⣿⣿⣿⣿⣿⡶⠏⠀⠀⠀
⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠚⣿⣿⣿⠿⣿⣿⠿⠟⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢠⠀⠀⢀⣿⣿⠁⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣾⠀⠀⣾⣿⠋⠀⢠⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣿⣆⣼⣿⠁⢠⠃⠈⠓⠦⣄⡀⠀⠀⠀⠀⠀
⣤⣤⣦⣄⣀⣀⠀⢀⣿⣿⠻⣿⣰⠻⠀⠸⣧⡀⠀⠉⠳⣄⠀⠀⠀
⠈⠉⠉⠙⠛⠿⣦⣼⡏⢻⣿⣿⠇⠀⠁⠀⠻⣿⠙⣶⣄⠈⠳⣄⡀
⠀⠀⠁⣐⠀⠀⠀⠈⠳⡘⣿⡟⣀⡠⠿⠶⠒⠟⠓⠀⠹⡄⢴⣬⣍
⠀⠀⠀⠙⢀⣀⠐⠲⠤⠁⢘⣠⣿⣷⣦⠀⠀⠀⠀⠀⠀⠙⢿⣿⣏
⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠈⣿⣿⣷⣯⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻
⠀⠀⠀⠀⠀⠀⠀⠀⠘⢦⠀⢹⣿⣏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⣸⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣤⣀⣄⣀⡀⠀⠀⠀⠀⠀⠀⠈⣿⣿⡄⣉⡀⠀⠀⠀⠀⠀⠀⠀⢀

"
    } else {
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
"
    }))
    .centered()
    .block(no_border_block.clone());

    frame.render_widget(title, title_chunks[0]);
    frame.render_widget(welcome, title_chunks[1]);
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
    let process_block = Block::new()
        .title(Line::styled(
            "processes",
            Style::default().fg(Color::LightBlue),
        ))
        .title(Line::styled(
            format!("{}", state.process_sort_strategy),
            Style::default().fg(state.process_sort_strategy.get_color()),
        ))
        .borders(Borders::LEFT | Borders::TOP)
        .padding(Padding::left(1));

    let process_table_header = ["pid", "name", "memory", "cpu usage", "user", "ppid"]
        .into_iter()
        .map(|h| Cell::new(h))
        .collect::<Row>()
        .style(Style::default().fg(Color::Blue))
        .bold()
        .height(1);

    let rows = state.processes.iter().map(|process| {
        // bytes to mb
        let row = [
            process.pid.to_string(),
            process.name.clone().unwrap_or(String::from("n/a")),
            bytes_to_str(process.memory),
            format!("{:.2}%", process.cpu_usage),
            match &process.user {
                Some(x) => x.clone(),
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
            Constraint::Length(30),
            Constraint::Length(15),
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(7),
        ],
    )
    .header(process_table_header)
    .row_highlight_style(
        Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Blue),
    )
    .highlight_symbol(Text::raw(" > "))
    .highlight_spacing(HighlightSpacing::Always)
    .block(process_block);

    frame.render_stateful_widget(t, chunk, &mut state.processes_state);
}

/// renders the additional info about a process
///
/// # Assumptions
/// That the state.CurrentScreen is set to ProcInfo and there is a pid in state.current_pid_watch
fn render_proc_info_popup(frame: &mut Frame, state: &mut State) {
    let proc_idx = state
        .processes
        .iter()
        .position(|p| p.pid == state.current_pid_watch.expect("no pid watching"));

    let proc_idx = match proc_idx {
        Some(x) => x,
        None => {
            // if we can't find the process then we don't render this
            // anymore
            state.current_pid_watch = None;
            state.current_screen = CurrentScreen::Main;
            return;
        }
    };

    let proc = &state.processes[proc_idx];

    let pad_len = 18;

    let mut pid = proc.pid.to_string();
    let mut pid_key = String::from("pid");
    right_pad(&mut pid_key, pad_len);
    let pid_key = pid_key + ": ";
    pid.insert_str(0, &pid_key);

    let mut start_time = proc.start_time.to_string();
    let mut time_key = String::from("start time");
    right_pad(&mut time_key, pad_len);
    let time_key = time_key + ": ";
    start_time.insert_str(0, &time_key);

    let mut run_time = proc.run_time.to_string();
    let mut time_key = String::from("run time");
    right_pad(&mut time_key, pad_len);
    let time_key = time_key + ": ";
    run_time.insert_str(0, &time_key);

    let mut disk_usage_read = bytes_to_str(proc.disk_usage_read);
    let mut read_key = String::from("disk read");
    right_pad(&mut read_key, pad_len);
    let read_key = read_key + ": ";
    disk_usage_read.insert_str(0, &read_key);

    let mut disk_usage_written = bytes_to_str(proc.disk_usage_written);
    let mut written_key = String::from("disk written");
    right_pad(&mut written_key, pad_len);
    let written_key = written_key + ": ";
    disk_usage_written.insert_str(0, &written_key);

    let mut open_files = proc.open_files.unwrap_or(0).to_string();
    let mut files_key = String::from("open files");
    right_pad(&mut files_key, pad_len);
    let files_key = files_key + ": ";
    open_files.insert_str(0, &files_key);

    let mut open_files_limit = proc.open_files_limit.unwrap_or(0).to_string();
    let mut limit_key = String::from("open files limit");
    right_pad(&mut limit_key, pad_len);
    let limit_key = limit_key + ": ";
    open_files_limit.insert_str(0, &limit_key);

    let mut cwd = proc.cwd.clone().unwrap_or(String::from("n/a"));
    let mut cwd_key = String::from("cwd");
    right_pad(&mut cwd_key, pad_len);
    let cwd_key = cwd_key + ": ";
    cwd.insert_str(0, &cwd_key);

    let mut exe = proc.exe.clone().unwrap_or(String::from("n/a"));
    let mut exe_key = String::from("exe");
    right_pad(&mut exe_key, pad_len);
    let exe_key = exe_key + ": ";
    exe.insert_str(0, &exe_key);

    let mut cmd = proc.cmd.clone().unwrap_or(String::from("n/a"));
    let mut cmd_key = String::from("cmd");
    right_pad(&mut cmd_key, pad_len);
    let cmd_key = cmd_key + ": ";
    cmd.insert_str(0, &cmd_key);

    let items: Vec<ListItem> = Vec::from([
        ListItem::from(pid),
        ListItem::from(start_time),
        ListItem::from(run_time),
        ListItem::from(disk_usage_read),
        ListItem::from(disk_usage_written),
        ListItem::from(open_files),
        ListItem::from(open_files_limit),
        ListItem::from(cwd),
        ListItem::from(exe),
        ListItem::from(cmd),
    ]);

    let area = proc_info_popup_area(70, frame.area());

    let l = List::new(items).block(black_title_block(Title::from(
        proc.name.clone().unwrap_or(String::from("no proc name")),
    )));

    frame.render_widget(Clear, area);
    frame.render_widget(l, area);
}

fn render_filter(frame: &mut Frame, chunk: Rect, state: &mut State) {
    let filter_block = Block::default().borders(Borders::BOTTOM | Borders::LEFT);

    let filter_color = match state.current_screen {
        CurrentScreen::Filter => Color::LightMagenta,
        _ => Color::White,
    };

    let filter_paragraph = Paragraph::new(Text::styled(
        String::from("f :> ") + &state.filter.clone(),
        Style::default().fg(filter_color),
    ))
    .left_aligned()
    .block(filter_block);

    frame.render_widget(filter_paragraph, chunk);
}

fn render_sysinfo(frame: &mut Frame, state: &mut State) {
    let area = responsive_area(frame.area());

    frame.render_widget(Clear, area);

    let hsplit = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Fill(1)])
        .split(area);
    let bottom_vsplit = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(hsplit[1]);

    render_sysinfo_info(frame, hsplit[0], state);

    render_sysinfo_cpu(frame, bottom_vsplit[0], state);

    render_sysinfo_mem(frame, bottom_vsplit[1], state);
}

fn render_sysinfo_info(frame: &mut Frame, chunk: Rect, state: &mut State) {
    let pad_len = 15;

    let mut os = state
        .info
        .long_os_version
        .clone()
        .unwrap_or(String::from("unknown"));
    let mut os_key = String::from("os");
    right_pad(&mut os_key, pad_len);
    let os_key = os_key + ": ";
    os.insert_str(0, &os_key);

    let mut kernel = state.info.kernel_long_version.clone();
    let mut kernel_key = String::from("kernel");
    right_pad(&mut kernel_key, pad_len);
    let kernel_key = kernel_key + ": ";
    kernel.insert_str(0, &kernel_key);

    let mut hostname = state
        .info
        .host_name
        .clone()
        .unwrap_or(String::from("unknown"));
    let mut hostname_key = String::from("hostname");
    right_pad(&mut hostname_key, pad_len);
    let hostname_key = hostname_key + ": ";
    hostname.insert_str(0, &hostname_key);

    let mut cpu_arch = state.info.cpu_arch.clone();
    let mut cpu_arch_key = String::from("architecture");
    right_pad(&mut cpu_arch_key, pad_len);
    let cpu_arch_key = cpu_arch_key + ": ";
    cpu_arch.insert_str(0, &cpu_arch_key);

    let mut physical_core_count = state.info.physical_core_count.unwrap_or(0).to_string();
    let mut physical_core_count_key = String::from("physical cores");
    right_pad(&mut physical_core_count_key, pad_len);
    let physical_core_count_key = physical_core_count_key + ": ";
    physical_core_count.insert_str(0, &physical_core_count_key);

    let items: Vec<ListItem> = Vec::from([
        ListItem::from(Text::raw(os).left_aligned()),
        ListItem::from(Text::raw(kernel).left_aligned()),
        ListItem::from(Text::raw(hostname).left_aligned()),
        ListItem::from(Text::raw(cpu_arch).left_aligned()),
        ListItem::from(Text::raw(physical_core_count).left_aligned()),
    ]);

    let l = List::new(items).block(black_title_block(Title::from("system info")));

    frame.render_widget(l, chunk);
}

fn render_sysinfo_cpu(frame: &mut Frame, chunk: Rect, state: &mut State) {
    let items: Vec<ListItem> = state
        .cpus
        .iter()
        .map(|cpu| {
            let mut cpu_name = cpu.name.clone();
            right_pad(&mut cpu_name, 8);
            ListItem::from(Text::raw(format!("{}: {:.2}%", cpu_name, cpu.usage)).left_aligned())
        })
        .collect();

    let l = List::new(items).block(black_title_block(Title::from("cpu")));
    frame.render_widget(l, chunk);
}

fn render_sysinfo_mem(frame: &mut Frame, chunk: Rect, state: &mut State) {
    let pad_len = 11;

    let mut total = bytes_to_str(state.ram.total);
    let mut total_key = String::from("total");
    right_pad(&mut total_key, pad_len);
    let total_key = total_key + ": ";
    total.insert_str(0, &total_key);

    let mut free = bytes_to_str(state.ram.free);
    let mut free_key = String::from("free");
    right_pad(&mut free_key, pad_len);
    let free_key = free_key + ": ";
    free.insert_str(0, &free_key);

    let mut available = bytes_to_str(state.ram.available);
    let mut available_key = String::from("available");
    right_pad(&mut available_key, pad_len);
    let available_key = available_key + ": ";
    available.insert_str(0, &available_key);

    let mut used = bytes_to_str(state.ram.used);
    let mut used_key = String::from("used");
    right_pad(&mut used_key, pad_len);
    let used_key = used_key + ": ";
    used.insert_str(0, &used_key);

    let mut total_swap = bytes_to_str(state.ram.total_swap);
    let mut total_swap_key = String::from("total_swap");
    right_pad(&mut total_swap_key, pad_len);
    let total_swap_key = total_swap_key + ": ";
    total_swap.insert_str(0, &total_swap_key);

    let mut free_swap = bytes_to_str(state.ram.free_swap);
    let mut free_swap_key = String::from("free_swap");
    right_pad(&mut free_swap_key, pad_len);
    let free_swap_key = free_swap_key + ": ";
    free_swap.insert_str(0, &free_swap_key);

    let mut used_swap = bytes_to_str(state.ram.used_swap);
    let mut used_swap_key = String::from("used_swap");
    right_pad(&mut used_swap_key, pad_len);
    let used_swap_key = used_swap_key + ": ";
    used_swap.insert_str(0, &used_swap_key);

    let mem_list_items: Vec<ListItem> = Vec::from([
        ListItem::from(Text::raw(total).left_aligned()),
        ListItem::from(Text::raw(free).left_aligned()),
        ListItem::from(Text::raw(available).left_aligned()),
        ListItem::from(Text::raw(used).left_aligned()),
        ListItem::from(Text::raw(total_swap).left_aligned()),
        ListItem::from(Text::raw(free_swap).left_aligned()),
        ListItem::from(Text::raw(used_swap).left_aligned()),
    ]);

    let l = List::new(mem_list_items).block(black_title_block(Title::from("memory")));

    frame.render_widget(l, chunk);
}

fn render_help(frame: &mut Frame) {
    let area = responsive_area(frame.area());

    frame.render_widget(Clear, area);

    let vsplit = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let right_hsplit = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vsplit[1]);

    let main_mode_keybinds = Paragraph::new(Text::raw(
        "[q] (q)uit
[j] down
[k] up
[g] first process
[G] last process
[s] change (s)ort (s)trategy {i.e. cpu usage, name, memory, etc.}
[K] (K)ill process
[i] system (i)nfo
[d] currently selected process info
[h] toggle (h)elp
[f] toggle (f)ilter
[Backspace] clear filter
",
    ))
    .block(black_title_block(Title::from("main mode keybinds")));

    frame.render_widget(main_mode_keybinds, vsplit[0]);

    let filter_mode_keybinds = Paragraph::new(Text::raw(
        "[Backspace] delete from filter
[Esc] return to main mode
[Enter] return to main mode
all other keys filter processes
",
    ))
    .block(black_title_block(Title::from("filter mode keybinds")));

    frame.render_widget(filter_mode_keybinds, right_hsplit[0]);

    let popup_keybinds = Paragraph::new(Text::raw(
        "[i] return to main mode from system (i)nfo
[d] return to main mode from selected process info
[h] return to main mode from (h)elp menu
[Esc] return to main mode from any popup
",
    ))
    .block(black_title_block(Title::from("popup keybinds")));

    frame.render_widget(popup_keybinds, right_hsplit[1]);
}

fn render_killconfirm(frame: &mut Frame, state: &mut State) {
    let proc_idx = state
        .processes
        .iter()
        .position(|p| p.pid == state.current_pid_watch.expect("no pid watching"));

    let proc_idx = match proc_idx {
        Some(x) => x,
        None => {
            // if we can't find the process then we don't render this
            // anymore
            state.current_pid_watch = None;
            state.current_screen = CurrentScreen::Main;
            return;
        }
    };

    let proc = &state.processes[proc_idx];

    let killconfirm_text = Paragraph::new(Text::raw(format!(
        "Are you sure you want to kill pid {} {}\npress [y]es / [n]o",
        proc.pid,
        proc.name.clone().unwrap_or(String::from("no proc name")),
    )))
    .centered()
    .block(black_title_block(Title::from("kill confirm")));

    let area = kill_confirm_popup_area(frame.area());

    frame.render_widget(Clear, area);
    frame.render_widget(killconfirm_text, area);
}

fn render_debug(
    frame: &mut Frame,
    state: &State,
    title: Duration,
    filter: Duration,
    proc_list: Duration,
    popup: Duration,
    chunk: Rect,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);

    let items: Vec<ListItem> = Vec::from([
        ListItem::from(format!(
            "sys collect: {:?}",
            state.debug_stats_sys.unwrap_or(Instant::now().elapsed())
        )),
        ListItem::from(format!(
            "state collect: {:?}",
            state.debug_stats_state.unwrap_or(Instant::now().elapsed())
        )),
        ListItem::from(format!(
            "ram collect: {:?}",
            state.debug_stats_ram.unwrap_or(Instant::now().elapsed())
        )),
        ListItem::from(format!(
            "cpu collect: {:?}",
            state.debug_stats_cpu.unwrap_or(Instant::now().elapsed())
        )),
    ]);

    let l = List::new(items);
    frame.render_widget(l, layout[0]);

    let items: Vec<ListItem> = Vec::from([
        ListItem::from(format!("title render: {:?}", title)),
        ListItem::from(format!("filter render: {:?}", filter)),
        ListItem::from(format!("proc list render: {:?}", proc_list)),
        ListItem::from(format!("popup render: {:?}", popup)),
    ]);

    let l = List::new(items);
    frame.render_widget(l, layout[1]);
}

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

/// helper function, similar to `centered_rect` from ratatui json editor tutorial, but has a
/// constant y value since we need our signal_menu and other stuff rendered there to take a
/// constant amount of space on the y axis
fn proc_info_popup_area(percent_x: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(13),
            Constraint::Fill(1),
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

/// helper function, similar to `centered_rect` from ratatui json editor tutorial, but has a
/// constant y value since we need our kill confirm menu there to take a
/// constant amount of space on the y axis
fn kill_confirm_popup_area(r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(100),
            Constraint::Fill(1),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

/// helper function to create black title block
fn black_title_block(title: Title) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().bg(Color::Black))
}

/// converts bytes to a string with the corresponding type
/// # Assumptions
/// - 1024 bytes in a kilobyte
/// - 1024 kilobytes in a megabyte
/// - 1024 megabytes in a gigabyte
/// - does not go past gigabytes
/// # Examples
/// ```rust
/// let b: u64 = 0;
/// let s: String = bytes_to_str(b); // "0 b"
/// ```
///
/// ```rust
/// let b: u64 = 1024;
/// let s: String = bytes_to_str(b); // "1 kb"
/// ```
///
fn bytes_to_str(bytes: u64) -> String {
    let kb: f64 = bytes as f64 / 1024.0;
    if kb < 1.0 {
        return bytes.to_string() + " b";
    }

    let mb: f64 = kb / 1024.0;
    if mb < 1.0 {
        return format!("{:.2} {}", kb, "kb");
    }

    let gb: f64 = mb / 1024.0;
    if gb < 1.0 {
        return format!("{:.2} {}", mb, "mb");
    }

    format!("{:.2} {}", gb, "gb")
}

/// simple in-place right pad for Strings
///
/// # Examples
/// ```rust
/// let mut s = String::from("hi");
/// right_pad(s, 5); // "hi   "
/// s.len(); // 5
/// ```
fn right_pad(s: &mut String, total_len: usize) {
    if s.len() >= total_len {
        return;
    }

    s.insert_str(s.len(), &" ".to_string().repeat(total_len - s.len()));
}

/// returns a centered rect based on screen size for popups
///
/// # Examples
/// ```rust
/// let area = responsive_area(frame.area());
/// frame.render_widget(widget, area);
/// ```
fn responsive_area(chunk: Rect) -> Rect {
    if chunk.width < 65 || chunk.height < 40 {
        centered_rect(90, 90, chunk)
    } else {
        centered_rect(50, 50, chunk)
    }
}
