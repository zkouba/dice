use clap::{Parser, Subcommand};
use crate::dice::rolls::{d12, d6, roll};
use crate::dice::rolls::Favourableness::{Favoured, Illfavoured, Neutral};

mod dice;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    command: RollType,
}

#[derive(Subcommand, Debug)]
enum RollType {
    D6,
    D12,
    #[command(short_flag = 's')]
    Standard{
        #[arg(long)]
        d6: u8,
        #[arg(short, long, action)]
        favoured: bool,
        #[arg(short = 'u', long, action)]
        illfavoured: bool,
        #[arg(short = 'i', long, action)]
        inspired: bool,
        #[arg(short, long, action)]
        miserable: bool,
        #[arg(short, long, action)]
        weary: bool,
        #[arg(short, long, action)]
        assisted: bool,
        #[arg(short='H', long, action)]
        hope: bool,
        #[arg(short, long)]
        bonus: Option<i8>,
    },
}

fn main() {
    let args = CliArgs::parse();
    roll_dice_app(args.command);
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
            let mut favourableness = if favoured {
                Favoured
            } else {
                Neutral(false)
            };
            if illfavoured {
                favourableness += Illfavoured;
            }
            let res = roll(
                d6,
                favourableness,
                bonus.or(Some(0)).unwrap(),
                hope,
                weary,
                miserable,
                inspired,
                assisted,
            );
            println!("{}", res.to_string());
        }
    }
}
