#[cfg(test)]
mod tests {
    use crate::apis::configuration::Configuration;
    use crate::apis::factions_api::{get_faction, get_factions};
    use crate::tests::GLOBAL_ACC;

    #[tokio::test]
    async fn test_get_faction() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let faction = get_faction(&configuration, "COSMIC").await;
        assert!(faction.is_ok());
    }
    #[tokio::test]
    async fn test_get_factions() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let factions = get_factions(&configuration, Some(1), Some(20)).await;
        assert!(factions.is_ok());
    }
}
