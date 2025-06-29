//! Built-in Sumsub event templates for testing
use clap::ValueEnum;
use serde_json::json;
use serde_json::Value;

/// Supported predefined Sumsub event types
#[allow(clippy::enum_variant_names)]
#[derive(Clone, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum EventKind {
    ApplicantCreated,
    ApplicantPending,
    ApplicantReviewed,
}

impl EventKind {
    /// Return a JSON string template for the selected predefined event
    pub fn payload(&self) -> Value {
        match self {
            EventKind::ApplicantCreated => json!({
                "applicantId": "5c9e177b0a975a6eeccf5960",
                "inspectionId": "5c9e177b0a975a6eeccf5961",
                "correlationId": "req-63f92830-4d68-4eee-98d5-875d53a12258",
                "levelName": "id-and-liveness",
                "externalUserId": "12672",
                "type": "applicantCreated",
                "sandboxMode": false,
                "reviewStatus": "init",
                "createdAtMs": "2020-02-21 13:23:19.002",
                "clientId": "coolClientId"
            }),
            EventKind::ApplicantPending => json!({
                "applicantId": "5c7791f80a975a1df426b9e9",
                "inspectionId": "5c7791f80a975a1df426b9ea",
                "applicantType": "individual",
                "correlationId": "req-4af54c06-6a50-4cb9-a7dc-b94b2f5b07eb",
                "levelName": "liveness-level",
                "externalUserId": "12672",
                "type": "applicantPending",
                "sandboxMode": false,
                "reviewStatus": "pending",
                "createdAtMs": "2020-02-21 13:23:19.002",
                "clientId": "coolClientId"
            }),
            EventKind::ApplicantReviewed => json!({
                "applicantId": "5cb56e8e0a975a35f333cb83",
                "inspectionId": "5cb56e8e0a975a35f333cb84",
                "correlationId": "req-a260b669-4f14-4bb5-a4c5-ac0218acb9a4",
                "externalUserId": "externalUserId",
                "levelName": "id-and-liveness",
                "type": "applicantReviewed",
                "reviewResult": {
                    "reviewAnswer": "GREEN"
                },
                "reviewStatus": "completed",
                "createdAtMs": "2020-02-21 13:23:19.321"
            }),
        }
    }
}

