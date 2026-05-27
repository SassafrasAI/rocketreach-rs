use crate::client::RocketReachClient;
use crate::error::Result;
use crate::search::{CompanySearchRequest, CompanyQuery};
use crate::types::{CompanyAddress, CompanyQuarterlyGrowth};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub website_domain: Option<String>,
    pub ticker_symbol: Option<String>,
    pub links: Option<CompanyLinks>,
    pub year_founded: Option<i32>,
    pub address: Option<CompanyAddress>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub num_employees: Option<i64>,
    pub revenue: Option<i64>,
    pub funding_investors: Option<Vec<String>>,
    pub industry: Option<String>,
    pub sic_codes: Option<Vec<i32>>,
    pub rr_profile_url: Option<String>,
    pub description: Option<String>,
    pub industry_keywords: Option<Vec<String>>,
    pub naics_codes: Option<Vec<i64>>,
    pub techstack: Option<Vec<String>>,
    pub industries: Option<Vec<String>>,
    pub competitors: Option<Vec<String>>,
    pub departments: Option<serde_json::Map<String, serde_json::Value>>,
    pub company_growth: Option<Vec<CompanyQuarterlyGrowth>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyLinks {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanySearchResult {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub email_domain: Option<String>,
    pub ticker_symbol: Option<String>,
    pub industry_str: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalCompanySearchResult {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub ticker_symbol: Option<String>,
    pub industry_str: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country_code: Option<String>,
    pub linkedin_url: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompanyApi {
    pub(crate) client: RocketReachClient,
}

impl CompanyApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn lookup_by_domain(&self, domain: &str) -> Result<Company> {
        let response = self.client
            .get("/company/lookup/")
            .query(&[("domain", domain)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_id(&self, id: u64) -> Result<Company> {
        let response = self.client
            .get("/company/lookup/")
            .query(&[("id", id.to_string())])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_name(&self, name: &str) -> Result<Company> {
        let response = self.client
            .get("/company/lookup/")
            .query(&[("name", name)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_linkedin_url(&self, url: &str) -> Result<Company> {
        let response = self.client
            .get("/company/lookup/")
            .query(&[("linkedin_url", url)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_ticker(&self, ticker: &str) -> Result<Company> {
        let response = self.client
            .get("/company/lookup/")
            .query(&[("ticker", ticker)])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn search(&self, request: &CompanySearchRequest) -> Result<Vec<CompanySearchResult>> {
        let response = self.client
            .post("/searchCompany")
            .json(request)
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn search_with_query(&self, query: CompanyQuery) -> Result<Vec<CompanySearchResult>> {
        let request = CompanySearchRequest::new(query);
        self.search(&request).await
    }
}