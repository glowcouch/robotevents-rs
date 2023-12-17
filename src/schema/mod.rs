pub mod team;
pub mod award;
pub mod event;
pub mod location;
pub mod skill;
pub mod season;
pub mod matches;
pub mod ranking;

pub use team::*;
pub use award::*;
pub use event::*;
pub use location::*;
pub use skill::*;
pub use season::*;
pub use matches::*;
pub use ranking::*;

use crate::api::robotevents::{RobotEvents, V2_API_BASE};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMeta {
    current_page: i32,
    first_page_url: String,
    from: i32,
    last_page: i32,
    last_page_url: String,
    prev_page_url: Option<String>,
    next_page_url: Option<String>,
    path: String,
    per_page: i32,
    to: i32,
    total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub meta: PageMeta,
    pub data: Vec<T>,
}

impl<T: DeserializeOwned> PaginatedResponse<T> {
    pub async fn prev_page(&self, robotevents: &RobotEvents) -> Option<Result<PaginatedResponse<T>, reqwest::Error>> {
        if let Some(url) = &self.meta.prev_page_url {
            match robotevents.request(url.trim_start_matches(V2_API_BASE)).await {
                Ok(response) => match response.json::<PaginatedResponse<T>>().await {
                    Ok(json) => Some(Ok(json)),
                    Err(error) => Some(Err(error)),
                },
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }

    pub async fn next_page(&self, robotevents: &RobotEvents) -> Option<Result<PaginatedResponse<T>, reqwest::Error>> {
        if let Some(url) = &self.meta.next_page_url {
            match robotevents.request(url.trim_start_matches(V2_API_BASE)).await {
                Ok(response) => match response.json::<PaginatedResponse<T>>().await {
                    Ok(json) => Some(Ok(json)),
                    Err(error) => Some(Err(error)),
                },
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }

    pub async fn first_page(&self, robotevents: &RobotEvents) -> Result<PaginatedResponse<T>, reqwest::Error> {
        robotevents.request(self.meta.first_page_url.trim_start_matches(V2_API_BASE)).await?.json::<PaginatedResponse<T>>().await
    }

    pub async fn last_page(&self, robotevents: &RobotEvents) -> Result<PaginatedResponse<T>, reqwest::Error> {
        robotevents.request(self.meta.last_page_url.trim_start_matches(V2_API_BASE)).await?.json::<PaginatedResponse<T>>().await
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdInfo {
    pub id: i32,
    pub name: String,
    #[serde(alias = "abbr")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Grade {
    College,

    #[serde(rename = "High School")]
    HighSchool,

    #[serde(rename = "Middle School")]
    MiddleSchool,

    #[serde(rename = "Elementary School")]
    ElementarySchool
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Self::College => "College",
            Self::HighSchool => "High School",
            Self::MiddleSchool => "Middle School",
            Self::ElementarySchool => "Elementary School"
        })
    }
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Error {
//     pub code: i32,
//     pub message: String,
// }