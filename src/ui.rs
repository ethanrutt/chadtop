use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::Title, Block, Borders, Cell, Clear, HighlightSpacing, List, ListItem, Paragraph,
        Row, Table,
    },
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
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Percentage(90),
        ])
        .split(chunks[1]);

    let filter_block = Block::default().borders(Borders::BOTTOM);

    let filter_color = match state.current_screen {
        CurrentScreen::Filter => Color::LightRed,
        _ => Color::White,
    };

    let filter_paragraph = Paragraph::new(Text::styled(
        String::from("f :> ") + &state.filter.clone(),
        Style::default().fg(filter_color),
    ))
    .left_aligned()
    .block(filter_block);

    frame.render_widget(filter_paragraph, body_chunks[1]);

    render_proc_list(frame, body_chunks[2], state);

    match state.current_screen {
        CurrentScreen::ProcInfo => render_proc_info_popup(frame, state),
        CurrentScreen::SysInfo => {
            // SysInfo
            // cpu mem
            let area = centered_rect(50, 50, frame.area());
            frame.render_widget(Clear, area);

            let hsplit = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(7), Constraint::Fill(1)])
                .split(area);
            let bottom_vsplit = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(hsplit[1]);

            let mut os = state
                .info
                .long_os_version
                .clone()
                .unwrap_or(String::from("unknown"));
            os.insert_str(0, "os: ");

            let mut kernel = state.info.kernel_long_version.clone();
            kernel.insert_str(0, "kernel: ");

            let mut hostname = state
                .info
                .host_name
                .clone()
                .unwrap_or(String::from("unknown"));
            hostname.insert_str(0, "hostname: ");

            let mut cpu_arch = state.info.cpu_arch.clone();
            cpu_arch.insert_str(0, "architecture: ");

            let mut physical_core_count = state.info.physical_core_count.unwrap_or(0).to_string();
            physical_core_count.insert_str(0, "physical cores: ");

            let items: Vec<ListItem> = Vec::from([
                ListItem::from(Text::raw(os).centered()),
                ListItem::from(Text::raw(kernel).centered()),
                ListItem::from(Text::raw(hostname).centered()),
                ListItem::from(Text::raw(cpu_arch).centered()),
                ListItem::from(Text::raw(physical_core_count).centered()),
            ]);

            let l = List::new(items).block(black_title_block(Title::from("system info")));

            frame.render_widget(l, hsplit[0]);

            let items: Vec<ListItem> = state
                .cpus
                .iter()
                .map(|cpu| {
                    ListItem::from(
                        Text::raw(cpu.name.clone() + ": " + &cpu.usage.to_string()).centered(),
                    )
                })
                .collect();

            let l = List::new(items).block(black_title_block(Title::from("cpu")));
            frame.render_widget(l, bottom_vsplit[0]);

            let mut total = state.ram.total.to_string();
            total.insert_str(0, "total: ");

            let mut free = state.ram.free.to_string();
            free.insert_str(0, "free: ");

            let mut available = state.ram.available.to_string();
            available.insert_str(0, "available: ");

            let mut used = state.ram.used.to_string();
            used.insert_str(0, "used: ");

            let mut total_swap = state.ram.total_swap.to_string();
            total_swap.insert_str(0, "total_swap: ");

            let mut free_swap = state.ram.free_swap.to_string();
            free_swap.insert_str(0, "free_swap: ");

            let mut used_swap = state.ram.used_swap.to_string();
            used_swap.insert_str(0, "used_swap: ");

            let mem_list_items: Vec<ListItem> = Vec::from([
                ListItem::from(Text::raw(total).centered()),
                ListItem::from(Text::raw(free).centered()),
                ListItem::from(Text::raw(available).centered()),
                ListItem::from(Text::raw(used).centered()),
                ListItem::from(Text::raw(total_swap).centered()),
                ListItem::from(Text::raw(free_swap).centered()),
                ListItem::from(Text::raw(used_swap).centered()),
            ]);

            let l = List::new(mem_list_items).block(black_title_block(Title::from("memory")));

            frame.render_widget(l, bottom_vsplit[1]);
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
            Constraint::Percentage(7),
            Constraint::Percentage(13),
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
(s) cycle next sort strategy | (d) more proc info",
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

    let process_table_header = ["pid", "name", "memory", "cpu usage", "user", "ppid"]
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
    .row_highlight_style(selected_row_style)
    .column_highlight_style(selected_col_style)
    .cell_highlight_style(selected_cell_style)
    .highlight_symbol(Text::from(vec![" > ".into()]))
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

    let mut pid = proc.pid.to_string();
    pid.insert_str(0, "pid: ");

    let mut start_time = proc.start_time.to_string();
    start_time.insert_str(0, "start time: ");

    let mut run_time = proc.run_time.to_string();
    run_time.insert_str(0, "run time: ");

    let disk_usage_read = proc.disk_usage_read / 1000000;
    let mut disk_usage_read = disk_usage_read.to_string() + " mb";
    disk_usage_read.insert_str(0, "disk read: ");

    let disk_usage_written = proc.disk_usage_written / 1000000;
    let mut disk_usage_written = disk_usage_written.to_string() + " mb";
    disk_usage_written.insert_str(0, "disk written: ");

    let mut open_files = proc.open_files.unwrap_or(0).to_string();
    open_files.insert_str(0, "open files: ");

    let mut open_files_limit = proc.open_files_limit.unwrap_or(0).to_string();
    open_files_limit.insert_str(0, "open files limit: ");

    let mut cwd = proc.cwd.clone().unwrap_or(String::from("n/a"));
    cwd.insert_str(0, "cwd: ");

    let mut exe = proc.exe.clone().unwrap_or(String::from("n/a"));
    exe.insert_str(0, "exe: ");

    let mut cmd = proc.cmd.clone().unwrap_or(String::from("n/a"));
    cmd.insert_str(0, "cmd: ");

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

    let area = signal_menu_rect(70, frame.area());

    let l = List::new(items).block(black_title_block(Title::from(
        proc.name.clone().unwrap_or(String::from("no proc name")),
    )));

    frame.render_widget(Clear, area);
    frame.render_widget(l, area);
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
fn signal_menu_rect(percent_x: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(12),
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

/// FIXME: maybe overload so we can also do it with &str like
/// black_title_block("memory");
/// helper function to create black title block
fn black_title_block(title: Title) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(Style::default().fg(Color::LightBlue))
        .style(Style::default().bg(Color::Black))
}
