pub mod award;
pub mod event;
pub mod location;
pub mod matches;
pub mod program;
pub mod ranking;
pub mod season;
pub mod skill;
pub mod team;

pub use award::*;
pub use event::*;
pub use location::*;
pub use matches::*;
pub use program::*;
pub use ranking::*;
pub use season::*;
pub use skill::*;
pub use team::*;

use crate::{client::error, RobotEvents, V2_API_BASE};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageMeta {
    pub current_page: i32,
    pub first_page_url: String,
    pub from: Option<i32>,
    pub to: Option<i32>,
    pub last_page: i32,
    pub last_page_url: String,
    pub prev_page_url: Option<String>,
    pub next_page_url: Option<String>,
    pub path: String,
    pub per_page: i32,
    pub total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub meta: PageMeta,
    pub data: Vec<T>,
}

impl<T: DeserializeOwned> PaginatedResponse<T> {
    pub async fn prev_page(
        &self,
        robotevents: &RobotEvents,
    ) -> Option<Result<PaginatedResponse<T>, error::Error>> {
        if let Some(url) = &self.meta.prev_page_url {
            match robotevents
                .request(url.trim_start_matches(V2_API_BASE))
                .await
            {
                Ok(response) => match response.json().await {
                    Ok(json) => Some(Ok(json)),
                    Err(error) => Some(Err(error.into())),
                },
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }

    pub async fn next_page(
        &self,
        robotevents: &RobotEvents,
    ) -> Option<Result<PaginatedResponse<T>, error::Error>> {
        if let Some(url) = &self.meta.next_page_url {
            match robotevents
                .request(url.trim_start_matches(V2_API_BASE))
                .await
            {
                Ok(response) => match response.json().await {
                    Ok(json) => Some(Ok(json)),
                    Err(error) => Some(Err(error.into())),
                },
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }

    pub async fn first_page(
        &self,
        robotevents: &RobotEvents,
    ) -> Result<PaginatedResponse<T>, error::Error> {
        robotevents
            .request(self.meta.first_page_url.trim_start_matches(V2_API_BASE))
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }

    pub async fn last_page(
        &self,
        robotevents: &RobotEvents,
    ) -> Result<PaginatedResponse<T>, error::Error> {
        robotevents
            .request(self.meta.last_page_url.trim_start_matches(V2_API_BASE))
            .await?
            .json()
            .await
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdInfo {
    pub id: i32,
    pub name: String,
    pub code: Option<String>,
}

