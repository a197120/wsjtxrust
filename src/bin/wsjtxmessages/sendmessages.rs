// // ALL CODE HERE IS EXPERIMENTAL AND NOT TESTED YET
// use byteorder::{BigEndian, WriteBytesExt};

// fn write_string_to_payload(payload: &mut Vec<u8>, s: &str) {
//     let bytes = s.as_bytes();
//     payload.write_u32::<BigEndian>(bytes.len() as u32).unwrap();
//     payload.extend_from_slice(bytes);
// }

// fn write_u64_to_payload(payload: &mut Vec<u8>, value: u64) {
//     payload.write_u64::<BigEndian>(value).unwrap();
// }

// fn write_u32_to_payload(payload: &mut Vec<u8>, value: u32) {
//     payload.write_u32::<BigEndian>(value).unwrap();
// }

// fn write_bool_to_payload(payload: &mut Vec<u8>, value: bool) {
//     payload.push(value as u8);
// }

// fn write_u8_to_payload(payload: &mut Vec<u8>, value: u8) {
//     payload.push(value);
// }

// fn write_i32_to_payload(payload: &mut Vec<u8>, value: i32) {
//     payload.write_i32::<BigEndian>(value).unwrap();
// }

// fn write_time_as_milliseconds_since_midnight(payload: &mut Vec<u8>, time: NaiveTime) {
//     let total_seconds = time.num_seconds_from_midnight();
//     let milliseconds = time.nanosecond() / 1_000_000;
//     let total_milliseconds = total_seconds * 1000 + milliseconds as u64;
//     payload.write_u32::<BigEndian>(total_milliseconds as u32).unwrap();
// }

// fn write_f64_to_payload(payload: &mut Vec<u8>, value: f64) {
//     payload.write_f64::<BigEndian>(value).unwrap();
// }

// pub fn encode_highlight_callsign_in(highlightcallsignin: &HighlightCallsignIn) -> Vec<u8> {
//     use byteorder::{BigEndian, WriteBytesExt};
//     let mut payload = Vec::new();

//     // Helper function to write a string to the payload.
//     let mut write_string = |s: &str| {
//         let bytes = s.as_bytes();
//         payload.write_u32::<BigEndian>(bytes.len() as u32).unwrap();
//         payload.extend_from_slice(bytes);
//     };

//     // Convert each field to bytes and append them to the payload.
//     write_string(&highlightcallsignin.id);
//     write_string(&highlightcallsignin.callsign);
//     write_string(&highlightcallsignin.background_color);
//     write_string(&highlightcallsignin.foreground_color);
//     payload.push(highlightcallsignin.highlight_last as u8);

//     payload
// }

// fn send_data(data: &[u8], src: SocketAddr, socket: &UdpSocket) {
//     // Your existing code...

//     // Create a new message.
//     let message = Message {
//         // Fill in your message fields here...
//     };

//     // Serialize the message to bytes.
//     let message_bytes = bincode::serialize(&message).expect("Failed to serialize message");

//     // Send the message back to the client.
//     socket.send_to(&message_bytes, src).expect("Failed to send message");
// }