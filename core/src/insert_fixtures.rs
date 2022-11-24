use crate::{document_renderer, insert::insert_batch};
use anyhow::Result;
use elasticsearch::Elasticsearch;
use tokio::fs::read_to_string;

use crate::fixture::Fixture;

pub async fn insert_fixtures<'a>(
    client: &'a Elasticsearch,
    fixtures: &'a Vec<Fixture>,
    batch_size: usize,
    mut on_progress: Box<dyn FnMut(usize) -> ()>,
) -> Result<()> {
    let mut total_generated: usize = 0;

    let mut renderer = document_renderer::DocumentRendererFactory::create_renderer();

    for fixture in fixtures.iter() {
        let template_file = read_to_string(&fixture.template).await?;
        let template: serde_json::Value = serde_json::from_str(&template_file)?;
        let values_definition = template.get("values");

        let mut local_to_generate = fixture.count;

        while local_to_generate > 0 {
            let batch_size = if batch_size > local_to_generate {
                local_to_generate
            } else {
                batch_size
            };

            let insertion_result = insert_batch(
                &client,
                &fixture.index,
                values_definition,
                batch_size,
                &mut renderer,
            )
            .await?;

            if insertion_result.status_code() != 200 {
                eprintln!(
                    "could not insert documents into index {};
                    request failed with status: {} (using template file: {})",
                    &fixture.index,
                    insertion_result.status_code(),
                    &fixture.template
                );

                dbg!(insertion_result);
            }

            local_to_generate -= batch_size;

            total_generated += batch_size;

            on_progress(total_generated);
        }
    }

    Ok(())
}
