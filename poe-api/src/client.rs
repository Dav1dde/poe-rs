use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::collections::{HashMap, LinkedList};
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore, SemaphorePermit};
use tokio::time::sleep_until;

use crate::response::{ApiErrorResponse, PoeError, PoeResult};

const API_URL: &str = "https://api.pathofexile.com";

pub struct PoeClient {
    client: Client,
    base_url: Url,
    rate_limiter: RateLimiter,
}

impl Default for PoeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PoeClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        PoeClient {
            client,
            base_url: Url::parse(API_URL).unwrap(),
            rate_limiter: RateLimiter::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, call_id: &str, url: &str) -> PoeResult<T> {
        let url = Url::options()
            .base_url(Some(&self.base_url))
            .parse(url)
            .unwrap();

        self.execute(call_id, self.client.get(url)).await
    }

    pub async fn post<T: DeserializeOwned, Body: Serialize + ?Sized>(
        &self,
        call_id: &str,
        url: &str,
        body: &Body,
    ) -> PoeResult<T> {
        let url = Url::options()
            .base_url(Some(&self.base_url))
            .parse(url)
            .unwrap();

        self.execute(call_id, self.client.post(url).json(body))
            .await
    }

    async fn execute<T: DeserializeOwned>(
        &self,
        call_id: &str,
        request: reqwest::RequestBuilder,
    ) -> PoeResult<T> {
        let response = self
            .rate_limiter
            .rate_limited(call_id, async { request.send().await })
            .await
            .map_err(PoeError::from)?;

        if response.status().is_success() {
            return response
                .text()
                .await
                .map_err(PoeError::from)
                .and_then(|raw| {
                    // strip BOM, which is sometimes included
                    serde_json::from_str::<T>(raw.trim_start_matches('\u{feff}'))
                        .map_err(PoeError::from)
                });
        }

        match response.json::<ApiErrorResponse>().await {
            Ok(error) => Err(PoeError::from(error.error)),
            Err(error) => Err(PoeError::from(error)),
        }
    }
}

#[derive(Debug)]
struct History {
    calls: Arc<std::sync::Mutex<LinkedList<Instant>>>,
    duration: Duration,
    limit: usize,
    tickets: Semaphore,
    active: AtomicUsize,
}

impl History {
    fn new() -> History {
        History {
            calls: Arc::new(std::sync::Mutex::new(LinkedList::new())),
            duration: Duration::from_secs(5),
            limit: 5,
            tickets: Semaphore::new(5),
            active: AtomicUsize::new(0),
        }
    }

    // fn from(header: &str) -> History {
    //     let mut limit_header = header
    //         .split(',')
    //         .next().unwrap()
    //         .split(':')
    //         .map(|val| val.parse::<u32>().unwrap());

    //     let limit = limit_header.next().unwrap();
    //     let duration = limit_header.next().unwrap();

    //     History {
    //         calls: Arc::new(Mutex::new(LinkedList::new())),
    //         duration: Duration::from_secs(duration as u64),
    //         limit,
    //         tickets: Semaphore::new(limit as usize),
    //         tickets2: AtomicUsize::new(0)
    //     }
    // }

    fn done(&self) {
        self.calls.lock().unwrap().push_back(Instant::now());
    }

    fn failed(&self) {
        self.active.fetch_sub(1, Ordering::Relaxed);
    }

    async fn wait(&self) -> SemaphorePermit<'_> {
        let ticket = self.tickets.acquire().await.unwrap();

        loop {
            let mut wait_time = None;

            if self.active.fetch_add(1, Ordering::SeqCst) < self.limit {
                return ticket;
            } else {
                self.active.fetch_sub(1, Ordering::SeqCst);

                let mut calls = self.calls.lock().unwrap();
                if let Some(time) = calls.front() {
                    let x = Instant::now() - self.duration;
                    if x > *time {
                        calls.pop_front();
                        self.active.fetch_sub(1, Ordering::SeqCst);
                    } else {
                        wait_time = Some(tokio::time::Instant::from_std(*time + self.duration));
                    }
                }

                drop(calls);
            }

            if let Some(time) = wait_time {
                sleep_until(time).await;
            }
        }
    }
}

#[derive(Debug)]
struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, History>>>,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn rate_limited(
        &self,
        call_id: &str,
        api_call: impl Future<Output = reqwest::Result<reqwest::Response>>,
    ) -> reqwest::Result<reqwest::Response> {
        if !self.limits.read().await.contains_key(call_id) {
            self.limits
                .write()
                .await
                .entry(call_id.to_string())
                .or_insert_with(History::new);
        }

        let limits = self.limits.read().await;
        let history = limits.get(call_id).unwrap();
        let _ticket = history.wait().await;

        match api_call.await {
            Ok(result) => {
                history.done();
                Ok(result)
            }
            Err(err) => {
                history.failed();
                Err(err)
            }
        }
    }
}
