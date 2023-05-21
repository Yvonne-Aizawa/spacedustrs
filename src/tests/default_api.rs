

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_register(){

        use crate::models::register_request::{Faction, RegisterRequest};
        use crate::apis::default_api::register;
        use crate::apis::configuration::Configuration;
        use tokio::fs::File;
        use tokio::io::AsyncWriteExt;
        use rand::prelude::*;
        let mut rng = thread_rng();
        let acc_id = rng.gen_range(0..10000);
        let acc_name = format!("test_{}",acc_id);
        let configuration = Configuration::new();
        
        let register_request = RegisterRequest {
            faction: Faction::Void,
            symbol: acc_name.to_string(),
            email: None
        };

        let response = register(&configuration,Some(register_request));
        assert!(&response.await.is_ok());
    }
}