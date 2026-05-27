use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LookupStatus {
    Complete,
    Progress,
    Searching,
    #[serde(rename = "not queued")]
    NotQueued,
    Failed,
    Waiting,
}

impl LookupStatus {
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::Progress | Self::Searching | Self::Waiting)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    Relevance,
    Popularity,
    Score,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LookupType {
    Standard,
    Premium,
    #[serde(rename = "premium (feeds disabled)")]
    PremiumFeedsDisabled,
    Bulk,
    Phone,
    Enrich,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Personal,
    Professional,
    Disposable,
    #[serde(rename = "role-based")]
    RoleBased,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SmtpValid {
    Valid,
    Invalid,
    #[serde(rename = "accept-all")]
    AcceptAll,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhoneType {
    Mobile,
    Landline,
    Work,
    Unknown,
    Home,
    Fax,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhoneValidity {
    Valid,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileEmail {
    pub email: String,
    pub smtp_valid: Option<SmtpValid>,
    #[serde(rename = "type")]
    pub email_type: Option<EmailType>,
    pub last_validation_check: Option<String>,
    pub grade: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfilePhone {
    pub number: String,
    pub e164: Option<String>,
    pub country_code: Option<String>,
    pub extension: Option<String>,
    #[serde(rename = "type")]
    pub phone_type: Option<PhoneType>,
    pub validity: Option<PhoneValidity>,
    pub recommended: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Education {
    pub school: Option<String>,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobHistory {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub company: Option<String>,
    pub company_name: Option<String>,
    pub company_id: Option<u64>,
    pub company_linkedin_url: Option<String>,
    pub company_city: Option<String>,
    pub company_region: Option<String>,
    pub company_country_code: Option<String>,
    pub department: Option<String>,
    pub sub_department: Option<String>,
    pub title: Option<String>,
    pub highest_level: Option<String>,
    pub description: Option<String>,
    pub last_updated: Option<String>,
    pub is_current: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NpiData {
    pub npi_number: Option<String>,
    pub credentials: Option<String>,
    pub license_number: Option<String>,
    pub specialization: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileList {
    pub id: u64,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyAddress {
    pub description: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompanyQuarterlyGrowth {
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchTeaser {
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<serde_json::Value>>,
    pub office_phones: Option<Vec<String>>,
    pub preview: Option<Vec<String>>,
    pub is_premium_phone_available: Option<bool>,
    pub personal_emails: Option<Vec<String>>,
    pub professional_emails: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitInfo {
    pub action: Option<String>,
    pub duration: Option<String>,
    pub limit: Option<u64>,
    pub used: Option<u64>,
    pub remaining: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalCreditUsage {
    pub credits_allocated: Option<u64>,
    pub credits_used: Option<u64>,
    pub credits_remaining: Option<u64>,
    pub last_synced: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniversalCreditUsageByAction {
    pub credit_action: Option<String>,
    pub attempted_count: Option<u64>,
    pub succeeded_count: Option<u64>,
    pub credits_used: Option<u64>,
    pub last_synced: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Plan {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub lookup_limit: Option<u64>,
    pub export_limit: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountState {
    Anonymous,
    TestUser,
    Registered,
}