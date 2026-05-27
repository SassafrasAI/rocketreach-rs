use crate::client::RocketReachClient;
use crate::error::Result;
use crate::types::{AccountState, Plan, RateLimitInfo, UniversalCreditUsage, UniversalCreditUsageByAction};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub state: Option<AccountState>,
    pub plan: Option<Plan>,
    pub api_key: Option<String>,
    pub api_key_domain: Option<String>,
    pub daily_api_num_calls: Option<u64>,
    pub daily_api_limit: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalAccount {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub state: Option<AccountState>,
    pub plan: Option<Plan>,
    pub api_key: Option<String>,
    pub api_key_domain: Option<String>,
    pub daily_api_num_calls: Option<u64>,
    pub daily_api_limit: Option<String>,
    pub credit_usage: Option<UniversalCreditUsage>,
    pub credit_usage_by_action: Option<Vec<UniversalCreditUsageByAction>>,
    pub rate_limits: Option<Vec<RateLimitInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeyResponse {
    pub api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AccountApi {
    pub(crate) client: RocketReachClient,
}

impl AccountApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Account> {
        let response = self.client.get("/account/").send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn create_api_key(&self) -> Result<ApiKeyResponse> {
        let response = self.client
            .post("/account/key/")
            .json(&serde_json::json!({}))
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }
}