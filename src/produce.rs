use anyhow::Ok;
use elasticsearch::{CreateParts, Elasticsearch};

use anyhow::Result;
use serde_json::json;

pub async fn produce(client: &Elasticsearch, index: &str) -> Result<()> {
    client
        .create(CreateParts::IndexId(index, "1"))
        .body(json!({
            "@timestamp": "2022-09-22T20:37:07.525Z",
            "threat.indicator.first_seen": "2022-09-22T20:37:07.525Z",
            "threat.feed.name": "rust_fakebeat",
            "threat.indicator.type": "url",
            "threat.indicator.url.full": "http://rust.dev",
            "event.type": "indicator",
            "event.category": "threat",
        }))
        .send()
        .await?;

    Ok(())
}
