use super::IdInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub event: IdInfo,
    pub division: IdInfo,
    pub round: i32,
    pub instance: i32,
    pub matchnum: i32,
    pub scheduled: String,
    pub started: String,
    pub field: String,
    pub scored: bool,
    pub name: String,
    pub alliances: Vec<Alliance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllianceColor {
    Red,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alliance {
    pub color: AllianceColor,
    pub score: i32,
    pub teams: Vec<AllianceTeam>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllianceTeam {
    pub team: IdInfo,
    pub sitting: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchRound {
    Practice = 1,
    Qualification = 2,
    Quarterfinals = 3,
    Semifinals = 4,
    Finals = 5,
    RoundOf16 = 6,
}
