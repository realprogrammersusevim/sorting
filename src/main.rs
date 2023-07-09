mod bubblesort;
mod generator;
mod quicksort;

use clap::Parser;
use clap::Subcommand;
use std::vec;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcmd: Subcmds,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Subcmds {
    Quicksort,
    BubbleSort,
    Generate {
        #[arg(short, long)]
        length: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();
    let datapath = concat!(env!("CARGO_MANIFEST_DIR"), "/data/random");
    let mut data: Vec<u32> = Vec::new();

    data = generator::read_random(datapath).unwrap_or(vec![]);

    match args.subcmd {
        Subcmds::Generate { length } => {
            generator::create_random(length.unwrap_or(1000), datapath).unwrap()
        }
        Subcmds::Quicksort => {
            println!("{:?}", quicksort::quicksort(data))
        }
        Subcmds::BubbleSort => println!("{:?}", bubblesort::bubblesort(data)),
    };
}
