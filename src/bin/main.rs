pub mod wsjtxmessages;
pub mod appstate;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;
use std::net::{UdpSocket, SocketAddr};
use std::io;
use simplelog::*;
use log::{info, LevelFilter};
use std::fs::File;
// use colored::*;
// use std::str;
pub use wsjtxmessages::*;
pub use wsjtxmessages::receivemessages::*;
pub use wsjtxmessages::sendmessages::*;
pub use appstate::*;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

const DEBUG: bool = false;



fn main() -> Result<()> {
    let log_file = File::create("wsjtxmessages.log").unwrap();
    let mut appstate = AppState::new().expect("Could not read callsigns");
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();
    info!("Starting WSJTX Message Server");
    info!("Designated Callsigns: {:?}", appstate.designated_callsigns);
    //uncomment below line for windows 
    //set_virtual_terminal(true).unwrap();
    info!("{}","WSJTX Message Server");

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(25);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let socket = UdpSocket::bind("127.0.0.1:2237").expect("Could not bind socket");
    socket.set_nonblocking(true).expect("Failed to enter non-blocking mode");
    while !appstate.should_quit {
        //render UI
        tui.draw(&mut appstate)?; 
        //Handle Events
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut appstate, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };

        let mut buffer = [0u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                if DEBUG {
                    info!("Received {} bytes from: {}", size, src);
                }

                handle_incoming_data(&buffer[..size], &mut appstate);

                // let close = Close {
                //     message_type: 6,
                //     id: "WSJT-X".to_string(),
                // };
                // let clear: Clear = Clear {
                //     message_type: 3,
                //     id: "rustyserv".to_string(),
                //     window: 2,
                // };
                // let free_text = FreeText {
                //     message_type: 9,
                //     id: "rustyserv".to_string(),
                //     text: "SUCCESSS!!!".to_string(),
                //     send: true,
                // };
                // let encoded_close = encode_close(&close);
                // let encoded_close = encode_message(encoded_close);
                // send_encoded_message(&socket, encoded_close, src).expect("Failed to send Close message");
                // let encoded_clear = encode_clear(&clear);
                // let encoded_clear = encode_message(encoded_clear);

                // send_encoded_message(&socket, encoded_clear, src).expect("Failed to send Clear message");
                // let encoded_free_text = encode_free_text(&free_text);
                // let encoded_free_text = encode_message(encoded_free_text);
                // send_encoded_message(&socket, encoded_free_text, src).expect("Failed to send Free Text message");
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // there's no packet, continue the loop
            },
            Err(e) => info!("Couldn't receive a datagram: {}", e),
        }

    }
    tui.exit()?;
    Ok(())
}

