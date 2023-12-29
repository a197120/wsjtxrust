use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, ListState, Block, BorderType, Borders, Paragraph, List, ListItem},
};

use crate::appstate::AppState;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn select(&mut self, i: usize) {
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub fn create_status_paragraph(appstate: &AppState) -> Paragraph {
    let parts: Vec<&str> = appstate.status_string.split_whitespace().collect();
    let mut vec_of_spans: Vec<Span> = Vec::new();

    for part in parts {
        let mut style = Style::new();
        if part == "true" || part == "true," {
            style = style.bg(Color::Green);
            vec_of_spans.push(Span::styled(" X ", style));
        } else if part == "false" || part == "false," {
            style = style.bg(Color::Red);
            vec_of_spans.push(Span::styled(" X ", style));

        }else {
            vec_of_spans.push(Span::styled(part, style));
            vec_of_spans.push(Span::raw(" "));
        }
    }
    let line = Line::from(vec_of_spans);
    let text = Text::from(vec![line]);

    Paragraph::new(text)
        .style(Style::default().fg(Color::White))
}

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

    


    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[1], &mut appstate.list_state.state);   

    let status_paragraph = create_status_paragraph(appstate);
    let top_panel = status_paragraph
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Client Info"));
    f.render_widget(top_panel, chunks[0]);
    // WORKING CODE FOR BOTTOM PANEL

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