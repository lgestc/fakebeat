use anyhow::{anyhow, Result};
use elasticsearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    Elasticsearch,
};

#[derive(Default)]
pub struct LocalElasticsearchBuilder {
    credentials: Option<Credentials>,
    url: Option<Url>,
}

impl LocalElasticsearchBuilder {
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);

        self
    }

    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);

        self
    }

    pub fn build(self) -> Result<Elasticsearch> {
        let url = self.url.ok_or(anyhow!("missing url"))?;
        let credentials = self.credentials.ok_or(anyhow!("missing credentials"))?;

        let conn_pool = SingleNodeConnectionPool::new(url);

        let transport = TransportBuilder::new(conn_pool).auth(credentials).build()?;

        let client = Elasticsearch::new(transport);

        Ok(client)
    }
}
