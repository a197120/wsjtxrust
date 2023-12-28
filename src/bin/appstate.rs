use std::fs;
use std::sync::Arc;
use std::io::{self, BufRead};
use std::path::Path;
pub struct AppState {
    pub designated_callsigns: Arc<Vec<String>>,
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
        Ok(Self { designated_callsigns })
    }
}