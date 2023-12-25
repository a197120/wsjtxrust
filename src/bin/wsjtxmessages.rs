pub mod receivemessages;
pub mod sendmessages;
use chrono::{DateTime, TimeZone, NaiveTime, Timelike};
use chrono::offset::Utc;
// use receivemessages::*;
// use sendmessages::*;
use byteorder::{ByteOrder, BigEndian};
use serde_derive::Serialize;


#[derive(Debug, Serialize)]
pub struct Message {
    pub magic_number: u32,
    pub schema_number: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct Heartbeat {
    message_type: u32,
    id: String,
    maximum_schema_number: u32,
    version: String,
    revision: String,
}

impl std::fmt::Display for Heartbeat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Maximum schema number: {}, Version: {}, Revision: {}", self.message_type,
            self.id, self.maximum_schema_number, self.version, self.revision)
    }
}

#[derive(Debug)]
pub struct Status {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, Dial Frequency: {}, Mode: {}, dx_call: {}, report: {}, tx_mode: {}, tx_enabled: {}, transmitting: {}, decoding: {}, rx_df: {}, tx_df: {}, de_call: {}, de_grid: {}, dx_grid: {}, tx_watchdog: {}, sub_mode: {}, fast_mode: {}, special_operation_mode: {}, frequency_tolerance: {}, tr_period: {}, configuration_name: {}, tx_message: {}", self.message_type
        , self.id, self.dial_frequency, self.mode, self.dx_call, self.report, self.tx_mode, self.tx_enabled, self.transmitting, self.decoding, self.rx_df, self.tx_df, self.de_call, self.de_grid, self.dx_grid, self.tx_watchdog, self.sub_mode, self.fast_mode, self.special_operation_mode, self.frequency_tolerance, self.tr_period, self.configuration_name, self.tx_message)
    }
}

// #[derive(Serialize, Deserialize, Debug, Display)]
#[derive(Debug)]
pub struct Decode {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, New: {}, Time: {}, SNR: {}, Delta Time: {}, Delta Frequency: {}, Mode: {}, Message: {}, Low Confidence: {}, Off Air: {}", self.message_type
        , self.id, self.new, self.time, self.snr, self.delta_time_s, self.delta_frequency_hz, self.mode, self.message, self.low_confidence, self.off_air)
    }
}
#[derive(Debug)]
pub struct Clear {
    pub message_type: u32,
    pub id: String,
    pub window: u8,
}
impl std::fmt::Display for Clear{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Window: {}", self.message_type
        , self.id, self.window)
    }
}

#[derive(Debug)]
pub struct Reply {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, Time: {}, SNR: {}, Delta Time: {}, Delta Frequency: {}, Mode: {}, Message: {}, Low Confidence: {}, Modifiers: {}", self.message_type
        , self.id, self.time, self.snr, self.delta_time_s, self.delta_frequency_hz, self.mode, self.message, self.low_confidence, self.modifiers)
    }
}

#[derive(Debug)]
pub struct LogData {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, Date Time Off: {}, DX Call: {}, DX Grid: {}, TX Frequency: {}, Mode: {}, Report Sent: {}, Report Received: {}, TX Power: {}, Comments: {}, Name: {}, Date Time On: {}, Operator Call: {}, My Call: {}, My Grid: {}, Exchange Sent: {}, Exchange Received: {}, ADIF Propagation Mode: {}", self.message_type
        , self.id, self.date_time_off, self.dx_call, self.dx_grid, self.tx_frequency_hz, self.mode, self.report_sent, self.report_received, self.tx_power, self.comments, self.name, self.date_time_on, self.operator_call, self.my_call, self.my_grid, self.exchange_sent, self.exchange_received, self.adif_propagation_mode)
    }
}
#[derive(Debug, Serialize)]
pub struct Close{
    pub message_type: u32,
    pub id: String,
}
impl std::fmt::Display for Close{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}", self.message_type
        , self.id)
    }
}
#[derive(Debug)]
pub struct Replay {
    message_type: u32,
    id: String,
}
impl std::fmt::Display for Replay{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}", self.message_type
        , self.id)
    }
}
#[derive(Debug)]
pub struct HaltTx {
    message_type: u32,
    id: String,
    auto_tx_only: bool,
}
impl std::fmt::Display for HaltTx{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Auto Tx Only: {}", self.message_type
        , self.id, self.auto_tx_only)
    }
}
#[derive(Debug)]
pub struct FreeText {
    pub message_type: u32,
    pub id: String,
    pub text: String,
    pub send: bool,
}
impl std::fmt::Display for FreeText{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Text: {}, Send: {}", self.message_type
        , self.id, self.text, self.send)
    }
}

#[derive(Debug)]
pub struct WSPRDecode {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, New: {}, Time: {}, SNR: {}, Delta Time: {}, Frequency: {}, Drift: {}, Callsign: {}, Grid: {}, Power: {}, Off Air: {}", self.message_type, 
        self.id, self.new, self.time, self.snr, self.delta_time_s, self.frequency_hz, self.drift, self.callsign, self.grid, self.power_dbm, self.off_air)
    }
}
#[derive(Debug)]
pub struct Location {
    message_type: u32,
    id: String,
    location: String,
}
impl std::fmt::Display for Location{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Location: {}", self.message_type
        , self.id, self.location)
    }
}
#[derive(Debug)]
pub struct LoggedADIF {
    message_type: u32,
    id: String,
    adif: String,
}
impl std::fmt::Display for LoggedADIF{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, ADIF: {}", self.message_type
        , self.id, self.adif)
    }
}
#[derive(Debug)]
pub struct HighlightCallsignIn {
    message_type: u32,
    id: String,
    callsign: String,
    background_color: String,  // are QCOLOR
    foreground_color: String,
    highlight_last: bool
}
impl std::fmt::Display for HighlightCallsignIn{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Callsign: {}, Background Color: {}, Foreground Color: {}, Highlight Last: {}", self.message_type
        , self.id, self.callsign, self.background_color, self.foreground_color, self.highlight_last)
    }
}
#[derive(Debug)]
pub struct SwitchConfiguration {
    message_type: u32,
    id: String,
    configuration_name: String,
}
impl std::fmt::Display for SwitchConfiguration{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Message Type: {}, Id: {}, Configuration Name: {}", self.message_type
        , self.id, self.configuration_name)
    }
}
#[derive(Debug)]
pub struct Configure {
    message_type: u32,
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
        write!(f, "Message Type: {}, Id: {}, Mode: {}, Frequency Tolerance: {}, Submode: {}, Fast Mode: {}, TR Period: {}, RX DF: {}, DX Call: {}, DX Grid: {}, Generate Messages: {}", self.message_type
        , self.id, self.mode, self.frequency_tolerance, self.submode, self.fast_mode, self.tr_period, self.rx_df, self.dx_call, self.dx_grid, self.generate_messages)
    }
}



