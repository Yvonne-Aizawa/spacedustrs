#[cfg(test)]
mod tests {
    use crate::apis::systems_api::{get_systems, get_systems_all, get_waypoint};
    use crate::apis::configuration::Configuration;
    use crate::tests::GLOBAL_ACC;

    #[tokio::test]
    async fn test_get_systems() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let systems = get_systems(&configuration, Some(1), Some(2)).await;
        assert!(systems.is_ok());
    }
    #[tokio::test]
    async fn test_get_systems_all() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let systems = get_systems_all(&configuration).await;
        assert!(systems.is_ok());
    }
    #[tokio::test]
    async fn test_get_waypoint(){
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let systems = get_systems(&configuration, Some(1), Some(2)).await;
        let waypoint = &systems.as_ref().unwrap().data[0].waypoints[0];
        let response = get_waypoint(&configuration, &systems.as_ref().unwrap().data[0].symbol,  &waypoint.symbol).await;
        assert!(response.is_ok());
    }
}
