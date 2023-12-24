use bincode;
use super::*;

pub fn add_string_to_payload(payload: &mut Vec<u8>, string: String) {
    let len = string.len() as u32;
    let mut len_bytes = [0u8; 4];
    BigEndian::write_u32(&mut len_bytes, len);
    payload.extend_from_slice(&len_bytes);
    payload.extend_from_slice(string.as_bytes());
}

pub fn add_u32_to_payload(payload: &mut Vec<u8>, value: u32) {
    let mut bytes = [0u8; 4];
    BigEndian::write_u32(&mut bytes, value);
    payload.extend_from_slice(&bytes);
}

pub fn add_u8_to_payload(payload: &mut Vec<u8>, value: u8) {
    payload.push(value);
}

pub fn add_bool_to_payload(payload: &mut Vec<u8>, value: bool) {
    payload.push(if value { 1 } else { 0 });
}

pub fn encode_message(encoded_message: Vec<u8>) -> Vec<u8> {
    let mut payload: Vec<u8> = Vec::new();
    add_u32_to_payload(&mut payload, 0xadbccbda,);
    add_u32_to_payload(&mut payload, 2,);
    payload.extend(encoded_message);
    
    payload



}
pub fn encode_clear(clear: &Clear) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, clear.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, clear.id.clone());

    //add window to the payload
    add_u8_to_payload(&mut payload, clear.window);
    payload
}
pub fn encode_close(close: &Close) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, close.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, close.id.clone());

    payload
}

pub fn encode_free_text(free_text: &FreeText) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, free_text.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, free_text.id.clone());

    // Add text to payload
    add_string_to_payload(&mut payload, free_text.text.clone());

    // Add send to payload
    add_bool_to_payload(&mut payload, free_text.send);
    payload
}

