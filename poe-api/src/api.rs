use crate::client::PoeClient;
use crate::response::PoeResult;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsResponse {
    pub items: Vec<Item>,
    pub character: Character,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub ascendancy_class: u32,
    pub class: String,
    pub class_id: u32,
    pub experience: u64,
    #[serde(default)]
    pub last_active: bool,
    pub league: String,
    pub level: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: u8,
    pub attr: String,
    pub s_colour: SocketColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketColor {
    B,
    G,
    R,
    W,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassivesResponse {
    pub hashes: Vec<u32>,
    pub items: Vec<Item>,
    #[serde(rename = "skillTreeData")]
    pub skill_tree_data: Option<SkillTreeData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeData {
    pub assets: HashMap<String, HashMap<String, String>>,
    pub classes: Vec<SkillTreeClass>,
    pub constants: SkillTreeConstants,
    #[serde(rename = "extraImages")]
    pub extra_images: HashMap<String, SkillTreeExtraImage>,
    pub groups: HashMap<String, SkillTreeGroup>,
    #[serde(rename = "imageZoomLevels")]
    pub image_zoom_levels: Vec<f32>,
    #[serde(rename = "jewelSlots")]
    pub jewel_slots: Vec<u32>,
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub nodes: HashMap<String, SkillTreeNode>,
    #[serde(rename = "skillSprites")]
    pub skill_sprites: HashMap<String, Vec<SkillTreeSprite>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeClass {
    pub name: String,
    pub base_str: u32,
    pub base_dex: u32,
    pub base_int: u32,
    pub ascendancies: Vec<SkillTreeAscendancy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeAscendancy {
    pub id: String,
    pub name: String,
    #[serde(rename = "flavourText")]
    pub flavour_text: Option<String>,
    #[serde(rename = "flavourTextColour")]
    pub flavour_text_colour: Option<String>,
    #[serde(rename = "flavourTextRect")]
    pub flavour_text_rect: Option<Rect>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeConstants {
    pub classes: HashMap<String, u32>,
    #[serde(rename = "characterAttributes")]
    pub character_attributes: HashMap<String, u32>,
    #[serde(rename = "PSSCentreInnerRadius")]
    pub pss_centre_inner_radius: u32,
    #[serde(rename = "skillsPerOrbit")]
    pub skills_per_orbit: Vec<u32>,
    #[serde(rename = "orbitRadii")]
    pub orbit_radii: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeExtraImage {
    pub x: f32,
    pub y: f32,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTreeGroup {
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub is_proxy: bool,
    pub orbits: Vec<u32>,
    pub nodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTreeNode {
    // missing on root node
    #[serde(default)]
    pub skill: u32,
    // missing on root node
    #[serde(default)]
    pub name: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_mastery: bool,
    #[serde(default)]
    pub is_notable: bool,
    #[serde(default)]
    pub is_keystone: bool,
    #[serde(default)]
    pub is_blighted: bool,
    #[serde(default)]
    pub is_jewel_socket: bool,
    pub expansion_jewel: Option<ExpansionJewel>,
    #[serde(default)]
    pub is_multiple_choice: bool,
    #[serde(default)]
    pub recipe: Vec<String>,
    #[serde(default)]
    pub granted_passive_points: u32,
    #[serde(default)]
    pub stats: Vec<String>,
    pub class_start_index: Option<u32>,
    #[serde(default)]
    pub reminder_text: Vec<String>,
    #[serde(default)]
    pub grantend_dexterity: u32,
    #[serde(default)]
    pub grantend_intelligence: u32,
    #[serde(default)]
    pub grantend_strength: u32,
    #[serde(default)]
    pub is_ascendancy_start: bool,
    pub ascendancy_name: Option<String>,
    pub orbit: Option<u32>,
    pub orbit_index: Option<u32>,
    #[serde(default)]
    pub out: Vec<String>,
    #[serde(default)]
    pub r#in: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpansionJewel {
    pub size: u32,
    pub index: u32,
    pub proxy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeSprite {
    pub filename: String,
    pub coords: HashMap<String, SkillTreeSpriteCoords>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeSpriteCoords {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueRule {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LadderResponse {
    pub total: usize,
    pub cached_since: DateTime<Utc>,
    pub entries: Vec<LadderEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderEntry {
    pub rank: i32,
    pub dead: bool,
    pub online: bool,
    pub character: LadderEntryCharacter,
    pub account: LadderEntryAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderEntryCharacter {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub class: String,
    pub experience: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub async fn get_passives(
        &self,
        account_name: &str,
        character: &str,
        skill_tree_data: bool,
    ) -> PoeResult<PassivesResponse> {
        let url = &format!(
            "/character-window/get-passive-skills?accountName={}&character={}&reqData={}",
            account_name,
            character,
            if skill_tree_data { 1 } else { 0 }
        );

        self.client.get("get_passives", url).await
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
    async fn get_passives() {
        let poe = PathOfExile::new();

        let passives = poe
            .get_passives("Steelmage", "SteelDD", false)
            .await
            .unwrap();
        assert_eq!(0, passives.hashes.len()); // actually 0 points allocated
        assert!(passives.skill_tree_data.is_none());
    }

    #[tokio::test]
    async fn get_passives_with_data() {
        let poe = PathOfExile::new();

        let passives = poe
            .get_passives("Steelmage", "SteelDD", true)
            .await
            .unwrap();
        assert_eq!(0, passives.hashes.len());
        assert!(passives.skill_tree_data.is_some());
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
