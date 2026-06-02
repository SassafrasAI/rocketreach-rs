use crate::client::RocketReachClient;
use crate::error::Result;
use crate::person::{PersonSearchResult, UniversalPersonLookupRequest, UniversalPersonProfile};
use crate::search::{PersonQuery, SearchRequest};

#[derive(Debug, Clone)]
pub struct UniversalPersonApi {
    client: RocketReachClient,
}

impl UniversalPersonApi {
    pub(crate) fn new(client: RocketReachClient) -> Self {
        Self { client }
    }

    pub async fn lookup_by_id(&self, request: &UniversalPersonLookupRequest) -> Result<UniversalPersonProfile> {
        let mut req = self.client.get("/universal/person/lookup");
        if let Some(ref id) = request.id {
            req = req.query(&[("id", id.to_string())]);
        }
        if let Some(ref name) = request.name {
            req = req.query(&[("name", name.as_str())]);
        }
        if let Some(ref employer) = request.current_employer {
            req = req.query(&[("current_employer", employer.as_str())]);
        }
        if let Some(ref email) = request.email {
            req = req.query(&[("email", email.as_str())]);
        }
        if let Some(ref url) = request.linkedin_url {
            req = req.query(&[("linkedin_url", url.as_str())]);
        }
        if let Some(ref title) = request.title {
            req = req.query(&[("title", title.as_str())]);
        }
        if let Some(npi) = request.npi_number {
            req = req.query(&[("npi_number", npi.to_string())]);
        }
        if let Some(ref webhook_id) = request.webhook_id {
            req = req.query(&[("webhook_id", webhook_id.to_string())]);
        }
        if let Some(cache) = request.return_cached_emails {
            req = req.query(&[("return_cached_emails", cache.to_string())]);
        }
        if let Some(val) = request.reveal_professional_email {
            req = req.query(&[("reveal_professional_email", val.to_string())]);
        }
        if let Some(val) = request.reveal_personal_email {
            req = req.query(&[("reveal_personal_email", val.to_string())]);
        }
        if let Some(val) = request.reveal_phone {
            req = req.query(&[("reveal_phone", val.to_string())]);
        }
        if let Some(val) = request.reveal_detailed_person_enrichment {
            req = req.query(&[("reveal_detailed_person_enrichment", val.to_string())]);
        }
        if let Some(val) = request.reveal_healthcare_enrichment {
            req = req.query(&[("reveal_healthcare_enrichment", val.to_string())]);
        }
        let response = req.send().await?;
        RocketReachClient::handle_response(response).await
    }

    pub async fn check_status(&self, ids: &[u64]) -> Result<Vec<UniversalPersonProfile>> {
        let ids_str: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let response = self.client
            .get("/universal/person/check_status")
            .query(&[("ids", ids_str.join(","))])
            .send()
            .await?;
        RocketReachClient::handle_response_extract_array(response, &["data", "profiles"]).await
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<Vec<PersonSearchResult>> {
        let response = self.client
            .post("/universal/person/search")
            .json(request)
            .send()
            .await?;
        RocketReachClient::handle_response_extract_array(response, &["people", "data", "results"]).await
    }

    pub async fn search_with_query(&self, query: PersonQuery) -> Result<Vec<PersonSearchResult>> {
        let request = SearchRequest::new(query);
        self.search(&request).await
    }
}