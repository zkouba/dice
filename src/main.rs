use std::env;
use crate::dice::parser::parse_dice_roll;
use crate::dice::rolls::{d12, d6, roll, RollResult};

mod dice;

// #[derive(Parser, Debug)]
// struct CliArgs {
//     #[clap(subcommand)]
//     command: RollType,
// }
//
// #[derive(Subcommand, Debug)]
// enum RollType {
//     D6,
//     D12,
//     // #[command(short_flag = 's')]
//     // Standard{
//     //     #[arg(long)]
//     //     d6: u8,
//     //     #[arg(short, long, action)]
//     //     favoured: bool,
//     //     #[arg(short = 'u', long, action)]
//     //     illfavoured: bool,
//     //     #[arg(short = 'i', long, action)]
//     //     inspired: bool,
//     //     #[arg(short, long, action)]
//     //     miserable: bool,
//     //     #[arg(short, long, action)]
//     //     weary: bool,
//     //     #[arg(short, long, action)]
//     //     assisted: bool,
//     //     #[arg(short='H', long, action)]
//     //     hope: bool,
//     //     #[arg(short, long)]
//     //     bonus: Option<i8>,
//     // },
// }

fn main() {
    let argv: Vec<String> = env::args().collect();
    roll_dice_app(argv);
}

fn roll_dice_app(roll_expressions: Vec<String>) {
    let mut results = Vec::<RollResult>::with_capacity(roll_expressions.len());
    for i in 1..roll_expressions.len() {
        let expression = &roll_expressions[i];
        let rolls = parse_dice_roll(expression).unwrap();
        for roll_cmd in rolls {
            results.push(roll(roll_cmd));
        }
    }

    let mut sum: u8 = 0;
    for result in results {
        println!("{:?}", result);
        sum += result.value;
    }
    println!("Total: {}", sum);
    // match roll_command {
    //     RollType::D6 => {
    //         let res = d6();
    //         println!("{}", res);
    //     }
    //     RollType::D12 => {
    //         let res = d12();
    //         println!("{}", res);
    //     }
    //     // RollType::Standard{
    //     //     d6,
    //     //     favoured,
    //     //     illfavoured,
    //     //     inspired,
    //     //     miserable,
    //     //     weary,
    //     //     assisted,
    //     //     hope,
    //     //     bonus
    //     // } => {
    //     //     let mut favourableness = if favoured {
    //     //         Favoured
    //     //     } else {
    //     //         Neutral(false)
    //     //     };
    //     //     if illfavoured {
    //     //         favourableness += Illfavoured;
    //     //     }
    //     //     let res = roll(
    //     //         d6,
    //     //         favourableness,
    //     //         bonus.or(Some(0)).unwrap(),
    //     //         hope,
    //     //         weary,
    //     //         miserable,
    //     //         inspired,
    //     //         assisted,
    //     //     );
    //     //     println!("{}", res.to_string());
    //     // }
    // }
}
