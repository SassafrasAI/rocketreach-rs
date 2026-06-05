use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_or_empty_vec<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::Array(arr)) => {
            let items: Vec<T> = arr
                .into_iter()
                .filter_map(|v| T::deserialize(v).ok())
                .collect();
            Ok(Some(items))
        }
        Some(serde_json::Value::Null) => Ok(None),
        Some(_) => Ok(None),
    }
}

pub fn deserialize_int_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(serde_json::Value::Null) => Ok(None),
        Some(v) => Ok(Some(v.to_string())),
    }
}

pub fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        Some(serde_json::Value::Array(arr)) => {
            let items: Vec<String> = arr
                .into_iter()
                .filter_map(|v| match v {
                    serde_json::Value::String(s) => Some(s),
                    other => Some(other.to_string()),
                })
                .collect();
            Ok(Some(items.join(", ")))
        }
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(serde_json::Value::Null) => Ok(None),
        Some(v) => Ok(Some(v.to_string())),
    }
}

pub fn deserialize_string_or_vec_required<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr
                .into_iter()
                .filter_map(|v| match v {
                    serde_json::Value::String(s) => Some(s),
                    other => Some(other.to_string()),
                })
                .collect();
            Ok(items.join(", "))
        }
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Null => Ok(String::new()),
        other => Ok(other.to_string()),
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEmail {
    #[serde(deserialize_with = "deserialize_string_or_vec_required")]
    pub email: String,
    #[serde(default)]
    pub smtp_valid: Option<SmtpValid>,
    #[serde(default, rename = "type")]
    pub email_type: Option<EmailType>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub last_validation_check: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub grade: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePhone {
    #[serde(deserialize_with = "deserialize_string_or_vec_required")]
    pub number: String,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub e164: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub extension: Option<String>,
    #[serde(default, rename = "type")]
    pub phone_type: Option<PhoneType>,
    #[serde(default)]
    pub validity: Option<PhoneValidity>,
    #[serde(default)]
    pub recommended: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub school: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub degree: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub major: Option<String>,
    #[serde(default)]
    pub start: Option<u32>,
    #[serde(default)]
    pub end: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobHistory {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub start_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub end_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company_name: Option<String>,
    #[serde(default)]
    pub company_id: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company_linkedin_url: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company_city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company_region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub company_country_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub department: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub sub_department: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub highest_level: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub is_current: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpiData {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub npi_number: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub credentials: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub license_number: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub specialization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileList {
    pub id: u64,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAddress {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub street: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub region_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub postal_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyQuarterlyGrowth {
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default)]
    pub quarter: Option<i32>,
    #[serde(default)]
    pub values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTeaser {
    #[serde(default)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub phones: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub office_phones: Option<Vec<String>>,
    #[serde(default)]
    pub preview: Option<Vec<String>>,
    #[serde(default)]
    pub is_premium_phone_available: Option<bool>,
    #[serde(default)]
    pub personal_emails: Option<Vec<String>>,
    #[serde(default)]
    pub professional_emails: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub limit: Option<String>,
    #[serde(default)]
    pub used: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub remaining: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditUsage {
    #[serde(default)]
    pub credit_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub allocated: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub used: Option<String>,
    #[serde(default, deserialize_with = "deserialize_int_or_string")]
    pub remaining: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountState {
    Anonymous,
    TestUser,
    Registered,
}