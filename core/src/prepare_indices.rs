use anyhow::Result;
use elasticsearch::Elasticsearch;
use tokio::fs::read_to_string;

use crate::{ensure_index::EnsureIndex, fixture::Fixture};

pub async fn prepare_indices(
    client: &Elasticsearch,
    document_creation_requests: &Vec<Fixture>,
    append: bool,
) -> Result<()> {
    let ensure = EnsureIndex::new(&client);

    for request in document_creation_requests.iter() {
        let template_file = read_to_string(&request.template).await?;
        let template: serde_json::Value = serde_json::from_str(&template_file)?;
        let index_definition = template.get("index");

        ensure
            .ensure_index(&request.index, index_definition, append)
            .await?;
    }

    Ok(())
}
