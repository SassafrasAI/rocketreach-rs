# rocketreach-rs

An asynchronous Rust client library for the [RocketReach API](https://docs.rocketreach.co/reference/rocketreach-api-account).

## Features

- **Full API coverage** — Account, Person Lookup/Search/Status, Company Lookup/Search, Person+Company Lookup, and all Universal Credit endpoints
- **Async/await** — Built on `reqwest` with `rustls-tls`
- **Type-safe** — Strongly typed request builders and response structs with serde deserialization
- **Builder pattern** — Ergonomic query construction for search endpoints

## Installation

```toml
[dependencies]
rocketreach-rs = "0.1"
```

## Quick Start

```rust
use rocketreach_rs::RocketReachClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RocketReachClient::new("your-api-key");

    // Look up a person by name and employer
    let profile = client.person()
        .lookup_by_name("John Doe", "Acme Corp", None)
        .await?;
    println!("Name: {:?}", profile.name);
    println!("Email: {:?}", profile.recommended_email);

    // Search for people
    let query = rocketreach_rs::PersonQuery::new()
        .current_title(vec!["Software Engineer".into()])
        .company_name(vec!["RocketReach".into()]);
    let request = rocketreach_rs::SearchRequest::new(query)
        .page_size(10)
        .order_by(rocketreach_rs::OrderBy::Popularity);
    let results = client.person().search(&request).await?;
    for result in results {
        println!("{} - {:?}", result.id.unwrap(), result.name);
    }

    // Look up a company
    let company = client.company().lookup_by_domain("rocketreach.co").await?;
    println!("Company: {:?}", company.name);

    // Check your account
    let account = client.account().get().await?;
    println!("Credits used: {:?}", account.daily_api_num_calls);

    // Universal person lookup with enrichment options
    let request = rocketreach_rs::UniversalPersonLookupRequest {
        id: Some(12345),
        reveal_professional_email: Some(true),
        reveal_phone: Some(true),
        ..Default::default()
    };
    let profile = client.universal().person().lookup_by_id(&request).await?;
    println!("Phones: {:?}", profile.phones);

    Ok(())
}
```

## API Reference

### Account

| Method | Endpoint | Description |
|--------|----------|-------------|
| `client.account().get()` | `GET /account/` | Retrieve account details |
| `client.account().create_api_key()` | `POST /account/key/` | Create a new API key |

### Person

| Method | Endpoint | Description |
|--------|----------|-------------|
| `client.person().lookup_by_id(id, return_cached_emails)` | `GET /person/lookup` | Look up person by profile ID |
| `client.person().lookup_by_name(name, employer, return_cached_emails)` | `GET /person/lookup` | Look up person by name + employer |
| `client.person().lookup_by_email(email, return_cached_emails)` | `GET /person/lookup` | Look up person by email |
| `client.person().lookup_by_linkedin_url(url, return_cached_emails)` | `GET /person/lookup` | Look up person by LinkedIn URL |
| `client.person().check_status(ids)` | `GET /person/checkStatus` | Check lookup status |
| `client.person().search(request)` | `POST /person/search` | Search people |
| `client.person().person_and_company_lookup_by_*` | `GET /profile-company/lookup` | Person + company lookup |

### Company

| Method | Endpoint | Description |
|--------|----------|-------------|
| `client.company().lookup_by_domain(domain)` | `GET /company/lookup/` | Look up company by domain |
| `client.company().lookup_by_id(id)` | `GET /company/lookup/` | Look up company by ID |
| `client.company().lookup_by_name(name)` | `GET /company/lookup/` | Look up company by name |
| `client.company().lookup_by_linkedin_url(url)` | `GET /company/lookup/` | Look up company by LinkedIn URL |
| `client.company().lookup_by_ticker(ticker)` | `GET /company/lookup/` | Look up company by stock ticker |
| `client.company().search(request)` | `POST /searchCompany` | Search companies |

### Universal (Credit-based)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `client.universal().account().get()` | `GET /universal/account/` | Get universal account details |
| `client.universal().person().lookup_by_id(request)` | `GET /universal/person/lookup` | Universal person lookup |
| `client.universal().person().check_status(ids)` | `GET /universal/person/check_status` | Check universal lookup status |
| `client.universal().person().search(request)` | `POST /universal/person/search` | Universal people search |
| `client.universal().company().lookup_by_*` | `GET /universal/company/lookup` | Universal company lookup |
| `client.universal().company().search(request)` | `POST /universal/company/search` | Universal company search |

## Error Handling

```rust
use rocketreach_rs::{RocketReachClient, Error};

async fn example() -> Result<(), Error> {
    let client = RocketReachClient::new("your-api-key");

    match client.person().lookup_by_email("nonexistent@example.com", None).await {
        Ok(profile) => println!("{:?}", profile.name),
        Err(Error::Unauthorized { request_id }) => {
            eprintln!("Invalid API key. Request ID: {:?}", request_id);
        }
        Err(Error::RateLimited { request_id }) => {
            eprintln!("Rate limited. Back off. Request ID: {:?}", request_id);
        }
        Err(Error::NotFound { request_id }) => {
            eprintln!("Profile not found. Request ID: {:?}", request_id);
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
```

## License

BSD-3-Clause. See [LICENSE](LICENSE) for details.