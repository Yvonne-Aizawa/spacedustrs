#[cfg(test)]
mod tests {
    use crate::models::register_request::{Faction, RegisterRequest};

    use crate::apis::configuration::Configuration;
    use crate::apis::default_api::get_status;
    use crate::apis::default_api::register;
    use rand::prelude::*;

    #[tokio::test]
    async fn test_register() {
        let mut rng = thread_rng();
        let acc_id = rng.gen_range(0..10000);
        let acc_name = format!("test_{}", acc_id);
        let configuration = Configuration::new();

        let register_request = RegisterRequest {
            faction: Faction::Void,
            symbol: acc_name.to_string(),
            email: None,
        };

        let response = register(&configuration, Some(register_request));
        assert!(&response.await.is_ok());
    }
    #[tokio::test]
    async fn test_get_status() {
        let configuration = Configuration::new();

        let response = get_status(&configuration);
        assert!(&response.await.is_ok());
    }
}
