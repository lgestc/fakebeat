mod ensure_index;
mod insert;
mod local_esclient;

use anyhow::Result;
use elasticsearch::{auth::Credentials, http::Url};

use ensure_index::EnsureIndex;
use insert::insert_batch;
use local_esclient::LocalElasticsearchBuilder;

use linya::{Bar, Progress};

use clap::Parser;
use tokio::fs::read_to_string;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// User name
    #[clap(short, long, value_parser, default_value = "elastic")]
    username: String,

    // Password
    #[clap(short, long, value_parser, default_value = "changeme")]
    password: String,

    // Url
    #[clap(long, value_parser, default_value = "http://localhost:9200")]
    url: String,

    /// Index name
    #[clap(short, long, value_parser, default_value = "logs")]
    index: String,

    /// How many documents
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: usize,

    /// Batch size
    /// There is no good answer on what the batch size should be. probably it should be based on trial and error
    #[clap(short, long, value_parser, default_value_t = 1000)]
    batch: usize,

    /// Template name
    #[clap(value_parser)]
    template: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let credentials = Credentials::Basic(args.username.into(), args.password.into());
    let url = Url::parse(&args.url)?;

    let client = LocalElasticsearchBuilder::default()
        .credentials(credentials)
        .url(url)
        .build()?;

    let ensure = EnsureIndex::new(&client);

    let template = read_to_string(args.template).await?;
    let template: serde_json::Value = serde_json::from_str(&template)?;

    let index_definition = template.get("index");
    let document_definition = template.get("template").unwrap();

    ensure.ensure_index(&args.index, index_definition).await?;

    let mut to_generate = args.count;

    let mut progress = Progress::new();
    let bar: Bar = progress.bar(args.count, "Generating documents");

    while to_generate > 0 {
        let batch_size = if to_generate >= args.batch {
            args.batch
        } else {
            to_generate
        };

        to_generate -= batch_size;

        let result = insert_batch(&client, &args.index, document_definition, batch_size).await?;

        dbg!(result);

        let generated = args.count - to_generate;

        progress.set_and_draw(&bar, generated);
    }

    println!("Done");

    Ok(())
}
