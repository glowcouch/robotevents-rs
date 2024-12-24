use crate::{
    client::RobotEvents,
    query::{
        TeamAwardsQuery, TeamEventsQuery, TeamMatchesQuery, TeamRankingsQuery, TeamSkillsQuery,
    },
    schema::{Award, Event, IdInfo, Location, Match, PaginatedResponse, Ranking, Skill},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Grade {
    College,

    #[serde(rename = "High School")]
    HighSchool,

    #[serde(rename = "Middle School")]
    MiddleSchool,

    #[serde(rename = "Elementary School")]
    ElementarySchool,
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::College => "College",
            Self::HighSchool => "High School",
            Self::MiddleSchool => "Middle School",
            Self::ElementarySchool => "Elementary School",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub number: String,
    pub team_name: String,
    pub robot_name: Option<String>,
    pub organization: Option<String>,
    pub location: Location,
    pub registered: bool,
    pub program: IdInfo,
    pub grade: Grade,
}

impl Team {
    pub async fn events(
        &self,
        client: &RobotEvents,
        query: TeamEventsQuery,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        client.team_events(self.id, query).await
    }
    pub async fn matches(
        &self,
        client: &RobotEvents,
        query: TeamMatchesQuery,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        client.team_matches(self.id, query).await
    }
    pub async fn rankings(
        &self,
        client: &RobotEvents,
        query: TeamRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        client.team_rankings(self.id, query).await
    }
    pub async fn skills(
        &self,
        client: &RobotEvents,
        query: TeamSkillsQuery,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        client.team_skills(self.id, query).await
    }
    pub async fn awards(
        &self,
        client: &RobotEvents,
        query: TeamAwardsQuery,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        client.team_awards(self.id, query).await
    }
}
