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
