use std::io::Write;
use crate::dice::error::DiceError;
use crate::dice::parser::parse_dice_roll;
use crate::dice::rolls::{roll, RollResult};

mod dice;
mod text_app;

fn main() {
    let prompt = "🎲: ";

    println!("\n\nWelcome to the Dice Roller!");
    std::io::stdout().flush().expect("Failed to flush stdout");
    _ = text_app::text_app_loop(prompt, &roll_dice_app);
}

fn roll_dice_app(roll_expressions: Vec<String>) -> Result<(), DiceError>{
    let mut results = Vec::<RollResult>::with_capacity(roll_expressions.len());
    for i in 0..roll_expressions.len() {
        let expression = &roll_expressions[i];
        let rolls = parse_dice_roll(expression)?;
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
    Ok(())
}
