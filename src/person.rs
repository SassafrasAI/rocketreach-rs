use crate::client::RocketReachClient;
use crate::error::Result;
use crate::search::SearchRequest;
use crate::types::{deserialize_int_or_string, deserialize_or_empty_vec, deserialize_string_or_vec};
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
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub profile_pic: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    #[serde(default)]
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub location: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_website: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_linkedin_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_industry: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub job_history: Option<Vec<JobHistory>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub education: Option<Vec<Education>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub skills: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub birth_year: Option<String>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    #[serde(default)]
    pub npi_data: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub tags: Option<String>,
    pub return_cached_emails: Option<bool>,
    pub linkedin_url_active: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_personal_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_professional_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_work_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_personal_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub emails: Option<Vec<ProfileEmail>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub phones: Option<Vec<ProfilePhone>>,
    #[serde(default)]
    pub profile_list: Option<ProfileList>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonSearchResult {
    pub id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub profile_pic: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    #[serde(default)]
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub location: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_website: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_linkedin_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub birth_year: Option<String>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    #[serde(default)]
    pub teaser: Option<SearchTeaser>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub suppressed: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonAndCompanyProfile {
    #[serde(flatten)]
    pub profile: PersonProfile,
    #[serde(default)]
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
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub profile_pic: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub linkedin_url: Option<String>,
    pub connections: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub location: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer: Option<String>,
    pub current_employer_id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_website: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_employer_linkedin_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub birth_year: Option<String>,
    pub region_latitude: Option<f64>,
    pub region_longitude: Option<f64>,
    pub return_cached_emails: Option<bool>,
    pub linkedin_url_active: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub job_history: Option<Vec<JobHistory>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub education: Option<Vec<Education>>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub skills: Option<Vec<String>>,
    #[serde(default)]
    pub links: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default)]
    pub npi_data: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub emails: Option<Vec<ProfileEmail>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_personal_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_personal_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub recommended_professional_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub current_work_email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_or_empty_vec")]
    pub phones: Option<Vec<ProfilePhone>>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
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
        RocketReachClient::handle_response_extract_array(response, &["data", "profiles"]).await
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<Vec<PersonSearchResult>> {
        let response = self.client
            .post("/person/search")
            .json(request)
            .send()
            .await?;
        RocketReachClient::handle_response_extract_array(response, &["people", "data", "results"]).await
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