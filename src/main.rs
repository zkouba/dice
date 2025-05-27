use std::ops::AddAssign;
use clap::{Parser, Subcommand};
use crate::dice::rolls::{d12, d6, roll, Favourableness};
use crate::dice::rolls::Favourableness::{Favoured, Illfavoured, Neutral};

pub mod character;
pub mod itinerary;
pub mod skills;
pub mod error;
mod dice;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Roll{
        #[clap(subcommand)]
        roll_command: RollType,
    },
}

#[derive(Subcommand, Debug)]
enum RollType {
    D6,
    D12,
    #[command(short_flag = 's')]
    Standard{
        #[arg(short, long)]
        d6: u8,
        #[arg(short, long)]
        favoured: Option<bool>,
        #[arg(short = 'u', long)]
        illfavoured: Option<bool>,
        #[arg(short = 'i', long)]
        inspired: Option<bool>,
        #[arg(short, long)]
        miserable: Option<bool>,
        #[arg(short, long)]
        weary: Option<bool>,
        #[arg(short, long)]
        assisted: Option<bool>,
        #[arg(short='H', long)]
        hope: Option<bool>,
        #[arg(short, long)]
        bonus: Option<i8>,
    },
}

fn main() {
    let args = CliArgs::parse();
    match args.command {
        Command::Roll{roll_command} => {
            roll_dice_app(roll_command);
        }
    }
}

fn roll_dice_app(roll_command: RollType) {
    match roll_command {
        RollType::D6 => {
            let res = d6();
            println!("{}", res);
        }
        RollType::D12 => {
            let res = d12();
            println!("{}", res);
        }
        RollType::Standard{
            d6,
            favoured,
            illfavoured,
            inspired,
            miserable,
            weary,
            assisted,
            hope,
            bonus
        } => {
            let mut favourableness = if favoured.unwrap_or(false) {
                Favoured
            } else {
                Neutral(false)
            };
            if illfavoured.unwrap_or(false) {
                favourableness += Illfavoured;
            }
            let res = roll(
                d6,
                favourableness,
                bonus.or(Some(0)).unwrap(),
                hope.or(Some(false)).unwrap(),
                weary.or(Some(false)).unwrap(),
                miserable.or(Some(false)).unwrap(),
                inspired.or(Some(false)).unwrap(),
                assisted.or(Some(false)).unwrap(),
            );
            println!("{}", res.to_string());
        }
    }
}
