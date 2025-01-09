use super::IdInfo;
use serde::{Deserialize, Serialize};

#[cfg(feature = "client")]
use super::{Event, PaginatedResponse, RobotEvents};
#[cfg(feature = "client")]
use crate::client::error;

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

#[cfg(feature = "client")]
impl Season {
    pub async fn events(
        &self,
        client: &RobotEvents,
    ) -> Result<PaginatedResponse<Event>, error::Error> {
        Ok(client
            .request(format!("/seasons/{}/events", self.id))
            .await?
            .json()
            .await?)
    }
}
