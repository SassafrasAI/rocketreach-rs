use rocketreach_rs::*;
use tokio::time::{sleep, Duration};

fn client() -> RocketReachClient {
    let key = std::env::var("ROCKET_REACH").expect("ROCKET_REACH not set");
    RocketReachClient::new(&key)
}

fn skip_if_no_key() -> bool {
    std::env::var("ROCKET_REACH").is_err()
}

macro_rules! live_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() {
            if skip_if_no_key() {
                eprintln!("Skipping {} — ROCKET_REACH not set", stringify!($name));
                return;
            }
            sleep(Duration::from_secs(1)).await;
            $body
        }
    };
}

// ── Universal Account ────────────────────────────────────────────────

live_test!(live_universal_account_get, {
    let account = client().universal().account().get().await.unwrap();
    eprintln!("✓ universal account get: id={:?}, email={:?}", account.id, account.email);
});

// ── Universal Person ─────────────────────────────────────────────────

live_test!(live_universal_person_lookup_by_email, {
    let request = UniversalPersonLookupRequest {
        email: Some("ajit@rocketreach.co".to_string()),
        ..Default::default()
    };
    let profile = client().universal().person().lookup_by_id(&request).await.unwrap();
    eprintln!("✓ universal person lookup_by_id (email): id={:?}, name={:?}", profile.id, profile.name);
});

live_test!(live_universal_person_lookup_by_name, {
    let request = UniversalPersonLookupRequest {
        name: Some("Ajit Nawalkha".to_string()),
        current_employer: Some("RocketReach".to_string()),
        ..Default::default()
    };
    let profile = client().universal().person().lookup_by_id(&request).await.unwrap();
    eprintln!("✓ universal person lookup_by_id (name): id={:?}, name={:?}", profile.id, profile.name);
});

live_test!(live_universal_person_check_status, {
    let profiles = client().universal().person().check_status(&[4496]).await.unwrap();
    eprintln!("✓ universal person check_status: {} profiles", profiles.len());
});

live_test!(live_universal_person_search, {
    let query = PersonQuery::new().name(vec!["John Smith".to_string()]);
    let request = SearchRequest::new(query).page_size(5);
    let results = client().universal().person().search(&request).await.unwrap();
    eprintln!("✓ universal person search: {} results", results.len());
});

live_test!(live_universal_person_search_with_query, {
    let query = PersonQuery::new()
        .current_title(vec!["CEO".to_string()])
        .company_domain(vec!["rocketreach.co".to_string()]);
    let results = client().universal().person().search_with_query(query).await.unwrap();
    eprintln!("✓ universal person search_with_query: {} results", results.len());
});

// ── Universal Company ────────────────────────────────────────────────

live_test!(live_universal_company_lookup_by_domain, {
    let company = client().universal().company().lookup_by_domain("rocketreach.co").await.unwrap();
    eprintln!("✓ universal company lookup_by_domain: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_universal_company_lookup_by_id, {
    let company = client().universal().company().lookup_by_id(4383).await.unwrap();
    eprintln!("✓ universal company lookup_by_id: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_universal_company_lookup_by_name, {
    let company = client().universal().company().lookup_by_name("RocketReach").await.unwrap();
    eprintln!("✓ universal company lookup_by_name: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_universal_company_lookup_by_linkedin_url, {
    let company = client().universal().company().lookup_by_linkedin_url("https://www.linkedin.com/company/rocketreach").await.unwrap();
    eprintln!("✓ universal company lookup_by_linkedin_url: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_universal_company_lookup_by_ticker, {
    let company = client().universal().company().lookup_by_ticker("GOOG").await.unwrap();
    eprintln!("✓ universal company lookup_by_ticker: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_universal_company_search, {
    let query = CompanyQuery::new().domain(vec!["rocketreach.co".to_string()]);
    let results = client().universal().company().search_with_query(query).await.unwrap();
    eprintln!("✓ universal company search: {} results", results.len());
});