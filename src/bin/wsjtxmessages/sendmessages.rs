use super::*;

pub fn add_string_to_payload(payload: &mut Vec<u8>, string: &str) {
    let len = string.len() as u32;
    let mut len_bytes = [0u8; 4];
    BigEndian::write_u32(&mut len_bytes, len);
    payload.extend_from_slice(&len_bytes);
    payload.extend_from_slice(string.as_bytes());
}

pub fn add_f64_to_payload(payload: &mut Vec<u8>, value: f64) {
    let mut bytes = [0u8; 8];
    BigEndian::write_f64(&mut bytes, value);
    payload.extend_from_slice(&bytes);
}

pub fn add_i32_to_payload(payload: &mut Vec<u8>, value: i32) {
    let mut bytes = [0u8; 4];
    BigEndian::write_i32(&mut bytes, value);
    payload.extend_from_slice(&bytes);
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

// pub fn add_naive_time_to_payload(payload: &mut Vec<u8>, time: NaiveTime) {
//     let total_seconds = time.num_seconds_from_midnight();
//     let milliseconds = time.nanosecond() / 1_000_000;
//     let total_milliseconds = (total_seconds * 1000) + milliseconds as u32;
//     add_u32_to_payload(payload, total_milliseconds);
// }

pub fn add_naive_time_to_payload(payload: &mut Vec<u8>, time: NaiveTime) {
    let total_seconds = time.num_seconds_from_midnight();
    let nanoseconds_within_last_second = time.nanosecond();
    let total_milliseconds = total_seconds * 1000 + nanoseconds_within_last_second / 1_000_000;
    add_u32_to_payload(payload, total_milliseconds as u32);
}

pub fn encode_message(encoded_message: Vec<u8>) -> Vec<u8> {
    let mut payload: Vec<u8> = Vec::new();
    add_u32_to_payload(&mut payload, 0xadbccbda,);
    add_u32_to_payload(&mut payload, 2,);
    payload.extend(encoded_message);
    
    payload
}

pub fn encode_heartbeat(heartbeat: &Heartbeat) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, heartbeat.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &heartbeat.id);

    // Add maximum_schema_number to payload
    add_u32_to_payload(&mut payload, heartbeat.maximum_schema_number);

    // Add version to payload
    add_string_to_payload(&mut payload, &heartbeat.version);

    // Add revision to payload
    add_string_to_payload(&mut payload, &heartbeat.revision);
    payload
}

pub fn encode_clear(clear: &Clear) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, clear.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &clear.id);

    //add window to the payload
    add_u8_to_payload(&mut payload, clear.window);
    payload
}

pub fn encode_reply(reply: &Reply) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, reply.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &reply.id);

    // add QTime to payload
    add_naive_time_to_payload(&mut payload, reply.time);

    //add snr to payload
    add_i32_to_payload(&mut payload, reply.snr);

    //add dt to payload
    add_f64_to_payload(&mut payload, reply.delta_time_s);

    //add df to payload
    add_u32_to_payload(&mut payload, reply.delta_frequency_hz);

    //add mode to payload
    add_string_to_payload(&mut payload, &reply.mode);

    //add message to payload
    add_string_to_payload(&mut payload, &reply.message);

    //add low_confidence to payload
    add_bool_to_payload(&mut payload, reply.low_confidence);
    
    //add modifiers to payload
    add_u8_to_payload(&mut payload, reply.modifiers);

    payload
}


pub fn encode_close(close: &Close) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, close.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &close.id);

    payload
}

pub fn encode_replay(replay: &Replay) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, replay.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &replay.id);
    payload
}

pub fn encode_halt_tx(halt_tx: &HaltTx) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, halt_tx.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &halt_tx.id);

    //add auto_tx_only to payload
    add_bool_to_payload(&mut payload, halt_tx.auto_tx_only);

    payload
}

pub fn encode_free_text(free_text: &FreeText) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, free_text.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &free_text.id);

    // Add text to payload
    add_string_to_payload(&mut payload, &free_text.text);

    // Add send to payload
    add_bool_to_payload(&mut payload, free_text.send);
    payload
}


pub fn encode_location(location: &Location) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, location.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &location.id);

    // Add location to payload
    add_string_to_payload(&mut payload, &location.location);
    payload
}

pub fn encode_highlight_callsign_in(highlight_callsign_in: &HighlightCallsignIn) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, highlight_callsign_in.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &highlight_callsign_in.id);

    // Add callsign to payload
    add_string_to_payload(&mut payload, &highlight_callsign_in.callsign);

    // Add background color to payload
    add_string_to_payload(&mut payload, &highlight_callsign_in.background_color);

    // Add foreground color to payloud 
    add_string_to_payload(&mut payload, &highlight_callsign_in.foreground_color);

    // Add highlight last to payload
    add_bool_to_payload(&mut payload, highlight_callsign_in.highlight_last);
    payload
}


pub fn encode_switch_configuration(switch_configuration: &SwitchConfiguration) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, switch_configuration.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &switch_configuration.id);

    // Add configuration_name to payload
    add_string_to_payload(&mut payload, &switch_configuration.configuration_name);
    payload
}

pub fn encode_configure(configure: &Configure) -> Vec<u8> {
    let mut payload = Vec::new();

    // Add message_type to payload
    add_u32_to_payload(&mut payload, configure.message_type);

    // Add id to payload
    add_string_to_payload(&mut payload, &configure.id);

    // Add mode to payload
    add_string_to_payload(&mut payload, &configure.mode);

    // Add frequency_tolerance to payload
    add_u32_to_payload(&mut payload, configure.frequency_tolerance);

    // Add submode to payload
    add_string_to_payload(&mut payload, &configure.submode);

    // add fast_mode to payload
    add_bool_to_payload(&mut payload, configure.fast_mode);

    // add tr period to payload 
    add_i32_to_payload(&mut payload, configure.tr_period);

    // add rx df to payload
    add_i32_to_payload(&mut payload, configure.rx_df);

    // add dx call to payload
    add_string_to_payload(&mut payload, &configure.dx_call);

    // add dx grid to payload  
    add_string_to_payload(&mut payload, &configure.dx_grid);

    // add generate messages to pay load
    add_bool_to_payload(&mut payload, configure.generate_messages);
    payload
}