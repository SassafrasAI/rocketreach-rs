use crate::client::RocketReachClient;
use crate::error::Result;
use crate::types::{AccountState, RateLimitInfo, CreditUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub state: Option<AccountState>,
    pub credit_usage: Option<Vec<CreditUsage>>,
    pub rate_limits: Option<Vec<RateLimitInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAccount {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub state: Option<AccountState>,
    pub credit_usage: Option<Vec<CreditUsage>>,
    pub rate_limits: Option<Vec<RateLimitInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            .json(&serde_json::json!({
                "api_key": self.client.api_key
            }))
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }
}