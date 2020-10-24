use std::time::Duration;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::response::{PoeResult, PoeError, ApiErrorResponse};


const API_URL: &'static str = "https://api.pathofexile.com";


pub struct PoeClient {
    client: Client,
    base_url: Url
}

impl PoeClient {
    pub fn new() -> PoeClient {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        PoeClient { client, base_url: Url::parse(API_URL).unwrap() }
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> PoeResult<T> {
        let url = Url::options()
            .base_url(Some(&self.base_url))
            .parse(url)
            .unwrap();

        let response = self.client.get(url)
            .send()
            .await
            .map_err(|err| PoeError::Reqwest(err))?;

        if response.status().is_success() {
            return response.json::<T>().await.map_err(|err| PoeError::Reqwest(err))
        }

        match response.json::<ApiErrorResponse>().await {
            Ok(error) => Err(PoeError::ApiError(error.error)),
            Err(error) => Err(PoeError::Reqwest(error))
        }
    }
}


