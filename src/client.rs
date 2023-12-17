use super::{
    filters::{EventsFilter, SeasonsFilter, TeamsFilter},
    schema::*,
};
use reqwest::header::USER_AGENT;
use std::time::Duration;

#[derive(Default, Debug, Clone)]
pub struct RobotEvents {
    pub bearer_token: String,
    pub req_client: reqwest::Client,
}

pub const V1_API_BASE: &str = "https://www.robotevents.com/api/v1";
pub const V2_API_BASE: &str = "https://www.robotevents.com/api/v2";

impl RobotEvents {
    pub fn new(bearer_token: impl AsRef<str>) -> Self {
        Self {
            bearer_token: bearer_token.as_ref().to_owned(),
            req_client: reqwest::Client::new(),
        }
    }

    /// Make a request to a [RobotEvents API v2](https://www.robotevents.com/api/v2) endpoint.
    ///
    /// Requires a bearer authentication token to be provided for requests to work. This can
    /// be obtained from RobotEvents by creating an account and requesting one.
    pub async fn request(
        &self,
        endpoint: impl AsRef<str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .req_client
            .get(format!("{V2_API_BASE}{}", endpoint.as_ref()))
            .header("accept-language", "en")
            .header(USER_AGENT, "RoboStats Discord Bot")
            .bearer_auth(&self.bearer_token)
            .timeout(Duration::from_secs(10))
            .send()
            .await?)
    }

    /// Make a request to a RobotEvents API v1 endpoint.
    pub async fn request_api_v1(
        &self,
        endpoint: impl AsRef<str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .req_client
            .get(format!("{V1_API_BASE}{}", endpoint.as_ref()))
            .header("accept-language", "en")
            .header(USER_AGENT, "RoboStats Discord Bot")
            .timeout(Duration::from_secs(10))
            .send()
            .await?)
    }

    /// Get a paginated list of [`Team`]s from RobotEvents.
    ///
    /// Team listings can be filtered using a [`TeamFilter`] search.
    pub async fn teams(
        &self,
        filter: TeamsFilter,
    ) -> Result<PaginatedResponse<Team>, reqwest::Error> {
        Ok(self
            .request(format!("/teams{filter}"))
            .await?
            .json()
            .await?)
    }
    /// Get a specific RobotEvents [`Team`] by ID.
    pub async fn team(&self, team_id: i32) -> Result<Team, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of [`Season`]s from RobotEvents.
    ///
    /// Season listings can be filtered using a [`SeasonFilter`] search.
    pub async fn seasons(
        &self,
        filter: SeasonsFilter,
    ) -> Result<PaginatedResponse<Season>, reqwest::Error> {
        Ok(self
            .request(format!("/seasons{filter}"))
            .await?
            .json()
            .await?)
    }
    /// Get a specific RobotEvents [`Season`] by ID.
    pub async fn season(&self, season_id: i32) -> Result<Season, reqwest::Error> {
        Ok(self
            .request(format!("/seasons/{season_id}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of all programs from RobotEvents.
    pub async fn programs(&self) -> Result<PaginatedResponse<IdInfo>, reqwest::Error> {
        Ok(self.request("/programs").await?.json().await?)
    }
    /// Get a specific RobotEvents program by ID.
    pub async fn program(&self, program_id: i32) -> Result<IdInfo, reqwest::Error> {
        Ok(self
            .request(format!("/programs/{program_id}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of [`Event`]s from RobotEvents.
    ///
    /// Event listings can be filtered using an [`EventFilter`] search.
    pub async fn events(
        &self,
        filter: EventsFilter,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/programs{filter}"))
            .await?
            .json()
            .await?)
    }
    /// Get a specific RobotEvents event by ID.
    pub async fn event(&self, event_id: i32) -> Result<Event, reqwest::Error> {
        Ok(self
            .request(format!("/programs/{event_id}"))
            .await?
            .json()
            .await?)
    }
}
