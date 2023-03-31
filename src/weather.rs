use alloc::string::String;
use alloc::vec::Vec;

use serde::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
struct ObservationsRaw {
    observations: Observations,
}

#[derive(Debug, Deserialize)]
struct Observations {
    data: Vec<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub enum WindDirection {
    CALM,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    NNE,
    ENE,
    ESE,
    SSE,
    SSW,
    WSW,
    WNW,
    NNW,
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub sort_order: u32,
    pub name: String,
    pub history_product: String,
    pub local_date_time: String,      //"11/01:30pm",
    pub local_date_time_full: String, // "20180811133000",
    pub aifstime_utc: String,         // "20180811033000",
    pub lat: f32,
    pub lon: f32,
    pub apparent_t: f32,
    pub delta_t: f32,
    pub gust_kmh: u32,
    pub gust_kt: u32,
    pub air_temp: f32,
    pub dewpt: f32,
    pub press: Option<f32>,
    pub press_qnh: Option<f32>,
    pub press_msl: Option<f32>,
    pub press_tend: String,
    pub rain_trace: String, // Rain since 9am, not sure why this is a string
    pub rel_hum: u32,
    pub wind_dir: WindDirection,
    pub wind_spd_kmh: u32,
    pub wind_spd_kt: u32,
}

pub fn parse(text: &[u8]) -> Result<Option<Observation>, serde_json::Error> {
    serde_json::from_slice::<'_, ObservationsRaw>(text)
        .map(|obs| obs.observations.data.into_iter().next())
        .and_then(|value| {
            value
                .map(|v| serde_json::from_value::<Observation>(v))
                .transpose()
        })
}
