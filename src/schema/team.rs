use crate::{
    client::{error, RobotEvents},
    query::{
        TeamAwardsQuery, TeamEventsQuery, TeamMatchesQuery, TeamRankingsQuery, TeamSkillsQuery,
    },
    schema::{Award, Event, IdInfo, Location, Match, PaginatedResponse, Ranking, Skill},
};
use itertools::Itertools;
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

#[cfg(feature = "fake")]
pub struct FakeGrade;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeGrade> for Grade {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeGrade, rng: &mut R) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::College,
            1 => Self::HighSchool,
            2 => Self::MiddleSchool,
            3 => Self::ElementarySchool,
            _ => unreachable!(),
        }
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
    ) -> Result<PaginatedResponse<Event>, error::Error> {
        client.team_events(self.id, query).await
    }
    pub async fn matches(
        &self,
        client: &RobotEvents,
        query: TeamMatchesQuery,
    ) -> Result<PaginatedResponse<Match>, error::Error> {
        client.team_matches(self.id, query).await
    }
    pub async fn rankings(
        &self,
        client: &RobotEvents,
        query: TeamRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, error::Error> {
        client.team_rankings(self.id, query).await
    }
    pub async fn skills(
        &self,
        client: &RobotEvents,
        query: TeamSkillsQuery,
    ) -> Result<PaginatedResponse<Skill>, error::Error> {
        client.team_skills(self.id, query).await
    }
    pub async fn awards(
        &self,
        client: &RobotEvents,
        query: TeamAwardsQuery,
    ) -> Result<PaginatedResponse<Award>, error::Error> {
        client.team_awards(self.id, query).await
    }
}

#[cfg(feature = "fake")]
pub struct FakeTeam;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeTeam> for Team {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeTeam, rng: &mut R) -> Self {
        use crate::schema::FakeLocation;
        use fake::{
            faker::company::en::{Buzzword, CompanyName},
            Fake,
        };

        Team {
            id: rng.gen_range(0..99999),
            number: format!("{}{}", rng.gen_range(0..99999), rng.gen_range('A'..'X')),
            team_name: Buzzword().fake_with_rng(rng),
            organization: CompanyName().fake_with_rng(rng),
            location: FakeLocation.fake_with_rng(rng),
            robot_name: Buzzword().fake_with_rng(rng),
            registered: rng.gen_bool(0.5),
            program: IdInfo {
                id: rng.gen_range(1..60),
                name: format!(
                    "{} {}",
                    {
                        let v: String = Buzzword().fake_with_rng(rng);
                        v
                    },
                    {
                        let v: String = CompanyName().fake_with_rng(rng);
                        v
                    }
                ),
                code: Some(
                    (0..rng.gen_range(0..6))
                        .map(|_| match rng.gen_bool(0.2) {
                            true => rng.gen_range(0..=9).to_string(),
                            false => rng.gen_range('A'..='Z').to_string(),
                        })
                        .join(""),
                ),
            },
            grade: FakeGrade.fake_with_rng(rng),
        }
    }
}
