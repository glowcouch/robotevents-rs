use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub venue: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub city: String,
    pub region: String,
    pub postcode: String,
    pub country: String,
    pub coordinates: Coordinates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}