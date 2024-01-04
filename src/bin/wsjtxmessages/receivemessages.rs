use super::*;

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
    if payload.is_empty() {
        (0, &[])
    } else {
        let (bytes, rest) = payload.split_at(1);
        let value = bytes[0];
        (value, rest)
    }
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
fn get_qdatetime_from_payload(payload: &[u8]) -> (NaiveDate, NaiveTime, u8, Option<i32>, &[u8]) {
    let (bytes, rest) = payload.split_at(8);
    let julian_day_number = i64::from_be_bytes(bytes.try_into().unwrap());
    let date = NaiveDate::from_num_days_from_ce((julian_day_number - 1_721_425) as i32);

    let (bytes, rest) = rest.split_at(4);
    let ms_since_midnight = u32::from_be_bytes(bytes.try_into().unwrap());
    let time = get_time_from_milliseconds_since_midnight(ms_since_midnight);

    let (bytes, rest) = rest.split_at(1);
    let timespec = u8::from_be_bytes(bytes.try_into().unwrap());

    let (offset, rest) = if timespec == 2 {
        let (bytes, rest) = rest.split_at(4);
        let offset = i32::from_be_bytes(bytes.try_into().unwrap());
        (Some(offset), rest)
    } else {
        (None, rest)
    };

    (date, time, timespec, offset, rest)
}
pub fn decode_heartbeat(payload: &[u8], debug: bool) -> Heartbeat{
    if debug {
        info!("Heartbeat message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (maximum_schema_number, rest) = get_u32_from_payload(rest);      
    let (version, rest) = get_string_from_payload(rest);
    let (revision, _rest) = get_string_from_payload(rest);
    let heartbeat = Heartbeat {
        message_type,
        id,
        maximum_schema_number,
        version: version,
        revision: revision,
        };
    if debug {
        info!("Heartbeat: {}", heartbeat)
    }
    heartbeat
}

pub fn decode_status(payload: &[u8], debug: bool) -> Status {
    if debug {
        info!("Status message");
    }
    // println!("Payload: {:?}", payload);
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
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
    let (tx_message, _rest) = get_string_from_payload(rest);

    let status = Status {
        message_type,
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
        info!("Status: {}", status);
    }
    status
}

pub fn decode_decode(payload: &[u8], debug: bool) -> Decode {
    if debug {
        info!("Decode message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (new, rest) = get_bool_from_payload(rest);
    let (time, rest) = get_u32_from_payload(rest);
    let time = get_time_from_milliseconds_since_midnight(time);
    let (snr, rest) = get_i32_from_payload(rest);
    let (delta_time_s, rest) = get_f64_from_payload(rest);
    let (delta_frequency_hz, rest) = get_u32_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (message, rest) = get_string_from_payload(rest);
    let (low_confidence, rest) = get_bool_from_payload(rest);
    let (off_air, _rest) = get_bool_from_payload(rest);
    let decode = Decode {
        message_type,
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
        info!("Decode: {}", decode);
    }
    decode
}

pub fn decode_clear(payload: &[u8], debug: bool) -> Clear{
    if debug {
        info!("Clear message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (window, _rest) = get_u8_from_payload(rest);
    let clear = Clear {
        message_type,
        id,
        window,
    };
    if debug {
        info!("Clear: {}", clear);
    }
    clear
}

pub fn decode_reply(payload: &[u8], debug: bool) -> Reply{
    if debug {
        info!("Reply message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (time, rest) = get_u32_from_payload(rest);
    let time = get_time_from_milliseconds_since_midnight(time);
    let (snr, rest) = get_i32_from_payload(rest);
    let (delta_time_s, rest) = get_f64_from_payload(rest);
    let (delta_frequency_hz, rest) = get_u32_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (message, rest) = get_string_from_payload(rest);
    let (low_confidence, rest) = get_bool_from_payload(rest);
    let (modifiers, _rest) = get_u8_from_payload(rest);
    let reply = Reply {
        message_type,
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
        info!("Reply: {}", reply);
    }
    reply
}

pub fn decode_logdata(payload: &[u8], debug: bool, app_state: &mut AppState) -> LogData {
    if debug {
        info!("LogData message");
    }
    info!("Decoding logdata, payload length: {}", payload.len());
    let (message_type, rest) = get_u32_from_payload(payload);
    info!("message_type: {}", message_type);
    let (id, rest) = get_string_from_payload(rest);
    info!("id: {}", id);
    let (dateoff, timeoff, timespecoff, offsetoff, rest) = get_qdatetime_from_payload(rest);    
    let date_time_off = dateoff.and_time(timeoff);
    info!("date_time_off: {}, timespec {:?} offset {:?}", date_time_off, timespecoff, offsetoff);
    let (dx_call, rest) = get_string_from_payload(rest);
    info!("dx_call: {}", dx_call);
    let (dx_grid, rest) = get_string_from_payload(rest);
    info!("dx_grid: {}", dx_grid);
    let (tx_frequency_hz, rest) = get_u64_from_payload(rest);
    info!("tx_frequency_hz: {}", tx_frequency_hz);
    let (mode, rest) = get_string_from_payload(rest);
    info!("mode: {}", mode);
    let (report_sent, rest) = get_string_from_payload(rest);
    info!("report_sent: {}", report_sent);
    let (report_received, rest) = get_string_from_payload(rest);
    info!("report_received: {}", report_received);
    let (tx_power, rest) = get_string_from_payload(rest);
    info!("tx_power: {}", tx_power);
    let (comments, rest) = get_string_from_payload(rest);
    info!("comments: {}", comments);
    let (name, rest) = get_string_from_payload(rest);
    info!("name: {}", name);
    let (dateon, timeon, timespecon, offseton, rest) = get_qdatetime_from_payload(rest);    
    info!("dateon: {}, timeon: {}, timespecon: {:?}, offseton: {:?}", dateon, timeon, timespecon, offseton);
    let date_time_on = dateon.and_time(timeon);
    info!("date_time_on: {}, timespec {:?} offset {:?}", date_time_on, timespecon, offseton);
    let (operator_call, rest) = get_string_from_payload(rest);
    info!("operator_call: {}", operator_call);
    let (my_call, rest) = get_string_from_payload(rest);
    info!("my_call: {}", my_call);
    let (my_grid, rest) = get_string_from_payload(rest);
    info!("my_grid: {}", my_grid);
    let (exchange_sent, rest) = get_string_from_payload(rest);
    info!("exchange_sent: {}", exchange_sent);
    let (exchange_received, rest) = get_string_from_payload(rest);
    info!("exchange_received: {}", exchange_received);
    let (adif_propagation_mode, _rest) = get_string_from_payload(rest);
    info!("adif_propagation_mode: {}", adif_propagation_mode);
    let logdata = LogData {
        message_type,
        id,
        date_time_off,
        timespecoff,
        offsetoff,
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
        timespecon,
        offseton,
        operator_call,
        my_call,
        my_grid,
        exchange_sent,
        exchange_received,
        adif_propagation_mode,
    };
    if debug {
        info!("LogData: {}", logdata);
    }
    logdata
}

pub fn decode_close(payload: &[u8], debug: bool) -> Close{
    if debug {
        info!("Close message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, _rest) = get_string_from_payload(rest);
    let close = Close {
        message_type,
        id,
    };
    if debug {
        info!("Close: {}", close);
    }
    close
}

pub fn decode_replay(payload: &[u8], debug: bool) -> Replay{
    if debug {
        info!("Replay message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, _rest) = get_string_from_payload(rest);
    let replay = Replay {
        message_type,
        id,
    };
    if debug {
        info!("Replay: {}", replay);
    }
    replay
}

pub fn decode_halt_tx(payload: &[u8], debug: bool) -> HaltTx{
    if debug {
        info!("Halt Tx message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (auto_tx_only, _rest) = get_bool_from_payload(rest);
    let halt_tx = HaltTx {
        message_type,
        id,
        auto_tx_only,
    };
    if debug {
        info!("HaltTx: {}", halt_tx);
    }
    halt_tx
}
pub fn decode_free_text(payload: &[u8], debug: bool) -> FreeText{
    if debug {
        info!("Free Text message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (text, rest) = get_string_from_payload(rest);
    let (send, _rest) = get_bool_from_payload(rest);
    let freetext = FreeText {
        message_type,
        id,
        text,
        send,
    };
    if debug {
        info!("FreeText: {}", freetext);
    }
    freetext
}

pub fn decode_wspr_decode(payload: &[u8], debug: bool) -> WSPRDecode{
    if debug {
        info!("WSPR Decode message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
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
    let (off_air, _rest) = get_bool_from_payload(rest);
    let wsprdecode = WSPRDecode {
        message_type,
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
        info!("WSPRDecode: {}", wsprdecode);
    }
    wsprdecode
}

pub fn decode_location(payload: &[u8], debug: bool) -> Location { 
    if debug {
        info!("Location message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (location, _rest) = get_string_from_payload(rest);
    let location = Location {
        message_type,
        id,
        location,
    };
    if debug {
        info!("Location: {}", location);
    }
    location
}

pub fn decode_logged_adif(payload: &[u8], debug: bool) -> LoggedADIF{
    if debug {
        info!("Logged ADIF message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (adif, _rest) = get_string_from_payload(rest);
    let loggedadif = LoggedADIF {
        message_type,
        id,
        adif,
    };
    if debug {
        info!("LoggedADIF: {}", loggedadif);
    }
    loggedadif
}

pub fn decode_highlight_callsign_in(payload: &[u8], debug: bool) -> HighlightCallsignIn{
    if debug {
        info!("Highlight Callsign In message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (callsign, rest) = get_string_from_payload(rest);
    let (background_color, rest) = get_string_from_payload(rest);
    let (foreground_color, rest) = get_string_from_payload(rest);
    let (highlight_last, _rest) = get_bool_from_payload(rest);
    let highlightcallsignin = HighlightCallsignIn {
        message_type,
        id,
        callsign,
        background_color,
        foreground_color,
        highlight_last,
    };
    if debug {
        info!("HighlightCallsignIn: {}", highlightcallsignin);
    }
    highlightcallsignin
}

pub fn decode_switch_configuration(payload: &[u8], debug: bool) -> SwitchConfiguration{
    if debug {
        info!("Switch Configuration message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (configuration_name, _rest) = get_string_from_payload(rest);
    let switchconfiguration = SwitchConfiguration {
        message_type,
        id,
        configuration_name,
    };
    if debug {
        info!("SwitchConfiguration: {}", switchconfiguration);
    }
    switchconfiguration
}

pub fn decode_configure(payload: &[u8], debug: bool) -> Configure{
    if debug {
        info!("Configure message");
    }
    let (message_type, rest) = get_u32_from_payload(payload);
    let (id, rest) = get_string_from_payload(rest);
    let (mode, rest) = get_string_from_payload(rest);
    let (frequency_tolerance, rest) = get_u32_from_payload(rest);
    let (submode, rest) = get_string_from_payload(rest);
    let (fast_mode, rest) = get_bool_from_payload(rest);
    let (tr_period, rest) = get_i32_from_payload(rest);
    let (rx_df, rest) = get_i32_from_payload(rest);
    let (dx_call, rest) = get_string_from_payload(rest);
    let (dx_grid, rest) = get_string_from_payload(rest);
    let (generate_messages, _rest) = get_bool_from_payload(rest);
    let configure = Configure {
        message_type,
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
        info!("Configure: {}", configure);
    }
    configure
}

pub fn handle_incoming_data(data: &[u8], app_state: &mut AppState) {
    if data.len() < 8 {
        info!("Data too short to be a valid message");
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
        info!("Message: {:?}", message);
    }
    // info!("Magic Number: {:#x}", magic_number);
    // info!("Schema Number: {:x}", schema_number);
    // info!("Received message: {:?}", data);
    // info!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    //get messagetype from the payload
    let messagetype = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]);
    // info!("Message Type: {:x}", messagetype); 
    match messagetype {
        0 => { decode_heartbeat(payload, DEBUG); }
        1 => { decode_status(payload, DEBUG).print_status(app_state); }
        2 => { decode_decode(payload, DEBUG).print_message(app_state) ; }
        // 2 => { decode_decode(payload, DEBUG) ; }
        3 => { decode_clear(payload, true); }
        4 => { decode_reply(payload, DEBUG); }
        5 => { decode_logdata(payload, true, app_state); }
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
        _ => info!("Unknown Message Type"),
        // _ => {}, // this was modified to ignore unknown message types presented in modified versions of WSJTX
    };
}
