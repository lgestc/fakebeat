use core::fixture::Fixture;

use clap::Parser;

/// Generates random Elasticsearch documents based on Handlebars templates
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// User name
    #[arg(short, long, value_parser, default_value = "elastic")]
    pub username: String,

    /// Password
    #[arg(short, long, value_parser, default_value = "changeme")]
    pub password: String,

    /// Elasticsearch host
    #[arg(long, value_parser, default_value = "http://localhost:9200")]
    pub url: String,

    /// Elastic cloud id. If specified, overrides the url setting
    #[arg(long, value_parser, required = false)]
    pub cloud: String,

    /// Batch size for inserts
    #[arg(short, long, value_parser, default_value_t = 1000)]
    pub batch: usize,

    /// Index to store documents in (per template)
    #[arg(short, long, value_parser, required_unless_present = "generators")]
    pub index: Vec<String>,

    /// Template file path
    #[arg(value_parser, required_unless_present = "generators")]
    pub template: Vec<String>,

    /// How many documents you want generated (per template)
    #[arg(short, long, value_parser, required_unless_present = "generators")]
    pub count: Vec<usize>,

    /// Append to the existing indices, instead of recreating them
    #[arg(short, long, value_parser, default_value_t = false)]
    pub append: bool,

    /// Print available generators
    #[arg(short, long, value_parser, default_value_t = false)]
    pub generators: bool,
}

impl<'a> TryFrom<&'a Args> for Vec<Fixture> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Args) -> Result<Self, Self::Error> {
        let mut output = Vec::<Fixture>::new();

        let indexes = value.index.len();

        if indexes != value.template.len() || indexes != value.count.len() {
            return Err(anyhow::anyhow!(
                "index and count arguments should be present for every template"
            ));
        }

        for i in 0..value.index.len() {
            output.push(Fixture {
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
