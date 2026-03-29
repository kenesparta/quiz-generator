use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug)]
pub struct Link {
    pub href: String,
    pub method: String,
}

impl Link {
    pub fn get(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "GET".to_string(),
        }
    }

    pub fn post(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "POST".to_string(),
        }
    }

    pub fn patch(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "PATCH".to_string(),
        }
    }

    pub fn put(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "PUT".to_string(),
        }
    }

    pub fn delete(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "DELETE".to_string(),
        }
    }
}

pub type Links = HashMap<String, Link>;

#[derive(Serialize, Debug)]
pub struct ListResponse<T: Serialize> {
    #[serde(rename = "_links")]
    pub links: Links,
    pub items: Vec<T>,
}
