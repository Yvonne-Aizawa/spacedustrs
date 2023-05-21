
#[cfg(test)]
mod tests {
    use crate::models::register_request::{Faction, RegisterRequest};
    use crate::apis::default_api::register;
    use crate::apis::factions_api::{get_faction, get_factions};
    use crate::apis::configuration::Configuration;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use rand::prelude::*;
    use crate::tests::GLOBAL_AAC;

    #[tokio::test]
    async fn test_get_faction(){
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_AAC.to_string());
        let faction = get_faction(&configuration, "COSMIC").await;
        assert!(faction.is_ok());
 
    }
    #[tokio::test]
    async fn test_get_factions(){
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_AAC.to_string());
        let factions = get_factions(&configuration, Some(1), Some(20)).await;
        assert!(factions.is_ok());
    }

}