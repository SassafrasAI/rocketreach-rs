use crate::client::RocketReachClient;
use crate::error::Result;
use crate::search::{SearchRequest};
use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PersonLookupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_employer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_number: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_type: Option<LookupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_cached_emails: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonProfile {
    pub id: Option<u64>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub profile_pic: Option<String>,
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    pub location: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub current_title: Option<String>,
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    pub current_employer_domain: Option<String>,
    pub current_employer_website: Option<String>,
    pub current_employer_linkedin_url: Option<String>,
    pub current_employer_industry: Option<String>,
    pub job_history: Option<Vec<JobHistory>>,
    pub education: Option<Vec<Education>>,
    pub skills: Option<Vec<String>>,
    pub birth_year: Option<u32>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    pub npi_data: Option<serde_json::Value>,
    pub tags: Option<String>,
    pub return_cached_emails: Option<bool>,
    pub linkedin_url_active: Option<bool>,
    pub recommended_email: Option<String>,
    pub recommended_personal_email: Option<String>,
    pub recommended_professional_email: Option<String>,
    pub current_work_email: Option<String>,
    pub current_personal_email: Option<String>,
    pub emails: Option<Vec<ProfileEmail>>,
    pub phones: Option<Vec<ProfilePhone>>,
    pub profile_list: Option<ProfileList>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonSearchResult {
    pub id: Option<u64>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub profile_pic: Option<String>,
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    pub location: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub current_title: Option<String>,
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    pub current_employer_domain: Option<String>,
    pub current_employer_website: Option<String>,
    pub current_employer_linkedin_url: Option<String>,
    pub birth_year: Option<u32>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    pub teaser: Option<SearchTeaser>,
    pub suppressed: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonAndCompanyProfile {
    #[serde(flatten)]
    pub profile: PersonProfile,
    pub company: Option<crate::company::Company>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct UniversalPersonLookupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_employer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi_number: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_cached_emails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_professional_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_personal_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_phone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_detailed_person_enrichment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reveal_healthcare_enrichment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalPersonProfile {
    pub id: Option<u64>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub profile_pic: Option<String>,
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    pub location: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub current_title: Option<String>,
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    pub current_employer_domain: Option<String>,
    pub current_employer_website: Option<String>,
    pub current_employer_linkedin_url: Option<String>,
    pub birth_year: Option<String>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    pub return_cached_emails: Option<bool>,
    pub linkedin_url_active: Option<bool>,
    pub job_history: Option<Vec<JobHistory>>,
    pub education: Option<Vec<Education>>,
    pub skills: Option<Vec<String>>,
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    pub npi_data: Option<serde_json::Value>,
    pub recommended_email: Option<String>,
    pub emails: Option<Vec<ProfileEmail>>,
    pub recommended_personal_email: Option<String>,
    pub current_personal_email: Option<String>,
    pub recommended_professional_email: Option<String>,
    pub current_work_email: Option<String>,
    pub phones: Option<Vec<ProfilePhone>>,
    pub metadata: Option<serde_json::Value>,
    pub profile_list: Option<ProfileList>,
}

#[derive(Debug, Clone)]
pub struct PersonApi {
    pub(crate) client: RocketReachClient,
}

impl PersonApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn lookup_by_id(&self, id: u64, return_cached_emails: Option<bool>) -> Result<PersonProfile> {
        let mut req = self.client.get("/person/lookup").query(&[("id", id.to_string())]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_name(&self, name: &str, current_employer: &str, return_cached_emails: Option<bool>) -> Result<PersonProfile> {
        let mut req = self.client
            .get("/person/lookup")
            .query(&[("name", name), ("current_employer", current_employer)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_email(&self, email: &str, return_cached_emails: Option<bool>) -> Result<PersonProfile> {
        let mut req = self.client
            .get("/person/lookup")
            .query(&[("email", email)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn lookup_by_linkedin_url(&self, url: &str, return_cached_emails: Option<bool>) -> Result<PersonProfile> {
        let mut req = self.client
            .get("/person/lookup")
            .query(&[("linkedin_url", url)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn check_status(&self, ids: &[u64]) -> Result<Vec<PersonProfile>> {
        let ids_str: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let response = self.client
            .get("/person/checkStatus")
            .query(&[("ids", ids_str.join(","))])
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<Vec<PersonSearchResult>> {
        let response = self.client
            .post("/person/search")
            .json(request)
            .send()
            .await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn person_and_company_lookup_by_id(&self, id: u64, return_cached_emails: Option<bool>) -> Result<PersonAndCompanyProfile> {
        let mut req = self.client
            .get("/profile-company/lookup")
            .query(&[("id", id.to_string())]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn person_and_company_lookup_by_name(&self, name: &str, current_employer: &str, return_cached_emails: Option<bool>) -> Result<PersonAndCompanyProfile> {
        let mut req = self.client
            .get("/profile-company/lookup")
            .query(&[("name", name), ("current_employer", current_employer)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn person_and_company_lookup_by_email(&self, email: &str, return_cached_emails: Option<bool>) -> Result<PersonAndCompanyProfile> {
        let mut req = self.client
            .get("/profile-company/lookup")
            .query(&[("email", email)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn person_and_company_lookup_by_linkedin_url(&self, url: &str, return_cached_emails: Option<bool>) -> Result<PersonAndCompanyProfile> {
        let mut req = self.client
            .get("/profile-company/lookup")
            .query(&[("linkedin_url", url)]);
        if let Some(cache) = return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }
}