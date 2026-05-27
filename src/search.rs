use serde::{Deserialize, Serialize};

use crate::types::OrderBy;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_title: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_or_previous_title: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_title: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_employer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_employer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_company_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_website_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_size: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_revenue: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_funding_min: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_funding_max: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_country_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_industry: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_industry_keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_naics_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_sic_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tag: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_competitors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_publicly_traded: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_geo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_list_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_news_signal: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_job_posting_signal: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_intent: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_skills: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_levels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_method: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_experience: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_change_signal: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_credentials: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_license: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_npi: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_specialization: Option<Vec<String>>,
}

impl PersonQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, names: impl Into<Vec<String>>) -> Self {
        self.name = Some(names.into());
        self
    }

    pub fn keyword(mut self, keywords: impl Into<Vec<String>>) -> Self {
        self.keyword = Some(keywords.into());
        self
    }

    pub fn current_title(mut self, titles: impl Into<Vec<String>>) -> Self {
        self.current_title = Some(titles.into());
        self
    }

    pub fn employer(mut self, employers: impl Into<Vec<String>>) -> Self {
        self.employer = Some(employers.into());
        self
    }

    pub fn current_employer(mut self, employers: impl Into<Vec<String>>) -> Self {
        self.current_employer = Some(employers.into());
        self
    }

    pub fn company_name(mut self, names: impl Into<Vec<String>>) -> Self {
        self.company_name = Some(names.into());
        self
    }

    pub fn company_domain(mut self, domains: impl Into<Vec<String>>) -> Self {
        self.company_domain = Some(domains.into());
        self
    }

    pub fn company_size(mut self, sizes: impl Into<Vec<String>>) -> Self {
        self.company_size = Some(sizes.into());
        self
    }

    pub fn company_industry(mut self, industries: impl Into<Vec<String>>) -> Self {
        self.company_industry = Some(industries.into());
        self
    }

    pub fn geo(mut self, geos: impl Into<Vec<String>>) -> Self {
        self.geo = Some(geos.into());
        self
    }

    pub fn id(mut self, ids: impl Into<Vec<String>>) -> Self {
        self.id = Some(ids.into());
        self
    }

    pub fn email(mut self, emails: impl Into<Vec<String>>) -> Self {
        self.email = Some(emails.into());
        self
    }

    pub fn link(mut self, links: impl Into<Vec<String>>) -> Self {
        self.link = Some(links.into());
        self
    }

    pub fn school(mut self, schools: impl Into<Vec<String>>) -> Self {
        self.school = Some(schools.into());
        self
    }

    pub fn degree(mut self, degrees: impl Into<Vec<String>>) -> Self {
        self.degree = Some(degrees.into());
        self
    }

    pub fn department(mut self, departments: impl Into<Vec<String>>) -> Self {
        self.department = Some(departments.into());
        self
    }

    pub fn skills(mut self, skills: impl Into<Vec<String>>) -> Self {
        self.skills = Some(skills.into());
        self
    }

    pub fn management_levels(mut self, levels: impl Into<Vec<String>>) -> Self {
        self.management_levels = Some(levels.into());
        self
    }

    pub fn contact_method(mut self, methods: impl Into<Vec<String>>) -> Self {
        self.contact_method = Some(methods.into());
        self
    }

    pub fn email_grade(mut self, grade: impl Into<String>) -> Self {
        self.email_grade = Some(grade.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_industry: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sic_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_funding: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub techstack: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tag: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_traded: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_posting_signal: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub news_signal: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_keyword: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simplified_keyword: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<Vec<String>>,
}

impl CompanyQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, names: impl Into<Vec<String>>) -> Self {
        self.name = Some(names.into());
        self
    }

    pub fn domain(mut self, domains: impl Into<Vec<String>>) -> Self {
        self.domain = Some(domains.into());
        self
    }

    pub fn industry(mut self, industries: impl Into<Vec<String>>) -> Self {
        self.industry = Some(industries.into());
        self
    }

    pub fn employees(mut self, sizes: impl Into<Vec<String>>) -> Self {
        self.employees = Some(sizes.into());
        self
    }

    pub fn revenue(mut self, revenues: impl Into<Vec<String>>) -> Self {
        self.revenue = Some(revenues.into());
        self
    }

    pub fn location(mut self, locations: impl Into<Vec<String>>) -> Self {
        self.location = Some(locations.into());
        self
    }

    pub fn techstack(mut self, techs: impl Into<Vec<String>>) -> Self {
        self.techstack = Some(techs.into());
        self
    }

    pub fn competitors(mut self, comps: impl Into<Vec<String>>) -> Self {
        self.competitors = Some(comps.into());
        self
    }

    pub fn id(mut self, ids: impl Into<Vec<String>>) -> Self {
        self.id = Some(ids.into());
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    pub query: PersonQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
}

impl SearchRequest {
    pub fn new(query: PersonQuery) -> Self {
        Self {
            start: None,
            page_size: None,
            query,
            order_by: None,
        }
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn order_by(mut self, order: OrderBy) -> Self {
        self.order_by = Some(order);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CompanySearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    pub query: CompanyQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
}

impl CompanySearchRequest {
    pub fn new(query: CompanyQuery) -> Self {
        Self {
            start: None,
            page_size: None,
            query,
            order_by: None,
        }
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn order_by(mut self, order: OrderBy) -> Self {
        self.order_by = Some(order);
        self
    }
}