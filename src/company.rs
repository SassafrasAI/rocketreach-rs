use crate::client::RocketReachClient;
use crate::error::Result;
use crate::search::{CompanySearchRequest, CompanyQuery};
use crate::types::{deserialize_int_or_string, deserialize_or_empty_vec, deserialize_string_or_vec, CompanyAddress, CompanyQuarterlyGrowth};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    pub id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub email_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub website_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub ticker_symbol: Option<String>,
    #[serde(default)]
    pub links: Option<CompanyLinks>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub year_founded: Option<String>,
    #[serde(default)]
    pub address: Option<CompanyAddress>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub fax: Option<String>,
    pub num_employees: Option<i64>,
    pub revenue: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub funding_investors: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub industry: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub sic_codes: Option<Vec<i32>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub rr_profile_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub industry_keywords: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub naics_codes: Option<Vec<i64>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub techstack: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub industries: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub competitors: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub company_growth: Option<Vec<CompanyQuarterlyGrowth>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyLinks {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub facebook: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub twitter: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub linkedin: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanySearchResult {
    pub id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub email_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub ticker_symbol: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub industry_str: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalCompanySearchResult {
    pub id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub ticker_symbol: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub industry_str: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub linkedin_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
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
        RocketReachClient::handle_response_extract_array(response, &["companies", "data", "results"]).await
    }

    pub async fn search_with_query(&self, query: CompanyQuery) -> Result<Vec<CompanySearchResult>> {
        let request = CompanySearchRequest::new(query);
        self.search(&request).await
    }
}