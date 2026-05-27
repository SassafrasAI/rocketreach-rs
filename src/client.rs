use crate::error::{self, Error, Result};

const BASE_URL: &str = "https://api.rocketreach.co/api/v2";

#[derive(Debug, Clone)]
pub struct RocketReachClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}

impl RocketReachClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http = client;
        self
    }

    pub fn account(&self) -> crate::account::AccountApi {
        crate::account::AccountApi::new(self.clone())
    }

    pub fn person(&self) -> crate::person::PersonApi {
        crate::person::PersonApi::new(self.clone())
    }

    pub fn company(&self) -> crate::company::CompanyApi {
        crate::company::CompanyApi::new(self.clone())
    }

    pub fn universal(&self) -> crate::universal::UniversalApi {
        crate::universal::UniversalApi::new(self.clone())
    }

    pub(crate) fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.http
            .get(format!("{}{}", self.base_url, path))
            .header("Api-Key", &self.api_key)
    }

    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.http
            .post(format!("{}{}", self.base_url, path))
            .header("Api-Key", &self.api_key)
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text().await?;

        if status.is_success() {
            serde_json::from_str(&body).map_err(Error::from)
        } else {
            Err(error::map_response_error(status, &headers, &body))
        }
    }
}