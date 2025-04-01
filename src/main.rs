use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

mod models;
use crate::models::pokerhand::HandValue;
use crate::models::sorce::Sorce;
use clap::Parser;
use ortalib::{Chips, Mult, Round};

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round) -> (Chips, Mult) {
    println!("{:?}", round);
    let hand = HandValue::evaluation(&round.cards_played);
    // println!("{:?}", &hand);
    println!("{:?}", &hand.cards_impl);
    for card in &hand.cards_impl {
        let enh = card.enhancement.as_ref().map(|e| format!("{:?}", e)).unwrap_or_default();
        let edi = card.edition.as_ref().map(|e| format!("{:?}", e)).unwrap_or_default();
        let output = match (enh.is_empty(), edi.is_empty()) {
            (true, true) => "None".to_string(),
            _ => [enh, edi].join(" ").trim().to_string(),
        };
        println!("{}", output);
    }
    let sorce = Sorce::get_card(hand);
    // println!("{:?}", sorce);
    return (sorce.total_chips, sorce.mult);
}
