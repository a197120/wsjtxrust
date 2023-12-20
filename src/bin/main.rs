use std::net::UdpSocket;
use std::str;
use serde_derive::{Serialize, Deserialize};
use byteorder::{ByteOrder, BigEndian};
use chrono::{DateTime, TimeZone, NaiveTime};
use chrono::offset::Utc;
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    magic_number: u32,
    schema_number: u32,
    payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Heartbeat {
    id: String,
    maximum_schema_number: u32,
    version: String,
    revision: String,
}

impl std::fmt::Display for Heartbeat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Maximum schema number: {}, Version: {}, Revision: {}",
               self.id, self.maximum_schema_number, self.version, self.revision)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    id: String,
    dial_frequency: u64,
    mode: String,
    dx_call: String,
    report: String,
    tx_mode: String,
    tx_enabled: bool,
    transmitting: bool,
    decoding: bool,
    rx_df: u32,
    tx_df: u32,
    de_call: String,
    de_grid: String,
    dx_grid: String,
    tx_watchdog: bool,
    sub_mode: String,
    fast_mode: bool,
    special_operation_mode: u8,
    frequency_tolerance: u32,
    tr_period: u32,
    configuration_name: String,
    tx_message: String,
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
struct Decode {
    id: String,
    new: bool,
    time: NaiveTime,
    snr: i32,
    delta_time_s: f64,
    delta_frequency_hz: f64,
    mode: String,
    message: String,
    low_confidence: bool,
    off_air: bool,
}
#[derive(Debug)]
struct Clear {
    id: String,
    window: u8,
}

#[derive(Debug)]
struct Reply {
    id: String,
    time: NaiveTime,
    snr: i32,
    delta_time_s: f64,
    delta_frequency_hz: u32,
    mode: String,
    message: String,
    low_confidence: bool,
    modifiers: u8,
}

#[derive(Debug)]
struct LogData {
    id: String,
    date_time_off: DateTime<Utc>,
    dx_call: String,
    dx_grid: String,
    tx_frequency_hz: u64,
    mode: String,
    report_sent: String,
    report_received: String,
    tx_power: String,
    comments: String,
    name: String,
    date_time_on: DateTime<Utc>,
    operator_call: String,
    my_call: String,
    my_grid: String,
    exchange_sent: String,
    exchange_received: String,
    adif_propagation_mode: String,
}

#[derive(Debug)]
struct Close{
    id: String,
}

#[derive(Debug)]
struct Replay {
    id: String,
}

#[derive(Debug)]
struct HaltTx {
    id: String,
    auto_tx_only: bool,
}

#[derive(Debug)]
struct FreeText {
    id: String,
    text: String,
    send: bool,
}

#[derive(Debug)]
struct WSPRDecode {
    id: String,
    new: bool,
    time: NaiveTime,
    snr: i32,
    delta_time_s: f64,
    frequency_hz: f64,  // are QINT64
    drift: i32,
    callsign: String,
    grid: String,
    power_dbm: i32,
    off_air: bool,
}

#[derive(Debug)]
struct Location {
    id: String,
    location: String,
}

#[derive(Debug)]
struct LoggedADIF {
    id: String,
    adif: String,
}

#[derive(Debug)]
struct HighlightCallsignIn {
    id: String,
    callsign: String,
    background_color: String,  // are QCOLOR
    foreground_color: String,
    highlight_last: bool
}
#[derive(Debug)]
struct SwitchConfiguration {
    id: String,
    configuration_name: String,
}

#[derive(Debug)]
struct Configure {
    id: String,
    mode: String,
    frequency_tolerance: i32,
    submode: String,
    fast_mode: bool,
    tr_period: i32,
    rx_df: i32,
    dx_call: String,
    dx_grid: String,
    generate_messages: bool,
}

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

fn handle_data(data: &[u8]) {
    // println!("Raw message ={:?}", data);
    if data.len() < 8 {
        eprintln!("Data too short to be a valid message");
        return;
    }

    let (header, payload) = data.split_at(8);
    // println!("Header bytes: {:?}", header);

    let magic_number = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let schema_number = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);

    let message = Message {
        magic_number,
        schema_number,
        payload: payload.to_vec(),
    };
    println!("Magic Number: {:#x}", magic_number);
    println!("Schema Number: {:x}", schema_number);
    // println!("Received message: {:?}", data);
    let (messagetype, payload) = payload.split_at(4);
    let messagetype = u32::from_be_bytes([messagetype[0], messagetype[1], messagetype[2], messagetype[3]]);
    // println!("Message Type: {:x}", messagetype); 
    // if message type equals zero
    match messagetype {
        0 => {
            let (id, rest) = get_string_from_payload(payload);
            println!("Id: {}", id);
            let (max_schema_number_bytes, rest) = rest.split_at(4);
            let max_schema_number = BigEndian::read_u32(&max_schema_number_bytes);       
            let (version, rest) = get_string_from_payload(rest);
            let (revision, rest) = get_string_from_payload(rest);
            let heartbeat = Heartbeat {
                 id: id,
                 maximum_schema_number: max_schema_number,
                 version: version,
                 revision: revision,
                };
            println!("Heartbeat: {}", heartbeat);
        },
        1 => {
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
            println!("Status: {:?}", status);
        },
        2 => {
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
                delta_frequency_hz: delta_frequency_hz as f64,
                mode,
                message,
                low_confidence,
                off_air,
            };
            println!("Decode: {:?}", decode);
        },
        3 => {
            println!("Clear message");
            let (id, rest) = get_string_from_payload(payload);
            let (window, rest) = get_u8_from_payload(rest);
            let clear = Clear {
                id,
                window,
            };
            println!("Clear: {:?}", clear);
        },
        4 => {
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
            println!("Reply: {:?}", reply);
        },
        5 => { 
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
            println!("LogData: {:?}", logdata);
        },
        6 => {
            println!("Close message");
            let (id, rest) = get_string_from_payload(payload);
            let close = Close {
                id,
            };
            println!("Close: {:?}", close);
        },
        7 => {
            println!("Replay message");
            let (id, rest) = get_string_from_payload(payload);
            let replay = Replay {
                id,
            };
            println!("Replay: {:?}", replay);
        },
        8 => {
            println!("Halt Tx message");
            let (id, rest) = get_string_from_payload(payload);
            let (auto_tx_only, rest) = get_bool_from_payload(rest);
            let halt_tx = HaltTx {
                id,
                auto_tx_only,
            };
            println!("HaltTx: {:?}", halt_tx);
        },
        9 => {
            println!("Free Text message");
            let (id, rest) = get_string_from_payload(payload);
            let (text, rest) = get_string_from_payload(rest);
            let (send, rest) = get_bool_from_payload(rest);
            let freetext = FreeText {
                id,
                text,
                send,
            };
            println!("FreeText: {:?}", freetext);
        },
        10 => {
            println!("WSPR Decode message");
            let (id, rest) = get_string_from_payload(payload);
            let (new, rest) = get_bool_from_payload(rest);
            let (time, rest) = get_u32_from_payload(rest);
            let time = get_time_from_milliseconds_since_midnight(time);
            let (snr, rest) = get_i32_from_payload(rest);
            let (delta_time_s, rest) = get_f64_from_payload(rest);
            let (frequency_hz, rest) = get_f64_from_payload(rest);
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
            println!("WSPRDecode: {:?}", wsprdecode);
        },
        11 => {
            println!("Location message");
            let (id, rest) = get_string_from_payload(payload);
            let (location, rest) = get_string_from_payload(rest);
            let location = Location {
                id,
                location,
            };
            println!("Location: {:?}", location);
        
        },
        12 => {
            println!("Logged ADIF message");
            let (id, rest) = get_string_from_payload(payload);
            let (adif, rest) = get_string_from_payload(rest);
            let loggedadif = LoggedADIF {
                id,
                adif,
            };
            println!("LoggedADIF: {:?}", loggedadif);
        },
        13 => {
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
            println!("HighlightCallsignIn: {:?}", highlightcallsignin);
        },

        14 => {
            println!("Switch Configuration message");
            let (id, rest) = get_string_from_payload(payload);
            let (configuration_name, rest) = get_string_from_payload(rest);
            let switchconfiguration = SwitchConfiguration {
                id,
                configuration_name,
            };
            println!("SwitchConfiguration: {:?}", switchconfiguration);
        },
        15 => {
            println!("Configure message");
            let (id, rest) = get_string_from_payload(payload);
            let (mode, rest) = get_string_from_payload(rest);
            let (frequency_tolerance, rest) = get_i32_from_payload(rest);
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
            println!("Configure: {:?}", configure);
        },
        _ => println!("Message Type not supported"),
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