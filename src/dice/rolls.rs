use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use rand::random_range;

#[derive(Debug)]
pub struct RollResult {
    pub die: DiceRoll,
    pub value: u8,
}


#[derive(Clone, Debug, PartialEq)]
pub enum DiceRoll {
    D6,
    D12(Favourableness),
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceRoll::D6 => f.write_str("d6"),
            DiceRoll::D12(_) => f.write_str("d12"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Favourableness {
    Favoured,
    Illfavoured,
    Neutral(bool),
}

impl Favourableness {
    pub fn combine(&self, other: &Favourableness) -> Self {
        if self.eq(&Favourableness::Neutral(true)) || other.eq(&Favourableness::Neutral(true)) {
            return Favourableness::Neutral(true);
        }
        match (self, other) {
            (Favourableness::Favoured, Favourableness::Illfavoured) => Favourableness::Neutral(true),
            (Favourableness::Illfavoured, Favourableness::Favoured) => Favourableness::Neutral(true),
            (Favourableness::Favoured, Favourableness::Neutral(false)) => Favourableness::Favoured,
            (Favourableness::Neutral(false), Favourableness::Favoured) => Favourableness::Favoured,
            (Favourableness::Illfavoured, Favourableness::Neutral(false)) => Favourableness::Illfavoured,
            (Favourableness::Neutral(false), Favourableness::Illfavoured) => Favourableness::Illfavoured,
            _ => self.clone(),
        }
    }
}

impl Add for Favourableness {
    type Output = Favourableness;
    fn add(self, other: Self) -> Self::Output {
        match self {
            Favourableness::Favoured => {
                match other {
                    Favourableness::Favoured => {
                        self
                    }
                    Favourableness::Illfavoured => {
                        Favourableness::Neutral(true)
                    }
                    Favourableness::Neutral(combined) => {
                        if combined {
                            other
                        } else {
                            self
                        }
                    }
                }
            }
            Favourableness::Illfavoured => {
                match other {
                    Favourableness::Favoured => {
                        Favourableness::Neutral(true)
                    }
                    Favourableness::Illfavoured => {
                        self
                    }
                    Favourableness::Neutral(combined) => {
                        if combined {
                            other
                        } else {
                            self
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
                        other
                    }
                    Favourableness::Illfavoured => {
                        other
                    }
                    Favourableness::Neutral(other_combined) => {
                        if other_combined {
                            other
                        } else {
                            Favourableness::Neutral(false)
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

impl Display for Favourableness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Favourableness::Favoured => f.write_str("+"),
            Favourableness::Illfavoured => f.write_str("-"),
            Favourableness::Neutral(_) => f.write_str(""),
        }
    }
}

pub fn roll(roll_command: DiceRoll) -> RollResult {
    match roll_command {
        DiceRoll::D6 => d6(),
        DiceRoll::D12(favourableness) => d12_fav(favourableness),
    }
}

pub fn d12_fav(favourableness: Favourableness) -> RollResult {
    RollResult {
        die: DiceRoll::D12(favourableness),
        value: match favourableness {
            Favourableness::Favoured => {
                // formula.push_str("2d12+ ");
                let x1 = d12();
                let x2 = d12();
                max(x1.value, x2.value)
            }
            Favourableness::Illfavoured => {
                // formula.push_str("2d12- ");
                let x1 = d12();
                let x2 = d12();
                min(x1.value, x2.value)
            }
            Favourableness::Neutral(_) => {
                // formula.push_str("1d12 ");
                d12().value
            }
        }
    }
}

pub fn d12() -> RollResult {
    RollResult{
        die: DiceRoll::D12(Favourableness::Neutral(false)),
        value: random_range(0..12)
    }
}

pub fn d6() -> RollResult {
    RollResult {
        die: DiceRoll::D6,
        value: random_range(1..7),
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::rolls::*;

    #[test]
    pub fn favourableness_combine_test() {
        assert_eq!(Favourableness::Neutral(true).combine(&Favourableness::Neutral(true)), Favourableness::Neutral(true));
        assert_eq!(Favourableness::Neutral(true).combine(&Favourableness::Favoured), Favourableness::Neutral(true));
        assert_eq!(Favourableness::Neutral(true).combine(&Favourableness::Illfavoured), Favourableness::Neutral(true));
        assert_eq!(Favourableness::Illfavoured.combine(&Favourableness::Neutral(true)), Favourableness::Neutral(true));
        assert_eq!(Favourableness::Favoured.combine(&Favourableness::Neutral(true)), Favourableness::Neutral(true));

        assert_eq!(Favourableness::Neutral(false).combine(&Favourableness::Neutral(false)), Favourableness::Neutral(false));
        assert_eq!(Favourableness::Favoured.combine(&Favourableness::Neutral(false)), Favourableness::Favoured);
        assert_eq!(Favourableness::Illfavoured.combine(&Favourableness::Neutral(false)), Favourableness::Illfavoured);
        assert_eq!(Favourableness::Neutral(false).combine(&Favourableness::Favoured), Favourableness::Favoured);
        assert_eq!(Favourableness::Neutral(false).combine(&Favourableness::Illfavoured), Favourableness::Illfavoured);

        assert_eq!(Favourableness::Favoured.combine(&Favourableness::Favoured), Favourableness::Favoured);
        assert_eq!(Favourableness::Illfavoured.combine(&Favourableness::Illfavoured), Favourableness::Illfavoured);

        assert_eq!(Favourableness::Favoured.combine(&Favourableness::Illfavoured), Favourableness::Neutral(true));
        assert_eq!(Favourableness::Illfavoured.combine(&Favourableness::Favoured), Favourableness::Neutral(true));
    }

    #[test]
    pub fn roll_d6_test() {
        let actual = d6();
        println!("D6 result: {:?}", actual.value);
        assert!(actual.value < 7);
    }

    #[test]
    pub fn roll_d12_test() {
        let actual = d12();
        println!("D12 result: {:?}", actual.value);
        assert!(actual.value < 12);
    }
}