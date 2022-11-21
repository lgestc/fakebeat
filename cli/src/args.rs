use core::document_creation_request::DocumentCreationRequest;

use clap::Parser;

/// Generates random Elasticsearch documents based on Handlebars templates
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// User name
    #[arg(short, long, value_parser, default_value = "elastic")]
    pub username: String,

    // Password
    #[arg(short, long, value_parser, default_value = "changeme")]
    pub password: String,

    // Url
    #[arg(long, value_parser, default_value = "http://localhost:9200")]
    pub url: String,

    /// Batch size
    /// There is no good answer on what the batch size should be.
    /// Adjust it with a trial-and-error approach.
    #[arg(short, long, value_parser, default_value_t = 1000)]
    pub batch: usize,

    /// Index name
    #[arg(short, long, value_parser, required = true)]
    pub index: Vec<String>,

    /// Template name
    #[arg(value_parser, required = true)]
    pub template: Vec<String>,

    /// How many documents
    #[arg(short, long, value_parser, required = true)]
    pub count: Vec<usize>,

    /// Append
    #[arg(short, long, value_parser, default_value_t = false)]
    pub append: bool,
}

impl<'a> TryFrom<&'a Args> for Vec<DocumentCreationRequest> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Args) -> Result<Self, Self::Error> {
        let mut output = Vec::<DocumentCreationRequest>::new();

        let indexes = value.index.len();

        if indexes != value.template.len() || indexes != value.count.len() {
            return Err(anyhow::anyhow!(
                "index and count arguments should be present for every template"
            ));
        }

        for i in 0..value.index.len() {
            output.push(DocumentCreationRequest {
                index: value
                    .index
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing index"))?
                    .to_owned(),
                count: value
                    .count
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing count"))?
                    .clone(),
                template: value
                    .template
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing template"))?
                    .to_owned(),
            });
        }

        Ok(output)
    }
}
