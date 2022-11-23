use core::{
    document_creation_request::DocumentCreationRequest, generate_documents::generate_documents,
    handlebars, local_esclient::LocalElasticsearchBuilder, prepare_indices::prepare_indices,
};

use anyhow::Result;
use clap::Parser;
use elasticsearch::{
    auth::Credentials,
    http::{transport::Transport, Url},
    Elasticsearch,
};
use linya::{Bar, Progress};

mod args;

use args::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.generators {
        let hb = handlebars::create();

        println!("Available generators:");

        for generator in hb.get_generators() {
            println!("{}", generator);
        }

        println!("");

        return Ok(());
    }

    let credentials = Credentials::Basic(args.username.clone(), args.password.clone());

    let url = Url::parse(&args.url)?;
    let append = args.append;
    let batch_size = args.batch;

    let document_creation_requests = Vec::<DocumentCreationRequest>::try_from(&args)?;

    let client = if args.cloud.len() > 0 {
        let credentials = Credentials::Basic(args.username.into(), args.password.into());
        let transport = Transport::cloud(&args.cloud, credentials)?;
        Elasticsearch::new(transport)
    } else {
        LocalElasticsearchBuilder::default()
            .credentials(credentials)
            .url(url)
            .build()?
    };

    println!("Setting up indices");

    prepare_indices(&client, &document_creation_requests, append).await?;

    println!("Indices ready");

    let total_docs_to_generate: usize = args.count.iter().sum();

    let mut progress = Progress::new();
    let bar: Bar = progress.bar(total_docs_to_generate, "Generating documents");

    let on_progress = Box::new(move |current_progress_value| {
        progress.set_and_draw(&bar, current_progress_value);
    });

    generate_documents(
        &client,
        &document_creation_requests,
        batch_size,
        on_progress,
    )
    .await?;

    println!("Done");

    Ok(())
}
