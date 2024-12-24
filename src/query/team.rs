use super::{impl_paginated_query, impl_query_display};

use itertools::join;
use std::collections::HashMap;

use crate::schema::{EventLevel, Grade, MatchRound, SkillType};

/// Queries for the RobotEvents `/teams` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamsQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamsQuery);
impl_query_display!(TeamsQuery);

impl TeamsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: i32) -> Self {
        self.query.insert("id%5B%5D", id.to_string());
        self
    }
    pub fn ids(mut self, ids: &[i32]) -> Self {
        self.query.insert("id%5B%5D", join(ids, ","));
        self
    }

    pub fn number(mut self, number: String) -> Self {
        self.query.insert("number%5B%5D", number.to_string());
        self
    }
    pub fn numbers(mut self, numbers: Vec<String>) -> Self {
        self.query.insert("number%5B%5D", join(numbers, ","));
        self
    }

    pub fn event(mut self, event: i32) -> Self {
        self.query.insert("event%5B%5D", event.to_string());
        self
    }
    pub fn events(mut self, events: &[i32]) -> Self {
        self.query.insert("event%5B%5D", join(events, ","));
        self
    }

    pub fn registered(mut self, registered: bool) -> Self {
        self.query.insert("registered", registered.to_string());
        self
    }

    pub fn program(mut self, program: i32) -> Self {
        self.query.insert("program%5B%5D", program.to_string());
        self
    }
    pub fn programs(mut self, programs: &[i32]) -> Self {
        self.query.insert("program%5B%5D", join(programs, ","));
        self
    }

    pub fn grade(mut self, grade: Grade) -> Self {
        self.query.insert("grade%5B%5D", grade.to_string());
        self
    }
    pub fn grades(mut self, grades: &[Grade]) -> Self {
        self.query.insert("grade%5B%5D", join(grades, ","));
        self
    }

    pub fn country(mut self, country: String) -> Self {
        self.query.insert("country%5B%5D", country.to_string());
        self
    }
    pub fn countries(mut self, countrys: &[String]) -> Self {
        for c in countrys {
            self.query.insert("country%5B%5D", c.to_string());
        }
        self
    }

    pub fn my_teams(mut self, my_teams: bool) -> Self {
        self.query.insert("myTeams", my_teams.to_string());
        self
    }
}

/// Queries for the RobotEvents `/teams/:id/events` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamEventsQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamEventsQuery);
impl_query_display!(TeamEventsQuery);

impl TeamEventsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sku(mut self, sku: i32) -> Self {
        self.query.insert("sku%5B%5D", sku.to_string());
        self
    }
    pub fn skus(mut self, skus: &[i32]) -> Self {
        self.query.insert("sku%5B%5D", join(skus, ","));
        self
    }

    pub fn season(mut self, season: i32) -> Self {
        self.query.insert("season%5B%5D", season.to_string());
        self
    }
    pub fn seasons(mut self, seasons: &[i32]) -> Self {
        self.query.insert("season%5B%5D", join(seasons, ","));
        self
    }

    pub fn start(mut self, start: String) -> Self {
        self.query.insert("start", start);
        self
    }
    pub fn end(mut self, end: String) -> Self {
        self.query.insert("end", end);
        self
    }

    pub fn level(mut self, level: EventLevel) -> Self {
        self.query.insert("level%5B%5D", level.to_string());
        self
    }
    pub fn levels(mut self, levels: &[EventLevel]) -> Self {
        self.query.insert("level%5B%5D", join(levels, ","));
        self
    }
}

/// Queries for the RobotEvents `/teams/:id/matches` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamMatchesQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamMatchesQuery);
impl_query_display!(TeamMatchesQuery);

impl TeamMatchesQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event(mut self, event: i32) -> Self {
        self.query.insert("event%5B%5D", event.to_string());
        self
    }
    pub fn events(mut self, events: &[i32]) -> Self {
        self.query.insert("event%5B%5D", join(events, ","));
        self
    }

    pub fn season(mut self, season: i32) -> Self {
        self.query.insert("season%5B%5D", season.to_string());
        self
    }
    pub fn seasons(mut self, seasons: &[i32]) -> Self {
        self.query.insert("season%5B%5D", join(seasons, ","));
        self
    }

    pub fn round(mut self, round: MatchRound) -> Self {
        self.query.insert("round%5B%5D", (round as i32).to_string());
        self
    }
    pub fn rounds(mut self, rounds: &[MatchRound]) -> Self {
        self.query.insert(
            "round%5B%5D",
            join(rounds.iter().map(|round| round.clone() as i32), ","),
        );
        self
    }

    pub fn instance(mut self, instance: i32) -> Self {
        self.query.insert("instance%5B%5D", instance.to_string());
        self
    }
    pub fn instances(mut self, instances: &[i32]) -> Self {
        self.query.insert("instance%5B%5D", join(instances, ","));
        self
    }

    pub fn matchnum(mut self, matchnum: i32) -> Self {
        self.query.insert("matchnum%5B%5D", matchnum.to_string());
        self
    }
    pub fn matchnums(mut self, matchnums: &[i32]) -> Self {
        self.query.insert("matchnum%5B%5D", join(matchnums, ","));
        self
    }
}

/// Queries for the RobotEvents `/teams/:id/rankings` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamRankingsQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamRankingsQuery);
impl_query_display!(TeamRankingsQuery);

impl TeamRankingsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event(mut self, event: i32) -> Self {
        self.query.insert("event%5B%5D", event.to_string());
        self
    }
    pub fn events(mut self, events: &[i32]) -> Self {
        self.query.insert("event%5B%5D", join(events, ","));
        self
    }

    pub fn rank(mut self, rank: i32) -> Self {
        self.query.insert("rank%5B%5D", rank.to_string());
        self
    }
    pub fn ranks(mut self, ranks: &[i32]) -> Self {
        self.query.insert("rank%5B%5D", join(ranks, ","));
        self
    }

    pub fn season(mut self, season: i32) -> Self {
        self.query.insert("season%5B%5D", season.to_string());
        self
    }
    pub fn seasons(mut self, seasons: &[i32]) -> Self {
        self.query.insert("season%5B%5D", join(seasons, ","));
        self
    }
}

/// Queries for the RobotEvents `/teams/:id/skills` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamSkillsQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamSkillsQuery);
impl_query_display!(TeamSkillsQuery);

impl TeamSkillsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event(mut self, event: i32) -> Self {
        self.query.insert("event%5B%5D", event.to_string());
        self
    }
    pub fn events(mut self, events: &[i32]) -> Self {
        self.query.insert("event%5B%5D", join(events, ","));
        self
    }

    pub fn skill_type(mut self, skill_type: SkillType) -> Self {
        self.query.insert("type%5B%5D", skill_type.to_string());
        self
    }
    pub fn skill_types(mut self, skill_types: &[SkillType]) -> Self {
        self.query.insert("type%5B%5D", join(skill_types, ","));
        self
    }

    pub fn season(mut self, season: i32) -> Self {
        self.query.insert("season%5B%5D", season.to_string());
        self
    }
    pub fn seasons(mut self, seasons: &[i32]) -> Self {
        self.query.insert("season%5B%5D", join(seasons, ","));
        self
    }
}

/// Queries for the RobotEvents `/teams/:id/awards` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TeamAwardsQuery {
    query: HashMap<&'static str, String>,
}

impl_paginated_query!(TeamAwardsQuery);
impl_query_display!(TeamAwardsQuery);

impl TeamAwardsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event(mut self, event: i32) -> Self {
        self.query.insert("event%5B%5D", event.to_string());
        self
    }
    pub fn events(mut self, events: &[i32]) -> Self {
        self.query.insert("event%5B%5D", join(events, ","));
        self
    }

    pub fn season(mut self, season: i32) -> Self {
        self.query.insert("season%5B%5D", season.to_string());
        self
    }
    pub fn seasons(mut self, seasons: &[i32]) -> Self {
        self.query.insert("season%5B%5D", join(seasons, ","));
        self
    }
}
