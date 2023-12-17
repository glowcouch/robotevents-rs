use super::IdInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub event: IdInfo,
    pub team: IdInfo,
    #[serde(rename = "type")]
    pub skill_type: SkillType,
    pub season: IdInfo,
    pub division: IdInfo,
    pub rank: i32,
    pub score: i32,
    pub attempts: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    Driver,
    Programming,
    PackageDeliveryTime,
}

impl std::fmt::Display for SkillType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::Driver => "driver",
            Self::Programming => "programming",
            Self::PackageDeliveryTime => "package_delivery_time",
        })
    }
}
