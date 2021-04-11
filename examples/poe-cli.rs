use clap::Clap;
use poe_api::{PathOfExile, PoeError};

#[derive(Clap)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Character items
    Items(Items),
    /// Character passives
    Passives(Passives),
}

#[derive(Clap)]
struct Items {
    /// Account name
    account: String,
    /// Character name
    character: String,
}

#[derive(Clap)]
struct Passives {
    /// Account name
    account: String,
    /// Character name
    character: String,
    /// Whether to include skill tree data
    #[clap(short, long)]
    skill_tree_data: bool,
}

async fn print_items_json(poe: &PathOfExile, opts: &Items) {
    let items = poe.get_items(&opts.account, &opts.character).await.unwrap();
    println!("{}", serde_json::to_string(&items).unwrap());
}

async fn print_passives_json(poe: &PathOfExile, opts: &Passives) {
    let passives = poe
        .get_passives(&opts.account, &opts.character, opts.skill_tree_data)
        .await
        .unwrap();
    println!("{}", serde_json::to_string(&passives).unwrap());
}

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

async fn try_main() -> Result<(), PoeError> {
    let opts: Opts = Opts::parse();

    tracing_subscriber::fmt()
        .with_max_level(match opts.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        })
        .init();

    let poe = PathOfExile::new();

    match opts.subcmd {
        SubCommand::Items(items) => {
            print_items_json(&poe, &items).await;
        }
        SubCommand::Passives(passives) => {
            print_passives_json(&poe, &passives).await;
        }
    }

    Ok(())
}
