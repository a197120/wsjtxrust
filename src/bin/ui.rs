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


//     f.render_widget(
//     List::new(format!(
//       "
//         Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
//         Designated Callsigns: {}
//       ",
//       appstate.designated_callsigns.join(", ")
//     ))
//     .block(
//       Block::default()
//         .title("WSJTX Message Server")
//         .title_alignment(Alignment::Center)
//         .borders(Borders::ALL)
//         .border_type(BorderType::Rounded),
//     )
//     .style(Style::default().fg(Color::Yellow))
//     .alignment(Alignment::Center),
//     chunks[2],
//   );
  let items: Vec<ListItem> = appstate.decode_strings.iter().map(|i| ListItem::new(i.clone())).collect();

  f.render_widget(
    List::new(items)
    .block(
        Block::default()
            .title("WSJTX Message Server")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Yellow)),
    chunks[1],
);
}

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