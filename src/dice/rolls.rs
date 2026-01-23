use std::cmp::max;
use std::fmt::Display;
use std::ops::{Add, AddAssign};
use rand::random_range;
use crate::dice::rolls::Favourableness::{Favoured, Illfavoured, Neutral};
use crate::dice::rolls::RollResultType::{AutoFailure, AutoSuccess, Numerical};

#[derive(Clone, Debug)]
pub struct DiceRoll {
    result_type: RollResultType,
    total: u8,
    feat_die: u8,
    success_dice: SuccessDiceRoll,
    formula: String,
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.result_type {
            AutoSuccess => {
                write!(
                    f,
                    "({}) => (Auto-success; extra: {})",
                    self.formula,
                    self.success_dice.extra_successes
                )
            }
            AutoFailure => {
                write!(
                    f,
                    "({}) => (Auto-failure; extra: {})",
                    self.formula,
                    self.success_dice.extra_successes
                )
            }
            Numerical => {
                write!(f, "({}) => ({}; feat: ", self.formula, self.total)?;
                match self.feat_die {
                    FEAT_DIE_GANDALF => write!(f, "Gandalf")?,
                    FEAT_DIE_SAURON => write!(f, "Sauron")?,
                    other => write!(f, "{other}")?,
                }
                write!(f, "; extra: {})", self.success_dice.extra_successes)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SuccessDiceRoll {
    pub sum: u8,
    pub extra_successes: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum RollResultType {
    AutoSuccess, AutoFailure, Numerical,
}

#[derive(Copy, Clone, Debug)]
pub enum Favourableness {
    Favoured,
    Illfavoured,
    Neutral(bool),
}

impl Add for Favourableness {
    type Output = Favourableness;
    fn add(self, other: Self) -> Self::Output {
        match self {
            Favoured => {
                match other {
                    Favoured => {
                        self
                    }
                    Illfavoured => {
                        Neutral(true)
                    }
                    Neutral(combined) => {
                        if combined {
                            other
                        } else {
                            self
                        }
                    }
                }
            }
            Illfavoured => {
                match other {
                    Favoured => {
                        Neutral(true)
                    }
                    Illfavoured => {
                        self
                    }
                    Neutral(combined) => {
                        if combined {
                            other
                        } else {
                            self
                        }
                    }
                }
            }
            Neutral(combined) => {
                if combined {
                    return self;
                }
                match other {
                    Favourableness::Favoured => {
                        other
                    }
                    Favourableness::Illfavoured => {
                        other
                    }
                    Favourableness::Neutral(other_combined) => {
                        if other_combined {
                            other
                        } else {
                            Neutral(false)
                        }
                    }
                }
            }
        }
    }
}

impl AddAssign for Favourableness {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

pub const FEAT_DIE_GANDALF: u8 = 11;
pub const FEAT_DIE_SAURON: u8 = 0;

pub fn d12() -> u8 {
    random_range(0..12)
}

pub fn d6() -> u8 {
    random_range(1..7)
}

fn roll_n_d6(n: u8, is_weary: bool) -> SuccessDiceRoll {
    let mut roll = SuccessDiceRoll{
        sum: 0,
        extra_successes: 0,
    };
    for _ in 0..n {
        let x = d6();
        if x > 3 || !is_weary {
            roll.sum += x;
        }
        if x == 6 {
            roll.extra_successes += 1;
        }
    }
    roll
}

pub fn roll(
    succ_dice: u8,
    favourableness: Favourableness,
    extra_dice: i8,
    spending_hope: bool,
    is_weary:bool,
    is_miserable: bool,
    is_inspired: bool,
    is_assisted: bool,
) -> DiceRoll {
    let mut formula = String::new();
    let feat_die_result = match favourableness {
        Favourableness::Favoured => {
            formula.push_str("2d12+ ");
            let x1 = d12();
            let x2 = d12();
            max(x1, x2)

        }
        Favourableness::Illfavoured => {
            formula.push_str("2d12- ");
            let x1 = d12();
            let x2 = d12();
            max(x1, x2)
        }
        Favourableness::Neutral(_) => {
            formula.push_str("1d12 ");
            d12()
        }
    };
    let mut succ_dice_cnt = succ_dice;
    if spending_hope {
        if is_inspired {
            succ_dice_cnt += 2;
        } else {
            succ_dice_cnt += 1;
        }
    }
    if is_assisted {
        succ_dice_cnt += 1;
    }
    succ_dice_cnt = max(0, (succ_dice_cnt as i8) + extra_dice) as u8;
    formula.push_str(format!("{}d6 ", succ_dice_cnt).as_str());
    let succ_dice_result = roll_n_d6(succ_dice_cnt, is_weary);
    let result_type = if is_miserable && feat_die_result == FEAT_DIE_SAURON {
        AutoFailure
    } else if feat_die_result == FEAT_DIE_GANDALF {
        AutoSuccess
    } else {
        Numerical
    };
    DiceRoll{
        result_type,
        total: feat_die_result + succ_dice_result.sum,
        feat_die: feat_die_result,
        success_dice: succ_dice_result,
        formula: formula,
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::rolls::*;

    #[test]
    pub fn roll_d6_test() {
        let actual = d6();
        println!("D6 result: {:?}", actual);
        assert!(actual > 0 && actual < 7);
    }

    #[test]
    pub fn roll_d12_test() {
        let actual = d12();
        println!("D12 result: {:?}", actual);
        assert!(actual >= 0 && actual < 12);
    }

    #[test]
    pub fn roll_test() {
        let succ_dice = 2;
        let actual = roll(
            succ_dice, Neutral(false), 0, false, false, false, false, false
        );
        println!("Roll({:?}d6, 1d12) result: {:?}", succ_dice, actual.to_string());
        assert!(actual.feat_die >= 0 && actual.feat_die < 12);
    }
}