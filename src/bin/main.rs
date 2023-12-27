pub mod wsjtxmessages;
pub mod appstate;
use std::net::{UdpSocket, SocketAddr};
use std::io;
use std::str::EncodeUtf16;
use colored::*;
// use std::str;
pub use wsjtxmessages::*;
pub use wsjtxmessages::receivemessages::*;
pub use wsjtxmessages::sendmessages::*;
pub use appstate::*;


const DEBUG: bool = false;



fn main() {
    let app_state = AppState::new().expect("Could not read callsigns");
    println!("Designated Callsigns: {:?}", app_state.designated_callsigns);
    //uncomment below line for windows 
    //set_virtual_terminal(true).unwrap();
    println!("{}","WSJTX Message Server".green().bold());
    let socket = UdpSocket::bind("127.0.0.1:2237").expect("Could not bind socket");
    loop {
        let mut buffer = [0u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                if DEBUG {
                    println!("Received {} bytes from: {}", size, src);
                }
                handle_incoming_data(&buffer[..size], &app_state);

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
            Err(e) => eprintln!("Couldn't receive a datagram: {}", e),
        }
    }
}

