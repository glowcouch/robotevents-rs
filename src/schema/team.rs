use crate::{
    client::RobotEvents,
    filters::{
        TeamAwardsFilter, TeamEventsFilter, TeamMatchesFilter, TeamRankingsFilter, TeamSkillsFilter,
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
    pub organization: String,
    pub location: Location,
    pub registered: bool,
    pub program: IdInfo,
    pub grade: Grade,
}

impl Team {
    pub async fn events(
        &self,
        client: &RobotEvents,
        filter: TeamEventsFilter,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        client.team_events(self.id, filter).await
    }
    pub async fn matches(
        &self,
        client: &RobotEvents,
        filter: TeamMatchesFilter,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        client.team_matches(self.id, filter).await
    }
    pub async fn rankings(
        &self,
        client: &RobotEvents,
        filter: TeamRankingsFilter,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        client.team_rankings(self.id, filter).await
    }
    pub async fn skills(
        &self,
        client: &RobotEvents,
        filter: TeamSkillsFilter,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        client.team_skills(self.id, filter).await
    }
    pub async fn awards(
        &self,
        client: &RobotEvents,
        filter: TeamAwardsFilter,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        client.team_awards(self.id, filter).await
    }
}
