#[cfg(test)]
mod tests {
    use crate::apis::contracts_api::{get_contract, get_contracts};
    use crate::apis::configuration::Configuration;
    use crate::tests::GLOBAL_ACC;

    #[tokio::test]
    async fn test_get_contracts() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let contracts = get_contracts(&configuration, Some(1), Some(20)).await;
        assert!(contracts.is_ok());
    }
    #[tokio::test]
    async fn test_get_contract() {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(GLOBAL_ACC.to_string());
        let contracts = get_contracts(&configuration, Some(1), Some(20)).await;
        let contract = get_contract(&configuration,&contracts.unwrap().data[0].id).await;


        assert!(contract.is_ok());
    }
}
