pub mod wsjtxmessages;
use std::net::UdpSocket;
// use std::str;
use wsjtxmessages::*;

const debug: bool = false;

fn handle_data(data: &[u8]) {
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
    // println!("Magic Number: {:#x}", magic_number);
    // println!("Schema Number: {:x}", schema_number);
    // println!("Received message: {:?}", data);
    //println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    //get messagetype from the payload
    let (messagetype, payload) = payload.split_at(4);
    let messagetype = u32::from_be_bytes([messagetype[0], messagetype[1], messagetype[2], messagetype[3]]);
    // println!("Message Type: {:x}", messagetype); 
    match messagetype {
        0 => decode_heartbeat(payload, debug),
        1 => decode_status(payload, debug),
        2 => decode_decode(payload, true),
        3 => decode_clear(payload, debug),
        4 => decode_reply(payload, debug),
        5 => decode_logdata(payload, debug),
        6 => decode_close(payload, debug),
        7 => decode_replay(payload, debug),
        8 => decode_halt_tx(payload, debug),
        9 => decode_free_text(payload, debug),
        10 => decode_wspr_decode(payload, debug),
        11 => decode_location(payload, debug),
        12 => decode_logged_adif(payload, debug),
        13 => decode_highlight_callsign_in(payload, debug),
        14 => decode_switch_configuration(payload, debug),
        15 => decode_configure(payload, debug),
        _ => eprintln!("Unknown Message Type"),
    };
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:2237").expect("Could not bind socket");
    loop {
        let mut buffer = [0u8; 4096];
        match socket.recv_from(&mut buffer) {
            Ok((size, _src)) => handle_data(&buffer[..size]),
            Err(e) => eprintln!("Couldn't receive a datagram: {}", e),
        }
    }
}