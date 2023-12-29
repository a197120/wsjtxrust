use std::fs;
use std::sync::Arc;
use std::io::{self, BufRead};
use std::path::Path;
use std::default::Default;
use ratatui::widgets::ListState;
use crate::ui::StatefulList;


pub struct AppState {
    pub designated_callsigns: Arc<Vec<String>>,
    pub should_quit: bool,
    pub decode_strings: Vec<String>,
    pub status_string: String,  
    pub list_state: StatefulList<String>,
    pub should_redraw: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            designated_callsigns: Arc::new(Vec::new()),
            should_quit: false,
            decode_strings: Vec::new(),
            status_string: String::new(),
            list_state: StatefulList::with_items(vec![]),
            should_redraw: true,
        }
    }
}
impl AppState {
    pub fn new() -> io::Result<Self> {
        let callsigns_path = Path::new("callsigns.txt");
        let callsigns_file = fs::File::open(&callsigns_path)?;
        let callsigns_reader = io::BufReader::new(callsigns_file);
        let designated_callsigns = Arc::new(
            callsigns_reader
                .lines()
                .filter_map(io::Result::ok)
                .collect()
        );
        Ok(Self { designated_callsigns, ..Self::default() })
    }
    pub fn tick(&self) {}
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

