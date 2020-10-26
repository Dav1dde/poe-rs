use crate::client::PoeClient;
use crate::response::PoeResult;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItemsResponse {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub verified: bool,
    pub w: u8,
    pub h: u8,
    pub ilvl: u8,
    pub icon: String,
    pub league: String,
    pub id: String,
    #[serde(default)]
    pub elder: bool,
    #[serde(default)]
    pub shaper: bool,
    #[serde(default)]
    pub fractured: bool,
    #[serde(default)]
    pub sockets: Vec<ItemSocket>,
    pub name: String,
    pub type_line: String,
    #[serde(default)]
    pub identified: bool,
    #[serde(default)]
    pub corrupted: bool,
    // properties
    #[serde(default)]
    pub utility_mods: Vec<String>,
    #[serde(default)]
    pub explicit_mods: Vec<String>,
    #[serde(default)]
    pub crafted_mods: Vec<String>,
    #[serde(default)]
    pub enchant_mods: Vec<String>,
    #[serde(default)]
    pub fractured_mods: Vec<String>,
    #[serde(default)]
    pub flavour_text: Vec<String>,
    #[serde(default)]
    pub descr_text: String,
    #[serde(default)]
    pub sec_descr_text: String,
    pub frame_type: i32,
    // category
    pub x: i32,
    pub y: i32,
    pub inventory_id: String,
    #[serde(default)]
    pub is_relic: bool,
    // socketetedItems
    #[serde(default)]
    pub socket: i32,
    #[serde(default)]
    pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: u8,
    pub attr: String,
    pub s_colour: SocketColor,
}

#[derive(Debug, Deserialize)]
pub enum SocketColor {
    B,
    G,
    R,
    W,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub realm: String,
    pub description: String,
    pub register_at: DateTime<Utc>,
    pub url: String,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub delve_event: bool,
    pub rules: Vec<LeagueRule>,
}

#[derive(Debug, Deserialize)]
pub struct LeagueRule {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct LadderResponse {
    pub total: usize,
    pub cached_since: DateTime<Utc>,
    pub entries: Vec<LadderEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LadderEntry {
    pub rank: i32,
    pub dead: bool,
    pub online: bool,
    pub character: LadderEntryCharacter,
    pub account: LadderEntryAccount,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LadderEntryCharacter {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub class: String,
    pub experience: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LadderEntryAccount {
    pub name: String,
    pub realm: String,
}

pub struct PathOfExile {
    client: PoeClient,
}

impl Default for PathOfExile {
    fn default() -> Self {
        Self::new()
    }
}

impl PathOfExile {
    pub fn new() -> PathOfExile {
        PathOfExile {
            client: PoeClient::new(),
        }
    }

    pub async fn get_items(&self, account_name: &str, character: &str) -> PoeResult<ItemsResponse> {
        let url = &format!(
            "/character-window/get-items?accountName={}&character={}",
            account_name, character
        );

        self.client.get("get_items", url).await
    }

    pub async fn leagues(&self, limit: usize, offset: usize) -> PoeResult<Vec<League>> {
        self.client
            .get(
                "leagues",
                &format!("/leagues?limit={}&offset={}", limit, offset),
            )
            .await
    }

    pub async fn ladder(
        &self,
        name: &str,
        limit: usize,
        offset: usize,
    ) -> PoeResult<LadderResponse> {
        self.client
            .get(
                "ladder",
                &format!("/ladders/{}?limit={}&offset={}", name, limit, offset),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::PathOfExile;
    use std::sync::Arc;

    #[tokio::test]
    async fn get_items() {
        let poe = PathOfExile::new();

        // let's hope he doesn't change or delete the character
        let items = poe.get_items("Steelmage", "SteelDD").await.unwrap();
        assert_eq!(17, items.items.len());
    }

    #[tokio::test]
    async fn leagues() {
        let poe = PathOfExile::new();

        let all = poe.leagues(50, 0).await.unwrap();
        assert_eq!("Standard", all.get(0).unwrap().id);
        assert_eq!("Hardcore", all.get(1).unwrap().id);
        assert_eq!("SSF Standard", all.get(2).unwrap().id);
        assert_eq!("SSF Hardcore", all.get(3).unwrap().id);

        let standard = poe.leagues(1, 0).await.unwrap();
        let hardcore = poe.leagues(1, 1).await.unwrap();

        assert_eq!(standard.len(), 1);
        assert_eq!("Standard", standard.get(0).unwrap().id);
        assert_eq!(hardcore.len(), 1);
        assert_eq!("Hardcore", hardcore.get(0).unwrap().id);

        let ssf = poe.leagues(2, 2).await.unwrap();
        assert_eq!("SSF Standard", ssf.get(0).unwrap().id);
        assert_eq!("SSF Hardcore", ssf.get(1).unwrap().id);
    }

    #[tokio::test]
    async fn ladder() {
        let poe = Arc::new(PathOfExile::new());

        let ladder = poe.ladder("Standard", 1, 0).await.unwrap();

        assert_eq!(15000, ladder.total);
        assert_eq!(1, ladder.entries.len());
        assert_eq!(1, ladder.entries.get(0).unwrap().rank);
    }

    #[ignore]
    #[tokio::test]
    async fn ladder_rate_limit() {
        let poe = Arc::new(PathOfExile::new());

        let n = 6;

        let mut threads = Vec::with_capacity(n);
        for _ in 0..n {
            let poe = Arc::clone(&poe);
            threads.push(tokio::spawn(async move {
                let ladder = poe.ladder("Standard", 1, 0).await.unwrap();
                assert_eq!(15000, ladder.total);
                assert_eq!(1, ladder.entries.len());
                assert_eq!(1, ladder.entries.get(0).unwrap().rank);
            }));
        }

        futures::future::join_all(threads).await;
    }
}
