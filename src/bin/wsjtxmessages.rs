use serde_derive::{Serialize, Deserialize};
use byteorder::{ByteOrder, BigEndian};
use chrono::{DateTime, TimeZone, NaiveTime};
use chrono::offset::Utc;
use derive_more::{Display};
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub magic_number: u32,
    pub schema_number: u32,
    pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
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
pub struct Status {
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
impl std::fmt::Display for Status{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Dial Frequency: {}, Mode: {}, dx_call: {}, report: {}, tx_mode: {}, tx_enabled: {}, transmitting: {}, decoding: {}, rx_df: {}, tx_df: {}, de_call: {}, de_grid: {}, dx_grid: {}, tx_watchdog: {}, sub_mode: {}, fast_mode: {}, special_operation_mode: {}, frequency_tolerance: {}, tr_period: {}, configuration_name: {}, tx_message: {}"
        , self.id, self.dial_frequency, self.mode, self.dx_call, self.report, self.tx_mode, self.tx_enabled, self.transmitting, self.decoding, self.rx_df, self.tx_df, self.de_call, self.de_grid, self.dx_grid, self.tx_watchdog, self.sub_mode, self.fast_mode, self.special_operation_mode, self.frequency_tolerance, self.tr_period, self.configuration_name, self.tx_message)
    }
}

// #[derive(Serialize, Deserialize, Debug, Display)]
#[derive(Debug)]
pub struct Decode {
    id: String,
    new: bool,
    time: NaiveTime,
    snr: i32,
    delta_time_s: f64,
    delta_frequency_hz: u32,
    mode: String,
    message: String,
    low_confidence: bool,
    off_air: bool,
}
impl std::fmt::Display for Decode{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, New: {}, Time: {}, SNR: {}, Delta Time: {}, Delta Frequency: {}, Mode: {}, Message: {}, Low Confidence: {}, Off Air: {}"
        , self.id, self.new, self.time, self.snr, self.delta_time_s, self.delta_frequency_hz, self.mode, self.message, self.low_confidence, self.off_air)
    }
}
#[derive(Debug)]
pub struct Clear {
    id: String,
    window: u8,
}
impl std::fmt::Display for Clear{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Window: {}"
        , self.id, self.window)
    }
}

#[derive(Debug)]
pub struct Reply {
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
impl std::fmt::Display for Reply{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Time: {}, SNR: {}, Delta Time: {}, Delta Frequency: {}, Mode: {}, Message: {}, Low Confidence: {}, Modifiers: {}"
        , self.id, self.time, self.snr, self.delta_time_s, self.delta_frequency_hz, self.mode, self.message, self.low_confidence, self.modifiers)
    }
}

#[derive(Debug)]
pub struct LogData {
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
impl std::fmt::Display for LogData{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Date Time Off: {}, DX Call: {}, DX Grid: {}, TX Frequency: {}, Mode: {}, Report Sent: {}, Report Received: {}, TX Power: {}, Comments: {}, Name: {}, Date Time On: {}, Operator Call: {}, My Call: {}, My Grid: {}, Exchange Sent: {}, Exchange Received: {}, ADIF Propagation Mode: {}"
        , self.id, self.date_time_off, self.dx_call, self.dx_grid, self.tx_frequency_hz, self.mode, self.report_sent, self.report_received, self.tx_power, self.comments, self.name, self.date_time_on, self.operator_call, self.my_call, self.my_grid, self.exchange_sent, self.exchange_received, self.adif_propagation_mode)
    }
}
#[derive(Debug)]
pub struct Close{
    id: String,
}
impl std::fmt::Display for Close{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}"
        , self.id)
    }
}
#[derive(Debug)]
pub struct Replay {
    id: String,
}
impl std::fmt::Display for Replay{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}"
        , self.id)
    }
}
#[derive(Debug)]
pub struct HaltTx {
    id: String,
    auto_tx_only: bool,
}
impl std::fmt::Display for HaltTx{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Auto Tx Only: {}"
        , self.id, self.auto_tx_only)
    }
}
#[derive(Debug)]
pub struct FreeText {
    id: String,
    text: String,
    send: bool,
}
impl std::fmt::Display for FreeText{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Text: {}, Send: {}"
        , self.id, self.text, self.send)
    }
}

#[derive(Debug)]
pub struct WSPRDecode {
    id: String,
    new: bool,
    time: NaiveTime,
    snr: i32,
    delta_time_s: f64,
    frequency_hz: u64,  
    drift: i32,
    callsign: String,
    grid: String,
    power_dbm: i32,
    off_air: bool,
}
impl std::fmt::Display for WSPRDecode{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, New: {}, Time: {}, SNR: {}, Delta Time: {}, Frequency: {}, Drift: {}, Callsign: {}, Grid: {}, Power: {}, Off Air: {}", 
        self.id, self.new, self.time, self.snr, self.delta_time_s, self.frequency_hz, self.drift, self.callsign, self.grid, self.power_dbm, self.off_air)
    }
}
#[derive(Debug)]
pub struct Location {
    id: String,
    location: String,
}
impl std::fmt::Display for Location{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Location: {}"
        , self.id, self.location)
    }
}
#[derive(Debug)]
pub struct LoggedADIF {
    id: String,
    adif: String,
}
impl std::fmt::Display for LoggedADIF{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, ADIF: {}"
        , self.id, self.adif)
    }
}
#[derive(Debug)]
pub struct HighlightCallsignIn {
    id: String,
    callsign: String,
    background_color: String,  // are QCOLOR
    foreground_color: String,
    highlight_last: bool
}
impl std::fmt::Display for HighlightCallsignIn{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Callsign: {}, Background Color: {}, Foreground Color: {}, Highlight Last: {}"
        , self.id, self.callsign, self.background_color, self.foreground_color, self.highlight_last)
    }
}
#[derive(Debug)]
pub struct SwitchConfiguration {
    id: String,
    configuration_name: String,
}
impl std::fmt::Display for SwitchConfiguration{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Configuration Name: {}"
        , self.id, self.configuration_name)
    }
}
#[derive(Debug)]
pub struct Configure {
    id: String,
    mode: String,
    frequency_tolerance: u32,
    submode: String,
    fast_mode: bool,
    tr_period: i32,
    rx_df: i32,
    dx_call: String,
    dx_grid: String,
    generate_messages: bool,
}
impl std::fmt::Display for Configure{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Id: {}, Mode: {}, Frequency Tolerance: {}, Submode: {}, Fast Mode: {}, TR Period: {}, RX DF: {}, DX Call: {}, DX Grid: {}, Generate Messages: {}"
        , self.id, self.mode, self.frequency_tolerance, self.submode, self.fast_mode, self.tr_period, self.rx_df, self.dx_call, self.dx_grid, self.generate_messages)
    }
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
