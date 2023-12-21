use super::*;
use byteorder::{ByteOrder, BigEndian};

fn get_string_from_payload(payload: &[u8]) -> (String, &[u8]) {
    let (len_bytes, mut rest) = payload.split_at(4);
    let len = BigEndian::read_u32(len_bytes);
    let str;
    if len == 4294967295 {
        str = "n/a".to_string();
    } else if len == 0 {
        str = "empty".to_string(); 
    } else {
        let (str_bytes, rest_new) = rest.split_at(len as usize);
        rest = rest_new;
        str = String::from_utf8(str_bytes.to_vec()).unwrap();
    }
    (str, rest)
}

fn get_u64_from_payload(payload: &[u8]) -> (u64, &[u8]) {
    let (bytes, rest) = payload.split_at(8);
    let value = BigEndian::read_u64(bytes);
    (value, rest)
}
fn get_u32_from_payload(payload: &[u8]) -> (u32, &[u8]) {
    let (bytes, rest) = payload.split_at(4);
    let value = BigEndian::read_u32(bytes);
    (value, rest)
}
fn get_bool_from_payload(payload: &[u8]) -> (bool, &[u8]) {
    let (bytes, rest) = payload.split_at(1);
    let value = bytes[0] != 0;
    (value, rest)
}
fn get_u8_from_payload(payload: &[u8]) -> (u8, &[u8]) {
    let (bytes, rest) = payload.split_at(1);
    let value = bytes[0];
    (value, rest)
}
fn get_i32_from_payload(payload: &[u8]) -> (i32, &[u8]) {
    let (bytes, rest) = payload.split_at(4);
    let value = BigEndian::read_i32(bytes);
    (value, rest)
}
fn get_time_from_milliseconds_since_midnight(ms: u32) -> NaiveTime {
    let total_seconds = ms / 1000;
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    let milliseconds = ms % 1000;

    NaiveTime::from_hms_milli(hours, minutes, seconds, milliseconds)
}
fn get_f64_from_payload(payload: &[u8]) -> (f64, &[u8]) {
    let (bytes, rest) = payload.split_at(8);
    let value = BigEndian::read_f64(bytes);
    (value, rest)
}

pub fn decode_heartbeat(payload: &[u8], debug: bool) {
    println!("Heartbeat message");
    let (id, rest) = get_string_from_payload(payload);
    let (maximum_schema_number, rest) = get_u32_from_payload(rest);      
    let (version, rest) = get_string_from_payload(rest);
    let (revision, rest) = get_string_from_payload(rest);
    let heartbeat = Heartbeat {
        id,
        maximum_schema_number,
        version: version,
        revision: revision,
        };
    if debug {
        println!("Heartbeat: {}", heartbeat)
    }
}

pub fn decode_status(payload: &[u8], debug: bool) {
    println!("Status message");
    // println!("Payload: {:?}", payload);
    let (id, rest) = get_string_from_payload(payload);
    let (dial_frequency, rest) = get_u64_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (dx_call, rest) = get_string_from_payload(rest);
    let (report, rest) = get_string_from_payload(rest);
    let (tx_mode, rest) = get_string_from_payload(rest);
    let (tx_enabled, rest) = get_bool_from_payload(rest);
    let (transmitting, rest) = get_bool_from_payload(rest);
    let (decoding, rest) = get_bool_from_payload(rest);
    let (rx_df, rest) = get_u32_from_payload(rest);
    let (tx_df, rest) = get_u32_from_payload(rest);
    let (de_call, rest) = get_string_from_payload(rest);
    let (de_grid, rest) = get_string_from_payload(rest);
    let (dx_grid, rest) = get_string_from_payload(rest);
    let (tx_watchdog, rest) = get_bool_from_payload(rest);
    let (sub_mode, rest) = get_string_from_payload(rest);
    let (fast_mode, rest) = get_bool_from_payload(rest);
    let (special_operation_mode, rest) = get_u8_from_payload(rest);
    let (frequency_tolerance, rest) = get_u32_from_payload(rest);
    let (tr_period, rest) = get_u32_from_payload(rest);
    let (configuration_name, rest) = get_string_from_payload(rest);
    let (tx_message, rest) = get_string_from_payload(rest);

    let status = Status {
        id,
        dial_frequency,
        mode,
        dx_call,
        report,
        tx_mode,
        tx_enabled,
        transmitting,
        decoding,
        rx_df,
        tx_df,
        de_call,
        de_grid,
        dx_grid,
        tx_watchdog,
        sub_mode,
        fast_mode,
        special_operation_mode,
        frequency_tolerance,
        tr_period,
        configuration_name,
        tx_message,
        };
    if debug {
        println!("Status: {}", status);
    }
}

pub fn decode_decode(payload: &[u8], debug: bool) {
    println!("Decode message");
    let (id, rest) = get_string_from_payload(payload);
    let (new, rest) = get_bool_from_payload(rest);
    let (time, rest) = get_u32_from_payload(rest);
    let time = get_time_from_milliseconds_since_midnight(time);
    let (snr, rest) = get_i32_from_payload(rest);
    let (delta_time_s, rest) = get_f64_from_payload(rest);
    let (delta_frequency_hz, rest) = get_u32_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (message, rest) = get_string_from_payload(rest);
    let (low_confidence, rest) = get_bool_from_payload(rest);
    let (off_air, rest) = get_bool_from_payload(rest);
    let decode = Decode {
        id,
        new,
        time,
        snr,
        delta_time_s: delta_time_s as f64,
        delta_frequency_hz,
        mode,
        message,
        low_confidence,
        off_air,
    };
    if debug {
        println!("Decode: {}", decode);
    }
}

pub fn decode_clear(payload: &[u8], debug: bool){
    println!("Clear message");
    let (id, rest) = get_string_from_payload(payload);
    let (window, rest) = get_u8_from_payload(rest);
    let clear = Clear {
        id,
        window,
    };
    if debug {
        println!("Clear: {}", clear);
    }
}

pub fn decode_reply(payload: &[u8], debug: bool){
    println!("Reply message");
    let (id, rest) = get_string_from_payload(payload);
    let (time, rest) = get_u32_from_payload(rest);
    let time = get_time_from_milliseconds_since_midnight(time);
    let (snr, rest) = get_i32_from_payload(rest);
    let (delta_time_s, rest) = get_f64_from_payload(rest);
    let (delta_frequency_hz, rest) = get_u32_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (message, rest) = get_string_from_payload(rest);
    let (low_confidence, rest) = get_bool_from_payload(rest);
    let (modifiers, rest) = get_u8_from_payload(rest);
    let reply = Reply {
        id,
        time,
        snr,
        delta_time_s: delta_time_s as f64,
        delta_frequency_hz,
        mode,
        message,
        low_confidence,
        modifiers,
    };
    if debug {
        println!("Reply: {}", reply);
    }
}

pub fn decode_logdata(payload: &[u8], debug: bool){
    println!("Logged QSO!");
    let (id, rest) = get_string_from_payload(payload);
    let (date_time_off, rest) = get_u64_from_payload(rest);
    let date_time_off = Utc.timestamp(date_time_off as i64, 0);
    let (dx_call, rest) = get_string_from_payload(rest);
    let (dx_grid, rest) = get_string_from_payload(rest);
    let (tx_frequency_hz, rest) = get_u64_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (report_sent, rest) = get_string_from_payload(rest);
    let (report_received, rest) = get_string_from_payload(rest);
    let (tx_power, rest) = get_string_from_payload(rest);
    let (comments, rest) = get_string_from_payload(rest);
    let (name, rest) = get_string_from_payload(rest);
    let (date_time_on, rest) = get_u64_from_payload(rest);
    let date_time_on = Utc.timestamp(date_time_on as i64, 0);
    let (operator_call, rest) = get_string_from_payload(rest);
    let (my_call, rest) = get_string_from_payload(rest);
    let (my_grid, rest) = get_string_from_payload(rest);
    let (exchange_sent, rest) = get_string_from_payload(rest);
    let (exchange_received, rest) = get_string_from_payload(rest);
    let (adif_propagation_mode, rest) = get_string_from_payload(rest);
    let logdata = LogData {
        id,
        date_time_off,
        dx_call,
        dx_grid,
        tx_frequency_hz,
        mode,
        report_sent,
        report_received,
        tx_power,
        comments,
        name,
        date_time_on,
        operator_call,
        my_call,
        my_grid,
        exchange_sent,
        exchange_received,
        adif_propagation_mode,
    };
    if debug {
        println!("LogData: {}", logdata);
    }
}

pub fn decode_close(payload: &[u8], debug: bool){
    println!("Close message");
    let (id, rest) = get_string_from_payload(payload);
    let close = Close {
        id,
    };
    if debug {
        println!("Close: {}", close);
    }
}

pub fn decode_replay(payload: &[u8], debug: bool){
    println!("Replay message");
    let (id, rest) = get_string_from_payload(payload);
    let replay = Replay {
        id,
    };
    if debug {
        println!("Replay: {}", replay);
    }
}

pub fn decode_halt_tx(payload: &[u8], debug: bool){
    println!("Halt Tx message");
    let (id, rest) = get_string_from_payload(payload);
    let (auto_tx_only, rest) = get_bool_from_payload(rest);
    let halt_tx = HaltTx {
        id,
        auto_tx_only,
    };
    if debug {
        println!("HaltTx: {}", halt_tx);
    }
}
pub fn decode_free_text(payload: &[u8], debug: bool){
    println!("Free Text message");
    let (id, rest) = get_string_from_payload(payload);
    let (text, rest) = get_string_from_payload(rest);
    let (send, rest) = get_bool_from_payload(rest);
    let freetext = FreeText {
        id,
        text,
        send,
    };
    if debug {
        println!("FreeText: {}", freetext);
    }
}

pub fn decode_wspr_decode(payload: &[u8], debug: bool){
    println!("WSPR Decode message");
    let (id, rest) = get_string_from_payload(payload);
    let (new, rest) = get_bool_from_payload(rest);
    let (time, rest) = get_u32_from_payload(rest);
    let time = get_time_from_milliseconds_since_midnight(time);
    let (snr, rest) = get_i32_from_payload(rest);
    let (delta_time_s, rest) = get_f64_from_payload(rest);
    let (frequency_hz, rest) = get_u64_from_payload(rest);
    let (drift, rest) = get_i32_from_payload(rest);
    let (callsign, rest) = get_string_from_payload(rest);
    let (grid, rest) = get_string_from_payload(rest);
    let (power_dbm, rest) = get_i32_from_payload(rest);
    let (off_air, rest) = get_bool_from_payload(rest);
    let wsprdecode = WSPRDecode {
        id,
        new,
        time,
        snr,
        delta_time_s: delta_time_s as f64,
        frequency_hz,
        drift,
        callsign,
        grid,
        power_dbm,
        off_air,
    };
    if debug {
        println!("WSPRDecode: {}", wsprdecode);
    }
}

pub fn decode_location(payload: &[u8], debug: bool){
    println!("Location message");
    let (id, rest) = get_string_from_payload(payload);
    let (location, rest) = get_string_from_payload(rest);
    let location = Location {
        id,
        location,
    };
    if debug {
        println!("Location: {}", location);
    }
}

pub fn decode_logged_adif(payload: &[u8], debug: bool){
    println!("Logged ADIF message");
    let (id, rest) = get_string_from_payload(payload);
    let (adif, rest) = get_string_from_payload(rest);
    let loggedadif = LoggedADIF {
        id,
        adif,
    };
    if debug {
        println!("LoggedADIF: {}", loggedadif);
    }
}

pub fn decode_highlight_callsign_in(payload: &[u8], debug: bool) {
    println!("Highlight Callsign In message");
    let (id, rest) = get_string_from_payload(payload);
    let (callsign, rest) = get_string_from_payload(rest);
    let (background_color, rest) = get_string_from_payload(rest);
    let (foreground_color, rest) = get_string_from_payload(rest);
    let (highlight_last, rest) = get_bool_from_payload(rest);
    let highlightcallsignin = HighlightCallsignIn {
        id,
        callsign,
        background_color,
        foreground_color,
        highlight_last,
    };
    if debug {
        println!("HighlightCallsignIn: {}", highlightcallsignin);
    }
}

pub fn decode_switch_configuration(payload: &[u8], debug: bool) {
    println!("Switch Configuration message");
    let (id, rest) = get_string_from_payload(payload);
    let (configuration_name, rest) = get_string_from_payload(rest);
    let switchconfiguration = SwitchConfiguration {
        id,
        configuration_name,
    };
    if debug {
        println!("SwitchConfiguration: {}", switchconfiguration);
    }
}

pub fn decode_configure(payload: &[u8], debug: bool) {
    println!("Configure message");
    let (id, rest) = get_string_from_payload(payload);
    let (mode, rest) = get_string_from_payload(rest);
    let (frequency_tolerance, rest) = get_u32_from_payload(rest);
    let (submode, rest) = get_string_from_payload(rest);
    let (fast_mode, rest) = get_bool_from_payload(rest);
    let (tr_period, rest) = get_i32_from_payload(rest);
    let (rx_df, rest) = get_i32_from_payload(rest);
    let (dx_call, rest) = get_string_from_payload(rest);
    let (dx_grid, rest) = get_string_from_payload(rest);
    let (generate_messages, rest) = get_bool_from_payload(rest);
    let configure = Configure {
        id,
        mode,
        frequency_tolerance,
        submode,
        fast_mode,
        tr_period,
        rx_df,
        dx_call,
        dx_grid,
        generate_messages,
    };
    if debug {
        println!("Configure: {}", configure);
    }
}