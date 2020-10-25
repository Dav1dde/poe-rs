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
    verified: bool,
    w: u8,
    h: u8,
    ilvl: u8,
    icon: String,
    league: String,
    id: String,
    #[serde(default)]
    elder: bool,
    #[serde(default)]
    shaper: bool,
    #[serde(default)]
    fractured: bool,
    #[serde(default)]
    sockets: Vec<ItemSocket>,
    name: String,
    type_line: String,
    #[serde(default)]
    identified: bool,
    #[serde(default)]
    corrupted: bool,
    // properties
    #[serde(default)]
    utility_mods: Vec<String>,
    #[serde(default)]
    explicit_mods: Vec<String>,
    #[serde(default)]
    crafted_mods: Vec<String>,
    #[serde(default)]
    enchant_mods: Vec<String>,
    #[serde(default)]
    fractured_mods: Vec<String>,
    #[serde(default)]
    flavour_text: Vec<String>,
    #[serde(default)]
    descr_text: String,
    #[serde(default)]
    sec_descr_text: String,
    frame_type: i32,
    // category
    x: i32,
    y: i32,
    inventory_id: String,
    #[serde(default)]
    is_relic: bool,
    // socketetedItems
    #[serde(default)]
    socket: i32,
    #[serde(default)]
    color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    group: u8,
    attr: String,
    s_colour: SocketColor,
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
    id: String,
    realm: String,
    description: String,
    register_at: DateTime<Utc>,
    url: String,
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
    delve_event: bool,
    rules: Vec<LeagueRule>,
}

#[derive(Debug, Deserialize)]
struct LeagueRule {
    id: String,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct LadderResponse {
    total: i32,
    cached_since: DateTime<Utc>,
    entries: Vec<LadderEntry>,
}

#[derive(Debug, Deserialize)]
pub struct LadderEntry {
    rank: i32,
    dead: bool,
    online: bool,
    character: LadderEntryCharacter,
    account: LadderEntryAccount,
}

#[derive(Debug, Deserialize)]
pub struct LadderEntryCharacter {
    id: String,
    name: String,
    level: u32,
    class: String,
    experience: u64,
}

#[derive(Debug, Deserialize)]
pub struct LadderEntryAccount {
    name: String,
    realm: String,
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

    pub async fn leagues(&self, limit: u32, offset: u32) -> PoeResult<Vec<League>> {
        self.client
            .get(
                "leagues",
                &format!("/leagues?limit={}&offset={}", limit, offset),
            )
            .await
    }

    pub async fn ladder(&self, name: &str, limit: u32, offset: u32) -> PoeResult<LadderResponse> {
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
