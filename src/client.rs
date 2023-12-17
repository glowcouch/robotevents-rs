use crate::filters::{
    DivisionMatchesFilter, DivisionRankingsFilter, EventAwardsFilter, EventSkillsFilter,
    EventTeamsFilter, SeasonEventsFilter, TeamAwardsFilter, TeamEventsFilter, TeamMatchesFilter,
    TeamRankingsFilter, TeamSkillsFilter,
};

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
    /// Creates a new RobotEvents API client.
    ///
    /// A bearer authentication token is required for requests to be made. This can
    /// be obtained from RobotEvents by creating an account and requesting one.
    /// 
    /// # Examples
    /// 
    /// Creating a client with a token stored as an enviornment variable:
    /// 
    /// ```
    /// use robotevents::RobotEvents;
    /// 
    /// let token = std::env::var("ROBOTEVENTS_TOKEN")?;
    /// let client = RobotEvents::new(token);
    /// ```
    pub fn new(bearer_token: impl AsRef<str>) -> Self {
        Self {
            bearer_token: bearer_token.as_ref().to_owned(),
            req_client: reqwest::Client::new(),
        }
    }

    /// Make a request to a [RobotEvents API v2](https://www.robotevents.com/api/v2) endpoint using the
    /// client's bearer token.
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

    /////////////////////////////////////////////////////////////////////////
    // Team-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of [`Team`]s from RobotEvents.
    ///
    /// Team listings can be filtered using a [`TeamsFilter`] search.
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

    /// Gets a List of [`Event`]s that a given Team ID has attended.
    pub async fn team_events(
        &self,
        team_id: i32,
        filter: TeamEventsFilter,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/events{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Match`]es that a given Team ID has played in.
    pub async fn team_matches(
        &self,
        team_id: i32,
        filter: TeamMatchesFilter,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/matches{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Ranking`]s that a given Team ID has played in.
    pub async fn team_rankings(
        &self,
        team_id: i32,
        filter: TeamRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/rankings{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Skill`]s runs that a given Team ID has performed.
    pub async fn team_skills(
        &self,
        team_id: i32,
        filter: TeamSkillsFilter,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/skills{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Award`]s that a given Team ID has received.
    pub async fn team_awards(
        &self,
        team_id: i32,
        filter: TeamAwardsFilter,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/awards{filter}"))
            .await?
            .json()
            .await?)
    }

    
    /////////////////////////////////////////////////////////////////////////
    // Season-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

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

    /// Gets a List of Events for a given Season.
    pub async fn season_events(
        &self,
        season_id: i32,
        filter: SeasonEventsFilter,
    ) -> Result<PaginatedResponse<Season>, reqwest::Error> {
        Ok(self
            .request(format!("/seasons/{season_id}/events{filter}"))
            .await?
            .json()
            .await?)
    }

    
    /////////////////////////////////////////////////////////////////////////
    // Program-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

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

    
    /////////////////////////////////////////////////////////////////////////
    // Event-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of [`Event`]s from RobotEvents.
    ///
    /// Event listings can be filtered using an [`EventFilter`] search.
    pub async fn events(
        &self,
        filter: EventsFilter,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/events{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Get a specific RobotEvents event by ID.
    pub async fn event(&self, event_id: i32) -> Result<Event, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of teams attending an event.
    pub async fn event_teams(
        &self,
        event_id: i32,
        filter: EventTeamsFilter,
    ) -> Result<PaginatedResponse<Team>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/teams{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of skills runs at an event.
    pub async fn event_skills(
        &self,
        event_id: i32,
        filter: EventSkillsFilter,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/skills{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of skills runs at an event.
    pub async fn event_awards(
        &self,
        event_id: i32,
        filter: EventAwardsFilter,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/awards{filter}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Matches for a single Division of an Event.
    pub async fn event_division_matches(
        &self,
        event_id: i32,
        division_id: i32,
        filter: DivisionMatchesFilter,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/matches{filter}"
            ))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Finalist Rankings for a single Division of an Event.
    pub async fn event_division_finalist_rankings(
        &self,
        event_id: i32,
        division_id: i32,
        filter: DivisionRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/finalistRankings{filter}"
            ))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Rankings for a single Division of an Event.
    pub async fn event_division_rankings(
        &self,
        event_id: i32,
        division_id: i32,
        filter: DivisionRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/finalist{filter}"
            ))
            .await?
            .json()
            .await?)
    }
}
