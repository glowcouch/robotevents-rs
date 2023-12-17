use super::impl_filter_display;
use itertools::join;
use std::collections::HashMap;

use crate::schema::{EventLevel, EventType, Grade, MatchRound, SkillType};

/// Filters for the RobotEvents `/events` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(EventsFilter);

impl EventsFilter {
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

    pub fn sku(mut self, sku: i32) -> Self {
        self.query.insert("sku%5B%5D", sku.to_string());
        self
    }
    pub fn skus(mut self, skus: &[i32]) -> Self {
        self.query.insert("sku%5B%5D", join(skus, ","));
        self
    }

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
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

    pub fn region(mut self, region: String) -> Self {
        self.query.insert("start", region);
        self
    }

    pub fn level(mut self, level: EventLevel) -> Self {
        self.query.insert("level%5B%5D", level.to_string());
        self
    }
    pub fn levels(mut self, levels: &[EventLevel]) -> Self {
        self.query.insert("season%5B%5D", join(levels, ","));
        self
    }

    pub fn my_events(mut self, my_events: bool) -> Self {
        self.query.insert("my_events", my_events.to_string());
        self
    }

    pub fn event_type(mut self, event_type: EventType) -> Self {
        self.query
            .insert("event_type%5B%5D", event_type.to_string());
        self
    }
    pub fn event_types(mut self, event_types: &[EventType]) -> Self {
        self.query.insert("season%5B%5D", join(event_types, ","));
        self
    }
}

/// Filters for the RobotEvents `/event/:id/teams` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventTeamsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(EventTeamsFilter);

impl EventTeamsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn number(mut self, number: String) -> Self {
        self.query.insert("number%5B%5D", number.to_string());
        self
    }
    pub fn numbers(mut self, numbers: Vec<String>) -> Self {
        self.query.insert("number%5B%5D", join(numbers, ","));
        self
    }

    pub fn registered(mut self, number: String) -> Self {
        self.query.insert("registered", number.to_string());
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

    pub fn country(mut self, country: i32) -> Self {
        self.query.insert("country%5B%5D", country.to_string());
        self
    }
    pub fn countries(mut self, countrys: &[i32]) -> Self {
        self.query.insert("country%5B%5D", join(countrys, ","));
        self
    }

    pub fn my_teams(mut self, my_teams: bool) -> Self {
        self.query.insert("myTeams", my_teams.to_string());
        self
    }
}

/// Filters for the RobotEvents `/events/:id/skills` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventSkillsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(EventSkillsFilter);

impl EventSkillsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
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
}

/// Filters for the RobotEvents `/event/:id/awards` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EventAwardsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(EventAwardsFilter);

impl EventAwardsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
        self
    }

    pub fn winner(mut self, winner: String) -> Self {
        self.query.insert("winner%5B%5D", winner.to_string());
        self
    }
    pub fn winners(mut self, winners: Vec<String>) -> Self {
        self.query.insert("winner%5B%5D", join(winners, ","));
        self
    }
}

/// Filters for the RobotEvents `/event/:id/divisions/:div/matches` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DivisionMatchesFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(DivisionMatchesFilter);

impl DivisionMatchesFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
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


/// Filters for the RobotEvents `/event/:id/divisions/:div/finalistRankings` and `/event/:id/divisions/:div/rankings` endpoints.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DivisionRankingsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(DivisionRankingsFilter);

impl DivisionRankingsFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
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
}
