use std::collections::HashMap;

use crate::{
    client::error,
    query::{
        DivisionMatchesQuery, DivisionRankingsQuery, EventAwardsQuery, EventSkillsQuery,
        EventTeamsQuery,
    },
    schema::{IdInfo, Location},
    RobotEvents,
};

use serde::{Deserialize, Serialize};

use super::{Award, Match, PaginatedResponse, Ranking, Skill, Team};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Division {
    pub id: i32,
    pub name: String,
    pub order: i32,
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
    pub locations: HashMap<String, Location>,
    pub divisions: Vec<Division>,
    pub level: EventLevel,
    pub ongoing: bool,
    pub awards_finalized: bool,
    pub event_type: Option<EventType>,
}

impl Event {
    pub async fn teams(
        &self,
        client: &RobotEvents,
        query: EventTeamsQuery,
    ) -> Result<PaginatedResponse<Team>, error::Error> {
        client.event_teams(self.id, query).await
    }
    pub async fn skills(
        &self,
        client: &RobotEvents,
        query: EventSkillsQuery,
    ) -> Result<PaginatedResponse<Skill>, error::Error> {
        client.event_skills(self.id, query).await
    }
    pub async fn awards(
        &self,
        client: &RobotEvents,
        query: EventAwardsQuery,
    ) -> Result<PaginatedResponse<Award>, error::Error> {
        client.event_awards(self.id, query).await
    }
    pub async fn division_matches(
        &self,
        division_id: i32,
        client: &RobotEvents,
        query: DivisionMatchesQuery,
    ) -> Result<PaginatedResponse<Match>, error::Error> {
        client
            .event_division_matches(self.id, division_id, query)
            .await
    }
    pub async fn division_finalist_rankings(
        &self,
        division_id: i32,
        client: &RobotEvents,
        query: DivisionRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, error::Error> {
        client
            .event_division_finalist_rankings(self.id, division_id, query)
            .await
    }
    pub async fn division_rankings(
        &self,
        division_id: i32,
        client: &RobotEvents,
        query: DivisionRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, error::Error> {
        client
            .event_division_rankings(self.id, division_id, query)
            .await
    }
}
