use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::{empty_array_is_map, is_false, is_zero, string_or_u32};

pub use std::collections::HashMap as Map;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ItemsResponse {
    pub items: Vec<Item>,
    pub character: CharacterInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub verified: bool,
    pub w: u8,
    pub h: u8,
    pub icon: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub support: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stack_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size_text: Option<String>,
    #[serde(default)]
    pub league: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub influences: Map<String, bool>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub elder: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub shaper: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub abyss_jewel: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub delve: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub fractured: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub synthesised: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sockets: Vec<ItemSocket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub socketed_items: Vec<Item>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    pub identified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_level: Option<i32>,
    pub ilvl: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub locked_to_character: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub locked_to_account: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub duplicated: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub corrupted: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub cis_reward: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub sea_race_reward: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub th_race_reward: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<ItemProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talisman_tier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec_descr_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub utility_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicit_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explicit_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub crafted_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enchant_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fractured_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cosmetic_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub veiled_mods: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub veiled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descr_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flavour_text: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flavour_text_parsed: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prophecy_text: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_relic: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub replica: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incubated_item: Option<IncubatedItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<ItemHybrid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,

    // missing from docs for now
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<ItemProperty>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    values: Vec<(String, u32)>,
    display_mode: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncubatedItem {
    name: String,
    level: u32,
    progress: u32, // maybe this is f32?
    total: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemHybrid {
    #[serde(default, skip_serializing_if = "is_false")]
    is_vaal_gem: bool,
    base_type_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    properties: Vec<ItemProperty>,
    explicit_mods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sec_descr_text: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemExtended {
    category: String,
    subcategories: Vec<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    prefixes: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    suffixes: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterInfo {
    pub ascendancy_class: u32,
    pub class: String,
    pub class_id: u32,
    pub experience: u64,
    #[serde(default, skip_serializing_if = "is_false")]
    pub last_active: bool,
    pub league: String,
    pub level: u32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_colour: Option<SocketColor>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocketColor {
    R,
    G,
    B,
    W,
    A,
    #[serde(rename = "UPPER_CASE")]
    Dv,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PassivesResponse {
    pub hashes: Vec<u32>,
    pub items: Vec<Item>,
    #[serde(default)]
    pub hashes_ex: Vec<u32>,
    #[serde(default, deserialize_with = "empty_array_is_map")]
    pub jewel_data: Map<String, JewelData>,
    #[serde(rename = "skillTreeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_tree_data: Option<SkillTreeData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JewelData {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_visual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subgraph: Option<JewelDataSubgraph>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JewelDataSubgraph {
    groups: Map<String, SkillTreeGroup>,
    nodes: Map<String, SkillTreeNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeData {
    /// Optional/missing since 3.18.1/3.19
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub assets: Map<String, Map<String, String>>,
    pub classes: Vec<SkillTreeClass>,
    pub constants: SkillTreeConstants,
    #[serde(rename = "extraImages")]
    pub extra_images: Map<String, SkillTreeExtraImage>,
    pub groups: Map<String, SkillTreeGroup>,
    #[serde(rename = "imageZoomLevels")]
    pub image_zoom_levels: Vec<f32>,
    #[serde(rename = "jewelSlots")]
    pub jewel_slots: Vec<u32>,
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub nodes: Map<String, SkillTreeNode>,
    /// Optional/missing since 3.18.1/3.19
    #[serde(
        default,
        rename = "skillSprites",
        skip_serializing_if = "Map::is_empty"
    )]
    pub skill_sprites: Map<String, Vec<SkillTreeSprite>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeClass {
    pub name: String,
    pub base_str: u32,
    pub base_dex: u32,
    pub base_int: u32,
    pub ascendancies: Vec<SkillTreeAscendancy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeAscendancy {
    pub id: String,
    pub name: String,
    #[serde(rename = "flavourText", skip_serializing_if = "Option::is_none")]
    pub flavour_text: Option<String>,
    #[serde(rename = "flavourTextColour", skip_serializing_if = "Option::is_none")]
    pub flavour_text_colour: Option<String>,
    #[serde(rename = "flavourTextRect", skip_serializing_if = "Option::is_none")]
    pub flavour_text_rect: Option<Rect>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeConstants {
    pub classes: Map<String, u32>,
    #[serde(rename = "characterAttributes")]
    pub character_attributes: Map<String, u32>,
    #[serde(rename = "PSSCentreInnerRadius")]
    pub pss_centre_inner_radius: u32,
    #[serde(rename = "skillsPerOrbit")]
    pub skills_per_orbit: Vec<u32>,
    #[serde(rename = "orbitRadii")]
    pub orbit_radii: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeExtraImage {
    pub x: f32,
    pub y: f32,
    pub image: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTreeGroup {
    pub x: f32,
    pub y: f32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_proxy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    pub orbits: Vec<u32>,
    pub nodes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTreeNode {
    // missing on root node
    #[serde(default, deserialize_with = "string_or_u32")]
    pub skill: u32,
    // missing on root node
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_mastery: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_notable: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_keystone: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_blighted: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_jewel_socket: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion_jewel: Option<ExpansionJewel>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_multiple_choice: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipe: Vec<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub granted_passive_points: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mastery_effects: Vec<MasteryEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_start_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reminder_text: Vec<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub grantend_dexterity: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub grantend_intelligence: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub grantend_strength: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_ascendancy_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascendancy_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub out: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#in: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpansionJewel {
    pub size: u32,
    pub index: u32,
    pub proxy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasteryEffect {
    pub effect: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reminder_text: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeSprite {
    pub filename: String,
    pub coords: Map<String, SkillTreeSpriteCoords>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeSpriteCoords {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub realm: String,
    pub description: String,
    pub register_at: DateTime<Utc>,
    pub url: String,
    pub start_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<DateTime<Utc>>,
    pub delve_event: bool,
    pub rules: Vec<LeagueRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeagueRule {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LadderResponse {
    pub total: usize,
    pub cached_since: DateTime<Utc>,
    pub entries: Vec<LadderEntry>,
}

impl Default for LadderResponse {
    fn default() -> Self {
        Self {
            total: 0,
            cached_since: DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc),
            entries: Vec::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LadderEntry {
    pub rank: i32,
    pub dead: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub online: bool,
    pub character: LadderEntryCharacter,
    pub account: LadderEntryAccount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LadderEntryCharacter {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub class: String,
    pub experience: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LadderEntryAccount {
    pub name: String,
    pub realm: String,
}
