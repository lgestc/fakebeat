use anyhow::{anyhow, Result};
use elasticsearch::{
    indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts},
    Elasticsearch,
};

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
        println!("Dropping \"{}\"", index);

        self.client
            .indices()
            .delete(IndicesDeleteParts::Index(&[index]))
            .send()
            .await?;

        Ok(())
    }

    async fn create_index(&self, index: &str, payload: &serde_json::Value) -> Result<()> {
        println!("Creating \"{}\"", index);

        self.client
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(payload)
            .send()
            .await?;

        Ok(())
    }

    pub async fn ensure_index(
        &self,
        index: &str,
        index_definition: Option<&serde_json::Value>,
        append: bool,
    ) -> Result<()> {
        let index_exists = self.exists(index).await?;

        if append {
            if index_exists {
                return Ok(());
            } else {
                return Err(anyhow!("index {} does not exist, cannot append. run this command without the 'append' flag first", index));
            }
        } else {
            if index_definition.is_some() {
                self.drop_index(index).await?;
                self.create_index(index, index_definition.unwrap()).await?;
            } else {
                return Err(anyhow!(
                    "index definition was not provided, check your template"
                ));
            }

            return Ok(());
        }
    }
}
