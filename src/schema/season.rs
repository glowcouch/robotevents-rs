use serde::{Serialize, Deserialize};
use super::{RobotEvents, PaginatedResponse, Event, IdInfo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: i32,
    pub name: String,
    pub program: IdInfo,
    pub start: String,
    pub end: String,
    pub years_start: i32,
    pub years_end: i32,
}

impl Season {
    pub async fn events(&self, client: &RobotEvents) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(client.request(format!("/seasons/{}/events", self.id)).await?.json().await?)
    }
}