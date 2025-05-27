use std::cmp::max;
use std::ops::{Add, AddAssign};
use std::str::FromStr;
use clap::builder::ValueParserFactory;
use clap::ValueEnum;
use rand::random_range;
use crate::dice::rolls::Favourableness::{Favoured, Illfavoured, Neutral};
use crate::dice::rolls::RollResultType::{AutoFailure, AutoSuccess, Numerical};

#[derive(Copy, Clone, Debug)]
pub struct DiceRoll {
    result_type: RollResultType,
    total: u8,
    feat_die: u8,
    success_dice: SuccessDiceRoll,
}

impl ToString for DiceRoll {
    fn to_string(&self) -> String {
        match self.result_type {
            AutoSuccess => {
                return format!("(Auto-success; extra: {:?})", self.success_dice.extra_successes);
            }
            AutoFailure => {
                return format!("(Auto-failure; extra: {:?})", self.success_dice.extra_successes);
            }
            Numerical => {
                let feat_str = if self.feat_die == FEAT_DIE_GANDALF {
                    "Gandalf".to_owned()
                } else if self.feat_die == FEAT_DIE_SAURON {
                    "Sauron".to_owned()
                } else {
                    self.feat_die.to_string()
                };
                return format!("({}; feat: {}; extra: {})", self.total, feat_str, self.success_dice.extra_successes);
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
            Favourableness::Favoured => {
                match other {
                    Favoured => {
                        return self;
                    }
                    Illfavoured => {
                        return Neutral(true);
                    }
                    Neutral(combined) => {
                        if combined {
                            return other;
                        } else {
                            return self;
                        }
                    }
                }
            }
            Favourableness::Illfavoured => {
                match other {
                    Favoured => {
                        return Neutral(true);
                    }
                    Illfavoured => {
                        return self;
                    }
                    Neutral(combined) => {
                        if combined {
                            return other;
                        } else {
                            return self;
                        }
                    }
                }
            }
            Favourableness::Neutral(combined) => {
                if combined {
                    return self;
                }
                match other {
                    Favourableness::Favoured => {
                        return other;
                    }
                    Favourableness::Illfavoured => {
                        return other;
                    }
                    Favourableness::Neutral(other_combined) => {
                        if other_combined {
                            return other;
                        } else {
                            return Neutral(false);
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
    let feat_die_result = match favourableness {
        Favourableness::Favoured => {
            let x1 = d12();
            let x2 = d12();
            max(x1, x2)
        }
        Favourableness::Illfavoured => {
            let x1 = d12();
            let x2 = d12();
            max(x1, x2)
        }
        Favourableness::Neutral(_) => {
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