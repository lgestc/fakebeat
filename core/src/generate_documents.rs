use crate::{handlebars, insert::insert_batch};
use anyhow::Result;
use elasticsearch::Elasticsearch;
use tokio::fs::read_to_string;

use crate::document_creation_request::DocumentCreationRequest;

pub async fn generate_documents<'a>(
    client: &'a Elasticsearch,
    document_creation_requests: &'a Vec<DocumentCreationRequest>,
    batch_size: usize,
    mut on_progress: Box<dyn FnMut(usize) -> ()>,
) -> Result<()> {
    let mut total_generated: usize = 0;

    let hb = handlebars::create();

    for request in document_creation_requests.iter() {
        let template_file = read_to_string(&request.template).await?;
        let template: serde_json::Value = serde_json::from_str(&template_file)?;
        let values_definition = template.get("values");

        let mut local_to_generate = request.count;

        while local_to_generate > 0 {
            let batch_size = if batch_size > local_to_generate {
                local_to_generate
            } else {
                batch_size
            };

            local_to_generate -= batch_size;

            let insertion_result =
                insert_batch(&client, &request.index, values_definition, batch_size, &hb).await?;

            if insertion_result.status_code() != 200 {
                eprintln!(
                    "could not insert documents into index {};
                    request failed with status: {} (using template file: {})",
                    &request.index,
                    insertion_result.status_code(),
                    &request.template
                );

                dbg!(insertion_result);
            }

            total_generated += batch_size;

            on_progress(total_generated);
        }
    }

    Ok(())
}
