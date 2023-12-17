use super::{IdInfo, Location, Season};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Division {
    id: i32,
    name: Season,
    order: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventLevel {
    World,
    National,
    Regional,
    State,
    Signature,
    Other,
}

impl std::fmt::Display for EventLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::World => "World",
            Self::National => "National",
            Self::Regional => "Regional",
            Self::State => "State",
            Self::Signature => "Signature",
            Self::Other => "Other",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Tournament,
    League,
    Workshop,
    Virtual,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Tournament => "Tournament",
            Self::League => "League",
            Self::Workshop => "Workshop",
            Self::Virtual => "Virtual",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub start: String,
    pub end: String,
    pub season: IdInfo,
    pub program: IdInfo,
    pub location: Location,
    pub locations: Vec<Location>,
    pub division: Vec<Division>,
    pub level: EventLevel,
    pub ongoing: bool,
    pub awards_finalized: bool,
    pub event_type: EventType,
}
