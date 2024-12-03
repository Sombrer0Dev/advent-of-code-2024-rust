use core::panic;

use clap::Parser;
mod solve;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: i8,
    #[arg(short, long)]
    part: i8,
    #[arg(short, long)]
    sample: bool,
}

fn main() {
    let args = Cli::parse();
    let res = solve::switch(args.day, args.part, args.sample);
    match res {
        Ok(v) => {
            println!("Day: {}, Part: {}\nAnswer: {v}", args.day, args.part)
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}
