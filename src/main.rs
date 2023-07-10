mod bubblesort;
mod generator;
mod quicksort;
mod sleepsort;

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
    Sleepsort,
    Generate {
        #[arg(short, long)]
        length: Option<usize>,
        #[arg(short, long)]
        max: Option<usize>,
    },
}

fn main() {
    let args = Args::parse();
    let datapath = concat!(env!("CARGO_MANIFEST_DIR"), "/data/random");
    let data = generator::read_random(datapath).unwrap_or(vec![]);

    match args.subcmd {
        Subcmds::Generate { length, max } => generator::create_random(
            length.unwrap_or(1000),
            datapath,
            max.unwrap_or((u32::MAX).try_into().unwrap())
                .try_into()
                .unwrap(),
        )
        .unwrap(),
        Subcmds::Quicksort => println!("{:?}", quicksort::quicksort(data)),
        Subcmds::BubbleSort => println!("{:?}", bubblesort::bubblesort(data)),
        Subcmds::Sleepsort => println!("{:?}", sleepsort::sleepsort(data)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_data() -> (Vec<u32>, Vec<u32>) {
        (
            vec![8, 2, 1, 3, 4, 6, 7, 9, 0, 5],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        )
    }

    #[test]
    fn test_bubblesort() {
        let data = test_data();
        assert_eq!(bubblesort::bubblesort(data.0), data.1)
    }

    #[test]
    fn test_quicksort() {
        let data = test_data();
        assert_eq!(quicksort::quicksort(data.0), data.1)
    }

    #[test]
    fn test_sleepsort() {
        let data = test_data();
        assert_eq!(sleepsort::sleepsort(data.0), data.1)
    }
}
