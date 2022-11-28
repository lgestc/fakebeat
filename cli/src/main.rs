use fakebeat_core::{
    document_renderer, fixture::Fixture, insert_fixtures::insert_fixtures,
    local_esclient::LocalElasticsearchBuilder, prepare_indices::prepare_indices,
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
        let renderer = document_renderer::DocumentRendererFactory::create_renderer();

        println!("Available generators:\n");

        for (name, desc) in renderer.get_generators() {
            println!("{} - {}", name, desc);
        }

        println!("");

        return Ok(());
    }

    let credentials = Credentials::Basic(args.username.clone(), args.password.clone());

    let url = Url::parse(&args.url)?;
    let append = args.append;
    let batch_size = args.batch;

    let fixtures = Vec::<Fixture>::try_from(&args)?;

    let client = if let Some(cloud) = args.cloud {
        let credentials = Credentials::Basic(args.username.into(), args.password.into());
        let transport = Transport::cloud(&cloud, credentials)?;
        Elasticsearch::new(transport)
    } else {
        LocalElasticsearchBuilder::default()
            .credentials(credentials)
            .url(url)
            .build()?
    };

    println!("Setting up indices");

    prepare_indices(&client, &fixtures, append).await?;

    println!("Indices ready");

    let total_fixtures_to_generate: usize = args.count.iter().sum();

    let mut progress = Progress::new();
    let bar: Bar = progress.bar(total_fixtures_to_generate, "Inserting fixtures");

    let on_progress = Box::new(move |current_progress_value| {
        progress.set_and_draw(&bar, current_progress_value);
    });

    insert_fixtures(&client, &fixtures, batch_size, on_progress).await?;

    println!("Done");

    Ok(())
}
