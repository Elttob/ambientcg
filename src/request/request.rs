use std::collections::{HashSet, HashMap};

use chrono::{DateTime, Utc};

use crate::{json, response};

#[derive(Debug, Default)]
pub struct Request {
    pub q: Vec<String>,
    pub method: HashSet<crate::CreationMethod>,
    pub data_type: HashSet<crate::DataType>,
    pub sort: Option<crate::Sort>,
    pub id: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub include: HashSet<super::Include>
}

impl Request {
    pub fn as_query_params(&self) -> String {
        let mut params = HashMap::new();

        if !self.q.is_empty() {
            params.insert("q", self.q.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap()
            );
        }
        if !self.method.is_empty() {
            params.insert("method", self.method.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap()
            );
        }
        if !self.data_type.is_empty() {
            params.insert("type", self.data_type.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap_or_default()
            );
        }
        if self.sort.is_some() {
            params.insert("sort", self.sort.as_ref().unwrap().to_string());
        }
        if self.id.is_some() {
            params.insert("id", self.id.as_ref().unwrap().clone());
        }
        if self.date.is_some() {
            params.insert("date", self.date.unwrap().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.limit.is_some() {
            params.insert("limit", self.limit.unwrap().to_string());
        }
        if self.offset.is_some() {
            params.insert("offset", self.offset.unwrap().to_string());
        }
        if !self.include.is_empty() {
            params.insert("include", self.include.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap_or_default()
            );
        }

        let query_params = params.into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .reduce(|a, b| format!("{}&{}", a, b))
            .unwrap_or_default();

        query_params
    }

    pub async fn submit(&self) -> Result<response::Response, Box<dyn std::error::Error>> {
        let url = format!("https://ambientCG.com/api/v2/full_json?{}", self.as_query_params());
        let resp = reqwest::get(url).await?.json::<json::Response>().await?;
        Ok(resp.into())
    }
}