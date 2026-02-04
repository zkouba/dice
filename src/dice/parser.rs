use regex::Regex;
use crate::dice::rolls::{DiceRoll, Favourableness};
use crate::dice::error::DiceError;

const DICE_EXPRESSION : &str = r"^([+\\-]?)(\d+)d(\d+)$";

pub fn parse_dice_roll(input: &str) -> Result<Vec<DiceRoll>, DiceError> {
    let mut rolls = Vec::<DiceRoll>::new();
    for r in input.split_whitespace().map(|s| parse_dice_expression(s)) {
        let r = r?;
        rolls.extend(r);
    }
    Ok(rolls)
}


//noinspection RsLift
#[allow(clippy::needless_return)]
pub fn parse_dice_expression(expression: &str) -> Result<Vec<DiceRoll>, DiceError> {
    let re = Regex::new(DICE_EXPRESSION).map_err(|e| DiceError::new_caused_by(Box::new(e)))?; // TODO - extract into an object initialization
    if let Some(captured_groups) = re.captures(expression) {
        let mut capture_idx = 1;
        let favourableness = if let Some(sign) = captured_groups.get(capture_idx) {
            let sign_str = sign.as_str();
            capture_idx += 1;
            match sign_str {
                "+" => Favourableness::Favoured,
                "-" => Favourableness::Illfavoured,
                "" => Favourableness::Neutral(false),
                _ => return Err(DiceError::new_standalone(format!("Invalid favourableness specification: {}", expression)))
            }
        } else {
            return Err(DiceError::new_standalone(format!("Unable to detect favourableness specification: {}", expression)))
        };
        let dice_num = if let Some(dice_num_str) = captured_groups.get(capture_idx) {
            capture_idx += 1;
            dice_num_str.as_str().parse::<usize>().map_err(|e| DiceError::new_caused_by(Box::new(e)))?
        } else {
            return Err(DiceError::new_standalone(format!("Unable to detect number of dice : {}", expression)))
        };
        if let Some(dice_type_str) = captured_groups.get(capture_idx) {
            match dice_type_str.as_str() {
                "6" => {
                    return Ok(vec![DiceRoll::D6; dice_num]);
                }
                "12" => {
                    return Ok(vec![DiceRoll::D12(favourableness); dice_num])
                }
                _ => return Err(DiceError::new_standalone(format!("Invalid dice type: {}", expression))),
            }
        } else {
            return Err(DiceError::new_standalone(format!("Unable to detect dice type: {}", expression)))
        }
    } else {
        return Err(DiceError::new_standalone(format!("Invalid dice expression: {}", expression)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_d6_roll() {
        let result = parse_dice_expression("3d6");
        assert!(result.is_ok());
        let rolls = result.unwrap();
        assert_eq!(rolls.len(), 3);
        for roll in rolls {
            assert_eq!(roll, DiceRoll::D6);
        }
    }

    #[test]
    fn test_parse_d12_favoured() {
        let result = parse_dice_expression("+1d12");
        assert!(result.is_ok());
        let rolls = result.unwrap();
        assert_eq!(rolls.len(), 1);
        assert_eq!(rolls[0], DiceRoll::D12(Favourableness::Favoured));
    }

    #[test]
    fn test_parse_d12_illfavoured() {
        let result = parse_dice_expression("-1d12");
        assert!(result.is_ok());
        let rolls = result.unwrap();
        assert_eq!(rolls.len(), 1);
        assert_eq!(rolls[0], DiceRoll::D12(Favourableness::Illfavoured));
    }

    #[test]
    fn test_invalid_expression() {
        let result = parse_dice_expression("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_dice_type() {
        let result = parse_dice_expression("2d20");
        assert!(result.is_err());
    }
}