use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, Block, BorderType, Borders, Paragraph, List, ListItem},
};

use crate::appstate::AppState;

pub fn render(appstate: &mut AppState, f: &mut Frame) {
    // Create the layout sections.
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(f.size());

    let items: Vec<ListItem> = appstate.decode_strings.iter().map(|i| {
        let parts: Vec<&str> = i.split_whitespace().collect();
        let mut vec_of_spans: Vec<Span> = Vec::new();
        let mut make_green = false;
        for part in parts {
            let mut style = Style::new();
            if make_green {
                style = style.fg(Color::Green);
            } else if part.starts_with("-") {
                style = style.fg(Color::Red);
            } else if part.starts_with("+") {
                style = style.fg(Color::Green);
            } else if appstate.designated_callsigns.contains(&part.to_string()) {
                style = style.fg(Color::Black).bg(Color::White); // highlight the callsign
            }
            vec_of_spans.push(Span::styled(part, style));
            vec_of_spans.push(Span::raw(" "));
            if ["Country:", "State:", "City:"].contains(&part) {
                make_green = true;
            }
            if part.ends_with(",") {
                make_green = false;
            }
        }
        let line = Line::from(vec_of_spans);
        let text = Text::from(vec![line]);
        ListItem::new(text)
    }).collect();

    f.render_widget(
        List::new(items)
        .block(
            Block::default()
                .title("Decoded Messages")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::new().fg(Color::Gray)),
        chunks[1],
    );
    let top_panel = Paragraph::new(appstate.status_string.clone())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Client Info"));
    f.render_widget(top_panel, chunks[0]);

    let bottom_panel = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(bottom_panel, chunks[2]);


    /// helper function to create a centered rect using up certain percentage of the available rect `r`
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
}