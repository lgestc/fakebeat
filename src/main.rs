mod args;
mod document_creation_request;
mod ensure_index;
mod generate_documents;
mod handlebars;
mod insert;
mod local_esclient;
mod prepare_indices;

use crate::args::Args;
use crate::generate_documents::generate_documents;
use crate::prepare_indices::prepare_indices;
use anyhow::Result;
use clap::Parser;
use document_creation_request::DocumentCreationRequest;
use elasticsearch::{auth::Credentials, http::Url};
use linya::{Bar, Progress};
use local_esclient::LocalElasticsearchBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let credentials = Credentials::Basic(args.username.clone(), args.password.clone());

    let url = Url::parse(&args.url)?;
    let append = args.append;
    let batch_size = args.batch;

    let document_creation_requests = Vec::<DocumentCreationRequest>::try_from(&args)?;

    let client = LocalElasticsearchBuilder::default()
        .credentials(credentials)
        .url(url)
        .build()?;

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
