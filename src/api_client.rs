use std::sync::Arc;

use crate::api::*;
use crate::client::PoeClient;
use crate::response::PoeResult;

const WEB_DOMAIN: &str = "https://www.pathofexile.com";

/// A builder to construct a configured [`PathOfExile`] client.
pub struct PathOfExileBuilder {
    application: (String, String),
    contact: Option<String>,
}

impl PathOfExileBuilder {
    fn new() -> Self {
        Self {
            application: ("poe-rs".to_string(), env!("CARGO_PKG_VERSION").to_string()),
            contact: None,
        }
    }

    /// Sets the application name and version. Defaults to the `poe-rs/{crate version}`.
    ///
    /// This information will be included in the User-Agent of the request.
    pub fn application(mut self, name: impl Into<String>, version: impl Into<String>) -> Self {
        self.application = (name.into(), version.into());
        self
    }

    /// Sets contact information for this client.
    ///
    /// This information will be included in the User-Agent of the request.
    pub fn contact(mut self, contact: impl Into<String>) -> Self {
        self.contact = Some(contact.into());
        self
    }

    /// Builds a [`PathOfExile`] which can be used to make API requests.
    pub fn build(self) -> PathOfExile {
        let mut client = PoeClient::new();

        let mut user_agent = format!("{}/{}", self.application.0, self.application.1);
        if let Some(contact) = self.contact.as_ref() {
            user_agent.push_str(" (contact: ");
            user_agent.push_str(contact);
            user_agent.push(')');
        }
        client.user_agent(user_agent);

        client.into()
    }
}

/// A client to make Path of Exile API requests with.
///
/// You should use [`PathOfExileBuilder`] through `PathOfExile::builder` to create a `PathOfExile`
/// client.
///
/// You do **not** have to wrap the `PathOfExile` client in an [`std::rc::Rc`]
/// or [`Arc`] to **reuse** it, because it already uses an [`Arc`] internally.
#[derive(Clone)]
pub struct PathOfExile {
    client: Arc<PoeClient>,
}

impl Default for PathOfExile {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PoeClient> for PathOfExile {
    fn from(client: PoeClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}

impl PathOfExile {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> PathOfExileBuilder {
        PathOfExileBuilder::new()
    }

    pub async fn get_characters(
        &self,
        account_name: impl AsRef<str>,
    ) -> PoeResult<Vec<CharacterInfo>> {
        let url = &format!(
            "{}/character-window/get-characters?accountName={}",
            WEB_DOMAIN,
            account_name.as_ref()
        );

        self.client.get("get_characters", url).await
    }

    pub async fn get_items(
        &self,
        account_name: impl AsRef<str>,
        character: impl AsRef<str>,
    ) -> PoeResult<ItemsResponse> {
        let url = &format!(
            "{}/character-window/get-items?accountName={}&character={}",
            WEB_DOMAIN,
            account_name.as_ref(),
            character.as_ref()
        );

        self.client.get("get_items", url).await
    }

    pub async fn get_passives(
        &self,
        account_name: impl AsRef<str>,
        character: impl AsRef<str>,
        skill_tree_data: bool,
    ) -> PoeResult<PassivesResponse> {
        let url = &format!(
            "{}/character-window/get-passive-skills?accountName={}&character={}&reqData={}",
            WEB_DOMAIN,
            account_name.as_ref(),
            character.as_ref(),
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
        name: impl AsRef<str>,
        limit: usize,
        offset: usize,
    ) -> PoeResult<LadderResponse> {
        self.client
            .get(
                "ladder",
                &format!(
                    "/ladders/{}?limit={}&offset={}",
                    name.as_ref(),
                    limit,
                    offset
                ),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::PathOfExile;

    #[tokio::test]
    async fn get_characters() {
        let poe = PathOfExile::new();

        let characters = poe.get_characters("Steelmage").await.unwrap();
        characters.iter().find(|c| c.name == "SteelDD").unwrap();
    }

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
        let poe = PathOfExile::new();

        let ladder = poe.ladder("Standard", 1, 0).await.unwrap();

        assert_eq!(15000, ladder.total);
        assert_eq!(1, ladder.entries.len());
        assert_eq!(1, ladder.entries.get(0).unwrap().rank);
    }

    #[ignore]
    #[tokio::test]
    async fn ladder_rate_limit() {
        let poe = PathOfExile::new();

        let n = 6;

        let mut threads = Vec::with_capacity(n);
        for _ in 0..n {
            let poe = poe.clone();
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
