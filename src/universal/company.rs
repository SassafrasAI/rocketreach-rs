use crate::client::RocketReachClient;
use crate::company::{Company, UniversalCompanySearchResult};
use crate::error::Result;
use crate::search::{CompanyQuery, CompanySearchRequest};

#[derive(Debug, Clone)]
pub struct UniversalCompanyApi {
    client: RocketReachClient,
}

impl UniversalCompanyApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn lookup_by_domain(&self, domain: &str) -> Result<Company> {
        let response = self.client
            .get("/universal/company/lookup")
            .query(&[("domain", domain)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_id(&self, id: u64) -> Result<Company> {
        let response = self.client
            .get("/universal/company/lookup")
            .query(&[("id", id.to_string())])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_name(&self, name: &str) -> Result<Company> {
        let response = self.client
            .get("/universal/company/lookup")
            .query(&[("name", name)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_linkedin_url(&self, url: &str) -> Result<Company> {
        let response = self.client
            .get("/universal/company/lookup")
            .query(&[("linkedin_url", url)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_ticker(&self, ticker: &str) -> Result<Company> {
        let response = self.client
            .get("/universal/company/lookup")
            .query(&[("ticker", ticker)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn search(&self, request: &CompanySearchRequest) -> Result<Vec<UniversalCompanySearchResult>> {
        let response = self.client
            .post("/universal/company/search")
            .json(request)
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn search_with_query(&self, query: CompanyQuery) -> Result<Vec<UniversalCompanySearchResult>> {
        let request = CompanySearchRequest::new(query);
        self.search(&request).await
    }
}