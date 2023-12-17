use crate::{
    client::RobotEvents,
    filters::{TeamEventsFilter, TeamRankingsFilter, TeamMatchesFilter, TeamAwardsFilter, TeamSkillsFilter},
    schema::{Award, Event, Grade, IdInfo, Location, Match, PaginatedResponse, Ranking, Skill}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub number: String,
    pub team_name: String,
    pub robot_name: Option<String>,
    pub organization: String,
    pub location: Location,
    pub registered: bool,
    pub program: IdInfo,
    pub grade: Grade,
}

impl Team {
    pub async fn events(&self, client: &RobotEvents, filter: TeamEventsFilter) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(client.request(format!("/teams/{}/events{filter}", self.id)).await?.json().await?)
    }
    pub async fn matches(&self, client: &RobotEvents, filter: TeamMatchesFilter) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        Ok(client.request(format!("/teams/{}/matches{filter}", self.id)).await?.json().await?)
    }
    pub async fn rankings(&self, client: &RobotEvents, filter: TeamRankingsFilter) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(client.request(format!("/teams/{}/rankings{filter}", self.id)).await?.json().await?)
    }
    pub async fn skills(&self, client: &RobotEvents, filter: TeamSkillsFilter) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        Ok(client.request(format!("/teams/{}/skills{filter}", self.id)).await?.json().await?)
    }
    pub async fn awards(&self, client: &RobotEvents, filter: TeamAwardsFilter) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        Ok(client.request(format!("/teams/{}/awards{filter}", self.id)).await?.json().await?)
    }
}