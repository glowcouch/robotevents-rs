use super::IdInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ranking {
    pub id: i32,
    pub event: IdInfo,
    pub division: IdInfo,
    pub rank: i32,
    pub team: IdInfo,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub wp: i32,
    pub ap: i32,
    pub sp: i32,
    pub high_score: i32,
    pub average_points: f64,
    pub total_points: i32,
}
