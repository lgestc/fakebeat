use anyhow::{anyhow, Result};
use elasticsearch::{
    indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts},
    Elasticsearch,
};
use serde_json::json;

/// Creates (dropping previous one optionally) or returns existing index do append fake logs to
pub struct EnsureIndex<'a> {
    client: &'a Elasticsearch,
}

impl<'a> EnsureIndex<'a> {
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }

    async fn exists(&self, index: &str) -> Result<bool> {
        let index_exists_response = self
            .client
            .indices()
            .exists(IndicesExistsParts::Index(&[index]))
            .send()
            .await?;

        Ok(index_exists_response.status_code() == 200)
    }

    async fn drop_index(&self, index: &str) -> Result<()> {
        println!("dropping index \"{}\"", index);

        self.client
            .indices()
            .delete(IndicesDeleteParts::Index(&[index]))
            .send()
            .await?;

        Ok(())
    }

    async fn create_index(&self, index: &str) -> Result<()> {
        println!("creating index \"{}\"", index);

        self.client
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(json!({
              "mappings": {
                "properties": {
                  "threat.indicator.type": { "type": "keyword" },
                  "threat.feed.name": { "type": "keyword" },
                  "threat.indicator.url.full": { "type": "keyword" },
                  "threat.indicator.first_seen": { "type": "date" },
                  "@timestamp": { "type": "date" }
                }
              }
            }))
            .send()
            .await?;

        Ok(())
    }

    pub async fn ensure_index(&self, index: &str, create: bool) -> Result<()> {
        let index_exists = self.exists(index).await?;

        if index_exists {
            if create {
                self.drop_index(index).await?;
                self.create_index(index).await?;
            }
        } else if !create {
            return Err(anyhow!("index does not exist and it was not created!"));
        }

        return Ok(());
    }
}
