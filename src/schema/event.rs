use crate::{
    RobotEvents,
    filters::{EventTeamsFilter, EventSkillsFilter, EventAwardsFilter, DivisionMatchesFilter, DivisionRankingsFilter},
    schema::{IdInfo, Location, Season}
};

use serde::{Deserialize, Serialize};

use super::{PaginatedResponse, Team, Skill, Award, Match, Ranking};

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

impl Event {
    pub async fn teams(
        &self,
        client: &RobotEvents,
        filter: EventTeamsFilter,
    ) -> Result<PaginatedResponse<Team>, reqwest::Error> {
        client.event_teams(self.id, filter).await
    }
    pub async fn skills(
        &self,
        client: &RobotEvents,
        filter: EventSkillsFilter,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        client.event_skills(self.id, filter).await
    }
    pub async fn awards(
        &self,
        client: &RobotEvents,
        filter: EventAwardsFilter,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        client.event_awards(self.id, filter).await
    }
    pub async fn division_matches(
        &self,
        division_id: i32,
        client: &RobotEvents,
        filter: DivisionMatchesFilter,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        client.event_division_matches(self.id, division_id, filter).await
    }
    pub async fn division_finalist_rankings(
        &self,
        division_id: i32,
        client: &RobotEvents,
        filter: DivisionRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        client.event_division_finalist_rankings(self.id, division_id, filter).await
    }
    pub async fn division_rankings(
        &self,
        division_id: i32,
        client: &RobotEvents,
        filter: DivisionRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        client.event_division_rankings(self.id, division_id, filter).await
    }
}
