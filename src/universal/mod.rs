mod account;
mod company;
mod person;

pub use account::UniversalAccountApi;
pub use company::UniversalCompanyApi;
pub use person::UniversalPersonApi;

use crate::client::RocketReachClient;

#[derive(Debug, Clone)]
pub struct UniversalApi {
    client: RocketReachClient,
}

impl UniversalApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub fn account(&self) -> UniversalAccountApi {
        UniversalAccountApi::new(self.client.clone())
    }

    pub fn person(&self) -> UniversalPersonApi {
        UniversalPersonApi::new(self.client.clone())
    }

    pub fn company(&self) -> UniversalCompanyApi {
        UniversalCompanyApi::new(self.client.clone())
    }
}