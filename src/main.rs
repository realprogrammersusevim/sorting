mod bubblesort;
mod generator;

use clap::Parser;
use clap::Subcommand;
use std::env;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmds,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Subcmds {
    Quicksort,
    BubbleSort,
    Generate,
}

fn main() {
    let args = Args::parse();
    let datapath = concat!(env!("CARGO_MANIFEST_DIR"), "/data/random");
    let mut data: Vec<u32> = Vec::new();

    if Path::new(datapath).exists() && args.subcmd != Subcmds::Generate {
        data = generator::read_random(datapath).unwrap();
    }

    match args.subcmd {
        Subcmds::Generate => generator::create_random(1000, datapath).unwrap(),
        Subcmds::Quicksort => unimplemented!(),
        Subcmds::BubbleSort => println!("{:?}", bubblesort::bubblesort(data)),
    };
}
