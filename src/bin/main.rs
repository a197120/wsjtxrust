pub mod wsjtxmessages;
use std::net::{UdpSocket, SocketAddr};
use std::io;
use std::str::EncodeUtf16;
// use std::str;
pub use wsjtxmessages::*;
pub use wsjtxmessages::receivemessages::*;
pub use wsjtxmessages::sendmessages::*;

const DEBUG: bool = false;

pub fn send_encoded_message(socket: &UdpSocket, message: Vec<u8>, address: SocketAddr) -> io::Result<()> {
    socket.send_to(&message, address)?;
    Ok(())
}
fn handle_incoming_data(data: &[u8]) {
    if data.len() < 8 {
        eprintln!("Data too short to be a valid message");
        return;
    }
    //split header from payload
    let (header, payload) = data.split_at(8);
    //get magic number and schema number from header
    let magic_number = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let schema_number = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
    // create a message object from the data
    let message = Message {
        magic_number,
        schema_number,
        payload: payload.to_vec(),
    };
    if DEBUG {
        println!("Message: {:?}", message);
    }
    // println!("Magic Number: {:#x}", magic_number);
    // println!("Schema Number: {:x}", schema_number);
    // println!("Received message: {:?}", data);
    // println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    //get messagetype from the payload
    let messagetype = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
    // println!("Message Type: {:x}", messagetype); 
    match messagetype {
        0 => { decode_heartbeat(payload, DEBUG); }
        1 => { decode_status(payload, DEBUG); }
        2 => { decode_decode(payload, DEBUG).print_message() ; }
        3 => { decode_clear(payload, true); }
        4 => { decode_reply(payload, DEBUG); }
        5 => { decode_logdata(payload, DEBUG); }
        6 => { decode_close(payload, true); }
        7 => { decode_replay(payload, DEBUG); }
        8 => { decode_halt_tx(payload, DEBUG); }
        9 => { decode_free_text(payload, DEBUG); }
        10 => { decode_wspr_decode(payload, DEBUG); }
        11 => { decode_location(payload, DEBUG); }
        12 => { decode_logged_adif(payload, DEBUG); }
        13 => { decode_highlight_callsign_in(payload, DEBUG); }
        14 => { decode_switch_configuration(payload, DEBUG); }
        15 => { decode_configure(payload, DEBUG); }
        _ => eprintln!("Unknown Message Type"),
    };
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:2237").expect("Could not bind socket");
    loop {
        let mut buffer = [0u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                if DEBUG {
                    println!("Received {} bytes from: {}", size, src);
                }
                handle_incoming_data(&buffer[..size]);

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

