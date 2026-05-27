use crate::account::UniversalAccount;
use crate::client::RocketReachClient;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct UniversalAccountApi {
    client: RocketReachClient,
}

impl UniversalAccountApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<UniversalAccount> {
        let response = self.client.get("/universal/account/").send().await?;
        RocketReachClient::handle_response(response).await
    }
}