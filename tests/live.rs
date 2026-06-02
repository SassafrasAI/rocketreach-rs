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

// ── Account ──────────────────────────────────────────────────────────

live_test!(live_account_get, {
    let account = client().account().get().await.unwrap();
    eprintln!("✓ account get: id={:?}, email={:?}", account.id, account.email);
});

// ── Person ───────────────────────────────────────────────────────────

live_test!(live_person_lookup_by_id, {
    let profile = client().person().lookup_by_id(4496, None).await.unwrap();
    eprintln!("✓ person lookup_by_id: id={:?}, name={:?}", profile.id, profile.name);
});

live_test!(live_person_lookup_by_name, {
    match client().person().lookup_by_name("Ajit Nawalkha", "RocketReach", None).await {
        Ok(profile) => eprintln!("✓ person lookup_by_name: id={:?}, name={:?}", profile.id, profile.name),
        Err(e) => eprintln!("⊘ person lookup_by_name: {}", e),
    }
});

live_test!(live_person_lookup_by_email, {
    match client().person().lookup_by_email("ajit@rocketreach.co", None).await {
        Ok(profile) => eprintln!("✓ person lookup_by_email: id={:?}, name={:?}", profile.id, profile.name),
        Err(e) => eprintln!("⊘ person lookup_by_email: {}", e),
    }
});

live_test!(live_person_lookup_by_linkedin_url, {
    let profile = client().person().lookup_by_linkedin_url("https://www.linkedin.com/in/scottmillar", None).await.unwrap();
    eprintln!("✓ person lookup_by_linkedin_url: id={:?}, name={:?}", profile.id, profile.name);
});

live_test!(live_person_check_status, {
    match client().person().check_status(&[4496]).await {
        Ok(profiles) => eprintln!("✓ person check_status: {} profiles", profiles.len()),
        Err(e) => eprintln!("⊘ person check_status: {}", e),
    }
});

live_test!(live_person_search, {
    let query = PersonQuery::new()
        .name(vec!["John Smith".to_string()])
        .current_title(vec!["Software Engineer".to_string()]);
    let request = SearchRequest::new(query).page_size(5);
    let results = client().person().search(&request).await.unwrap();
    eprintln!("✓ person search: {} results", results.len());
});

// ── Person + Company ─────────────────────────────────────────────────

live_test!(live_person_and_company_lookup_by_id, {
    let profile = client().person().person_and_company_lookup_by_id(4496, None).await.unwrap();
    eprintln!("✓ person+company lookup_by_id: name={:?}, company={:?}",
        profile.profile.name,
        profile.company.as_ref().and_then(|c| c.name.clone()),
    );
});

live_test!(live_person_and_company_lookup_by_name, {
    match client().person().person_and_company_lookup_by_name("Ajit Nawalkha", "RocketReach", None).await {
        Ok(profile) => eprintln!("✓ person+company lookup_by_name: name={:?}, company={:?}",
            profile.profile.name,
            profile.company.as_ref().and_then(|c| c.name.clone()),
        ),
        Err(e) => eprintln!("⊘ person+company lookup_by_name: {}", e),
    }
});

live_test!(live_person_and_company_lookup_by_email, {
    match client().person().person_and_company_lookup_by_email("ajit@rocketreach.co", None).await {
        Ok(profile) => eprintln!("✓ person+company lookup_by_email: name={:?}, company={:?}",
            profile.profile.name,
            profile.company.as_ref().and_then(|c| c.name.clone()),
        ),
        Err(e) => eprintln!("⊘ person+company lookup_by_email: {}", e),
    }
});

live_test!(live_person_and_company_lookup_by_linkedin_url, {
    let profile = client().person().person_and_company_lookup_by_linkedin_url("https://www.linkedin.com/in/scottmillar", None).await.unwrap();
    eprintln!("✓ person+company lookup_by_linkedin_url: name={:?}, company={:?}",
        profile.profile.name,
        profile.company.as_ref().and_then(|c| c.name.clone()),
    );
});

// ── Company ──────────────────────────────────────────────────────────

live_test!(live_company_lookup_by_domain, {
    let company = client().company().lookup_by_domain("rocketreach.co").await.unwrap();
    eprintln!("✓ company lookup_by_domain: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_company_lookup_by_id, {
    let company = client().company().lookup_by_id(4383).await.unwrap();
    eprintln!("✓ company lookup_by_id: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_company_lookup_by_name, {
    let company = client().company().lookup_by_name("RocketReach").await.unwrap();
    eprintln!("✓ company lookup_by_name: id={:?}, name={:?}", company.id, company.name);
});

live_test!(live_company_lookup_by_linkedin_url, {
    match client().company().lookup_by_linkedin_url("https://www.linkedin.com/company/rocketreach").await {
        Ok(company) => eprintln!("✓ company lookup_by_linkedin_url: id={:?}, name={:?}", company.id, company.name),
        Err(e) => eprintln!("⊘ company lookup_by_linkedin_url: {}", e),
    }
});

live_test!(live_company_lookup_by_ticker, {
    let company = client().company().lookup_by_ticker("GOOG").await.unwrap();
    eprintln!("✓ company lookup_by_ticker: id={:?}, name={:?}, domain={:?}", company.id, company.name, company.domain);
});

live_test!(live_company_search, {
    let query = CompanyQuery::new().domain(vec!["google.com".to_string()]);
    let results = client().company().search_with_query(query).await.unwrap();
    eprintln!("✓ company search: {} results", results.len());
});