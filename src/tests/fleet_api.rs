#[cfg(test)]
mod tests {
    use crate::apis::fleet_api::{get_my_ships, get_my_ship, get_my_ship_cargo, get_ship_cooldown, get_ship_nav};
    use crate::apis::configuration::Configuration;
    use crate::tests::GLOBAL_ACC;
    #[tokio::test]
    async fn test_get_my_ships() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let ships = get_my_ships(&configuration, Some(1), Some(20)).await;
        assert!(ships.is_ok());
    }
    #[tokio::test]
    async fn test_get_my_ship() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let ships = get_my_ships(&configuration, Some(1), Some(20)).await;
        let ship = get_my_ship(&configuration,&ships.unwrap().data[0].symbol).await;
        assert!(ship.is_ok());
    }
    #[tokio::test]
    async fn test_get_my_ship_cargo() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let ships = get_my_ships(&configuration, Some(1), Some(20)).await;
        let cargo = get_my_ship_cargo(&configuration,&ships.unwrap().data[0].symbol).await;
        assert!(cargo.is_ok());
    }
    #[tokio::test]
    async fn test_get_ship_cooldown() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let ships = get_my_ships(&configuration, Some(1), Some(20)).await;
        let cooldown = get_ship_cooldown(&configuration,&ships.unwrap().data[0].symbol).await;
        // !TODO this will error since there is no cooldown since that the ship has not extracted anything
        assert!(cooldown.is_err());
    }
    #[tokio::test]
    async fn test_get_ship_nav() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let ships = get_my_ships(&configuration, Some(1), Some(20)).await;
        let nav = get_ship_nav(&configuration,&ships.unwrap().data[0].symbol).await;
        assert!(nav.is_ok());
    }
}
