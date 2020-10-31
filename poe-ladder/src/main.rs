use clap::{App, Arg};
use futures::stream::TryStreamExt;
use poe_api::page::PagedStream;
use poe_api::{PathOfExile, PoeError};

#[tokio::main(core_threads = 4)]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("PoE Ladder")
        .arg(
            Arg::with_name("LEAGUE")
                .help("the PoE league")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("print-delay")
                .long("print-delay")
                .short("d")
                .help("delay in ms between printing rows")
                .takes_value(true),
        )
        .get_matches();

    let league_name = app.value_of("LEAGUE").unwrap();
    let print_delay = app
        .value_of("print-delay")
        .map(|delay| delay.parse::<u64>().expect("invalid delay"));

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
            tokio::time::delay_for(tokio::time::Duration::from_millis(delay)).await;
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
