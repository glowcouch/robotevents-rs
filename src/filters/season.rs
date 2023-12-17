use std::collections::HashMap;
use itertools::join;
use crate::{schema::EventLevel, filters::impl_filter_display};

/// Filters for the RobotEvents `/seasons` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SeasonsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(SeasonsFilter);

impl SeasonsFilter {
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

    pub fn program(mut self, program: i32) -> Self {
        self.query.insert("program%5B%5D", program.to_string());
        self
    }
	pub fn programs(mut self, programs: &[i32]) -> Self {
        self.query.insert("program%5B%5D", join(programs, ","));
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

    pub fn start(mut self, start: String) -> Self {
        self.query.insert("start", start);
        self
    }
    pub fn end(mut self, end: String) -> Self {
        self.query.insert("start", end);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.query.insert("active", active.to_string());
        self
    }
}

/// Filters for the RobotEvents `/seasons/:id/events` endpoint.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SeasonEventsFilter {
    query: HashMap<&'static str, String>,
}

impl_filter_display!(SeasonEventsFilter);

impl SeasonEventsFilter {
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

    pub fn team(mut self, team: i32) -> Self {
        self.query.insert("team%5B%5D", team.to_string());
        self
    }
    pub fn teams(mut self, teams: &[i32]) -> Self {
        self.query.insert("team%5B%5D", join(teams, ","));
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
        self.query.insert("season%5B%5D", join(levels, ","));
        self
    }
}
