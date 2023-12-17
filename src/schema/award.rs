use super::IdInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamAwardWinner {
    division: IdInfo,
    team: IdInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AwardClassification {
    Champion,
    Finalist,
    Semifinalist,
    Quarterfinalist,
}

impl std::fmt::Display for AwardClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Champion => "Champion",
            Self::Finalist => "Finalist",
            Self::Semifinalist => "Semifinalist",
            Self::Quarterfinalist => "Quarterfinalist",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AwardDesignation {
    Tournament,
    Division,
}

impl std::fmt::Display for AwardDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Tournament => "Tournament",
            Self::Division => "Division",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub id: i32,
    pub event: IdInfo,
    pub order: i32,
    pub title: String,
    pub qualifications: Vec<String>,
    pub designcation: Option<AwardDesignation>,
    pub classification: Option<AwardClassification>,
    pub team_winners: Vec<TeamAwardWinner>,
    pub individual_winners: Vec<String>,
}
