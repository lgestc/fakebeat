use anyhow::Ok;
use elasticsearch::{
    http::{request::JsonBody, response::Response},
    BulkParts, Elasticsearch,
};
use fake::{
    faker::internet::en::{DomainSuffix, Username},
    Fake,
};
use rand::Rng;

use anyhow::Result;
use serde_json::json;

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Duration, Utc};

const FORMAT_ISO: &str = "%FT%T%z";

// This is temporary until id's are optional
fn generate_id() -> String {
    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string();

    timestamp
}

fn random_iso_date() -> String {
    let mut rng = rand::thread_rng();
    let random_offset = rng.gen_range(0..30);

    let dt = Utc::now() - Duration::days(random_offset);

    dt.format(FORMAT_ISO).to_string()
}

fn random_username() -> String {
    Username().fake()
}

fn random_domain() -> String {
    DomainSuffix().fake()
}

/// Insert documents in bulk
pub async fn insert_batch(
    client: &Elasticsearch,
    index: &str,
    document_template: &serde_json::Value,
    batch_size: usize,
) -> Result<Response> {
    let mut operations: Vec<JsonBody<serde_json::Value>> = Vec::with_capacity(batch_size * 2);

    for _ in 0..batch_size {
        // read document from file
        operations.push(json!({"index": {"_id": generate_id().as_str()}}).into());

        let compiled_body = document_template
            .to_string()
            .replace("{username}", &random_username())
            .replace("{domain}", &random_domain())
            .replace("{date.iso}", &random_iso_date());

        let parsed: serde_json::Value = serde_json::from_str(&compiled_body)?;

        operations.push(parsed.into());
    }

    let response = client
        .bulk(BulkParts::Index(index))
        .body(operations)
        .send()
        .await?;

    Ok(response)
}
