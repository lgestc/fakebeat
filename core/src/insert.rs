use anyhow::Ok;
use elasticsearch::{
    http::{request::JsonBody, response::Response},
    BulkParts, Elasticsearch,
};

use anyhow::Result;
use serde_json::json;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::document_renderer::DocumentRenderer;

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

/// Insert documents in bulk
pub async fn insert_batch<'a>(
    client: &Elasticsearch,
    index: &str,
    document_template: Option<&'a serde_json::Value>,
    batch_size: usize,
    renderer: &mut DocumentRenderer,
) -> Result<Response> {
    let mut bulk_operations: Vec<JsonBody<serde_json::Value>> = Vec::with_capacity(batch_size * 2);

    for _ in 0..batch_size {
        // read document from file
        bulk_operations.push(json!({"index": {"_id": generate_id().as_str()}}).into());

        // handlebars template for a document to insert
        let document_template_string = document_template
            .ok_or(anyhow::anyhow!("missing template"))?
            .to_string();

        let rendered_document = renderer.render(&document_template_string).unwrap();

        let parsed_document_json: serde_json::Value = serde_json::from_str(&rendered_document)?;

        bulk_operations.push(parsed_document_json.into());
    }

    let response = client
        .bulk(BulkParts::Index(index))
        .body(bulk_operations)
        .send()
        .await?;

    Ok(response)
}
