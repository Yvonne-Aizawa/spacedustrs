#[cfg(test)]
mod tests {
    use crate::apis::agents_api::get_my_agent;
    use crate::apis::configuration::Configuration;
    use crate::tests::GLOBAL_ACC;

    #[tokio::test]
    async fn test_get_my_agent() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let agent = get_my_agent(&configuration).await;
        assert!(agent.is_ok());
    }
}
