mod ensure_index;
mod local_esclient;
mod produce;

use anyhow::Result;
use elasticsearch::{auth::Credentials, http::Url};

use ensure_index::EnsureIndex;
use local_esclient::LocalElasticsearchBuilder;
use produce::produce;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::Basic("elastic".into(), "changeme".into());
    let url = Url::parse("http://localhost:9200")?;

    let client = LocalElasticsearchBuilder::default()
        .credentials(credentials)
        .url(url)
        .build()?;

    let ensure = EnsureIndex::new(&client);

    ensure.ensure_index("logs-ti", true).await?;

    produce(&client, "logs-ti").await?;

    Ok(())
}
