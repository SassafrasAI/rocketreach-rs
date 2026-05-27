use rocketreach_rs::*;

#[test]
fn test_person_query_builder() {
    let query = PersonQuery::new()
        .name(vec!["John Doe".to_string()])
        .current_employer(vec!["Acme Corp".to_string()])
        .current_title(vec!["Software Engineer".to_string()])
        .email_grade("A-".to_string());

    assert!(query.name.is_some());
    assert!(query.current_employer.is_some());
    assert!(query.current_title.is_some());
    assert!(query.email_grade.is_some());
    assert!(query.geo.is_none());
}

#[test]
fn test_company_query_builder() {
    let query = CompanyQuery::new()
        .name(vec!["Acme Corp".to_string()])
        .domain(vec!["acme.com".to_string()])
        .industry(vec!["Software".to_string()]);

    assert!(query.name.is_some());
    assert!(query.domain.is_some());
    assert!(query.industry.is_some());
    assert!(query.employees.is_none());
}

#[test]
fn test_search_request_builder() {
    let query = PersonQuery::new()
        .keyword(vec!["Rust".to_string()]);
    let request = SearchRequest::new(query)
        .start(1)
        .page_size(25)
        .order_by(OrderBy::Popularity);

    assert_eq!(request.start, Some(1));
    assert_eq!(request.page_size, Some(25));
    assert!(request.order_by.is_some());
}

#[test]
fn test_company_search_request_builder() {
    let query = CompanyQuery::new()
        .domain(vec!["rocketreach.co".to_string()]);
    let request = CompanySearchRequest::new(query)
        .page_size(50);

    assert_eq!(request.page_size, Some(50));
}

#[test]
fn test_person_query_serialization() {
    let query = PersonQuery::new()
        .name(vec!["John".to_string()])
        .current_title(vec!["Engineer".to_string()]);

    let json = serde_json::to_string(&query).unwrap();
    assert!(json.contains("\"name\""));
    assert!(json.contains("\"current_title\""));
    assert!(!json.contains("\"geo\""));
}

#[test]
fn test_company_query_serialization() {
    let query = CompanyQuery::new()
        .domain(vec!["example.com".to_string()])
        .employees(vec!["51-200".to_string()]);

    let json = serde_json::to_string(&query).unwrap();
    assert!(json.contains("\"domain\""));
    assert!(json.contains("\"employees\""));
}

#[test]
fn test_deserialize_lookup_status() {
    let status: LookupStatus = serde_json::from_str("\"complete\"").unwrap();
    assert_eq!(status, LookupStatus::Complete);
    assert!(status.is_complete());
    assert!(!status.is_in_progress());

    let status: LookupStatus = serde_json::from_str("\"progress\"").unwrap();
    assert!(status.is_in_progress());

    let status: LookupStatus = serde_json::from_str("\"not queued\"").unwrap();
    assert_eq!(status, LookupStatus::NotQueued);
}

#[test]
fn test_deserialize_order_by() {
    let ob: OrderBy = serde_json::from_str("\"relevance\"").unwrap();
    assert_eq!(ob, OrderBy::Relevance);

    let ob: OrderBy = serde_json::from_str("\"popularity\"").unwrap();
    assert_eq!(ob, OrderBy::Popularity);

    let ob: OrderBy = serde_json::from_str("\"score\"").unwrap();
    assert_eq!(ob, OrderBy::Score);
}

#[test]
fn test_deserialize_email() {
    let json = r#"{"email":"test@example.com","smtp_valid":"valid","type":"professional","grade":"A-"}"#;
    let email: ProfileEmail = serde_json::from_str(json).unwrap();
    assert_eq!(email.email, "test@example.com");
    assert_eq!(email.grade, Some("A-".to_string()));
}

#[test]
fn test_deserialize_phone() {
    let json = r#"{"number":"+1 234-555-6789","e164":"+12345556789","type":"mobile","validity":"valid","recommended":true}"#;
    let phone: ProfilePhone = serde_json::from_str(json).unwrap();
    assert_eq!(phone.number, "+1 234-555-6789");
    assert!(phone.recommended.unwrap_or(false));
}

#[test]
fn test_error_variants() {
    let err = Error::Api {
        status: 400,
        message: "bad request".to_string(),
        request_id: None,
    };
    assert!(err.to_string().contains("400"));
    assert!(err.to_string().contains("bad request"));

    let err = Error::RateLimited { request_id: Some("abc".to_string()) };
    assert!(err.to_string().contains("Rate limited"));
    assert_eq!(err.request_id(), Some("abc"));

    let err = Error::Unauthorized { request_id: None };
    assert!(err.to_string().contains("Unauthorized"));
    assert!(err.request_id().is_none());
}

#[test]
fn test_universal_person_lookup_request_fields() {
    let req = UniversalPersonLookupRequest {
        id: Some(12345),
        name: Some("John Doe".to_string()),
        current_employer: Some("Acme".to_string()),
        reveal_professional_email: Some(true),
        reveal_phone: Some(true),
        ..Default::default()
    };
    assert_eq!(req.id, Some(12345));
    assert!(req.reveal_professional_email.unwrap_or(false));
    assert!(req.reveal_phone.unwrap_or(false));
    assert!(req.email.is_none());
}

#[test]
fn test_person_profile_deserialization() {
    let json = r#"{
        "id": 12345,
        "status": "complete",
        "name": "John Doe",
        "current_title": "Software Engineer",
        "current_employer": "Acme Corp",
        "connections": 500
    }"#;
    let profile: PersonProfile = serde_json::from_str(json).unwrap();
    assert_eq!(profile.id, Some(12345));
    assert_eq!(profile.name, Some("John Doe".to_string()));
    assert_eq!(profile.current_title, Some("Software Engineer".to_string()));
    assert_eq!(profile.connections, Some(500));
}

#[test]
fn test_company_deserialization() {
    let json = r#"{
        "id": 67890,
        "name": "Acme Corp",
        "domain": "acme.com",
        "num_employees": 500,
        "revenue": 10000000
    }"#;
    let company: Company = serde_json::from_str(json).unwrap();
    assert_eq!(company.id, Some(67890));
    assert_eq!(company.name, Some("Acme Corp".to_string()));
    assert_eq!(company.num_employees, Some(500));
}