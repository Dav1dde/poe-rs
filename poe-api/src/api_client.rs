use crate::api::*;
use crate::client::PoeClient;
use crate::response::PoeResult;

const WEB_DOMAIN: &str = "https://www.pathofexile.com";

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
            "{}/character-window/get-items?accountName={}&character={}",
            WEB_DOMAIN, account_name, character
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
            "{}/character-window/get-passive-skills?accountName={}&character={}&reqData={}",
            WEB_DOMAIN,
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
