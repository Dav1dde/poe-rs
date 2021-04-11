use clap::Clap;
use futures::stream::TryStreamExt;
use poe_api::page::PagedStream;
use poe_api::{PathOfExile, PoeError};

#[derive(Clap)]
struct Args {
    /// Path of Exile league name
    league: String,
    /// delay in ms between each row
    #[clap(short = 'd')]
    print_delay: Option<u64>,
}

#[tokio::main(worker_threads = 4)]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let league_name = &args.league;
    let print_delay = args.print_delay;

    let poe = PathOfExile::new();

    let mut stream = PagedStream::new(5, 200, Some(15000), |pr| {
        let poe = &poe;
        async move {
            let ladder = poe.ladder(league_name, pr.limit, pr.offset).await.unwrap();
            Ok::<_, PoeError>(ladder.entries.into_iter())
        }
    });

    let mut human = human_format::Formatter::new();
    human.with_decimals(3);

    row(
        " S",
        "R",
        "LVL",
        "Character Name",
        "Account Name",
        "Experience",
    );
    divider();

    while let Some(entry) = stream.try_next().await? {
        let symbol = match entry.dead {
            true => "\u{1F480}",
            false => match entry.online {
                true => "\u{1F30E}",
                false => "  ",
            },
        };

        let experience = human.format(entry.character.experience as f64);

        row(
            symbol,
            &entry.rank.to_string(),
            &entry.character.level.to_string(),
            &entry.character.name,
            &entry.account.name,
            &experience,
        );

        if let Some(delay) = print_delay {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }
    }

    Ok(())
}

fn row(symbol: &str, rank: &str, level: &str, character: &str, account: &str, experience: &str) {
    println!(
        "| {} | {:>5} | {:>3} | {:<25} | {:<25} | {:<10} |",
        symbol, rank, level, character, account, experience
    );
}

fn divider() {
    println!("{:=<89}", "");
}
