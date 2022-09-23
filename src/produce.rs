use anyhow::Ok;
use elasticsearch::{http::request::JsonBody, BulkParts, Elasticsearch};
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

/// Insert documents in bulk
pub async fn insert_batch(client: &Elasticsearch, index: &str, batch_size: usize) -> Result<()> {
    let mut body: Vec<JsonBody<serde_json::Value>> = Vec::with_capacity(batch_size * 2);

    for _ in 0..batch_size {
        let random_date = random_iso_date();

        body.push(json!({"index": {"_id": generate_id().as_str()}}).into());
        body.push(
            json!({
                "@timestamp": random_date,
                "threat.indicator.first_seen": random_date,
                "threat.feed.name": "rust_fakebeat",
                "threat.indicator.type": "url",
                "threat.indicator.url.full": "http://rust.dev",
                "event.type": "indicator",
                "event.category": "threat",
            })
            .into(),
        )
    }

    client
        .bulk(BulkParts::Index(index))
        .body(body)
        .send()
        .await?;

    Ok(())
}
