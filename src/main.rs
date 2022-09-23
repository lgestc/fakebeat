mod ensure_index;
mod local_esclient;
mod produce;

use anyhow::Result;
use elasticsearch::{auth::Credentials, http::Url};

use ensure_index::EnsureIndex;
use local_esclient::LocalElasticsearchBuilder;
use produce::insert_batch;

use linya::{Bar, Progress};

use clap::Parser;

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

    ensure.ensure_index(&args.index, true).await?;

    let mut to_generate = args.count;

    let mut progress = Progress::new();
    let bar: Bar = progress.bar(args.count, "Generating documents");

    // TODO parse template

    while to_generate > 0 {
        let batch_size = if to_generate >= args.batch {
            args.batch
        } else {
            to_generate
        };

        to_generate -= batch_size;

        insert_batch(&client, &args.index, batch_size).await?;

        let generated = args.count - to_generate;

        progress.set_and_draw(&bar, generated);
    }

    println!("Done");

    Ok(())
}
