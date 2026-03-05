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
pub enum DiceType {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Display for DiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceType::D2 => f.write_str("d2"),
            DiceType::D4 => f.write_str("d4"),
            DiceType::D6 => f.write_str("d6"),
            DiceType::D8 => f.write_str("d8"),
            DiceType::D10 => f.write_str("d10"),
            DiceType::D12 => f.write_str("d12"),
            DiceType::D20 => f.write_str("d20"),
            DiceType::D100 => f.write_str("d100"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiceRoll {
    pub die_type: DiceType,
    pub favourableness: Favourableness,
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.favourableness, self.die_type))
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

pub fn roll_fav(roll_cmd: DiceRoll) -> RollResult {
    RollResult {
        die: roll_cmd.clone(),
        value: match roll_cmd.favourableness {
            Favourableness::Favoured => {
                let x1 = roll(roll_cmd.die_type.clone());
                let x2 = roll(roll_cmd.die_type);
                max(x1.value, x2.value)
            }
            Favourableness::Illfavoured => {
                let x1 = roll(roll_cmd.die_type.clone());
                let x2 = roll(roll_cmd.die_type);
                min(x1.value, x2.value)
            }
            Favourableness::Neutral(_) => {
                roll(roll_cmd.die_type).value
            }
        }
    }
}

pub fn roll(roll_command: DiceType) -> RollResult {
    match roll_command {
        DiceType::D2 => d2(),
        DiceType::D4 => d4(),
        DiceType::D6 => d6(),
        DiceType::D8 => d8(),
        DiceType::D10 => d10(),
        DiceType::D12 => d12(),
        DiceType::D20 => d20(),
        DiceType::D100 => d100(),
    }
}



pub fn d100() -> RollResult {
    RollResult{
        die: DiceRoll{die_type: DiceType::D100, favourableness: Favourableness::Neutral(false)},
        value: random_range(0..101)
    }
}

pub fn d20() -> RollResult {
    RollResult{
        die: DiceRoll{die_type: DiceType::D20, favourableness: Favourableness::Neutral(false)},
        value: random_range(0..21)
    }
}

pub fn d12() -> RollResult {
    RollResult{
        die: DiceRoll{die_type: DiceType::D12, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..13)
    }
}

pub fn d10() -> RollResult {
    RollResult {
        die: DiceRoll{die_type: DiceType::D10, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..11),
    }
}

pub fn d8() -> RollResult {
    RollResult {
        die: DiceRoll{die_type: DiceType::D8, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..9),
    }
}

pub fn d6() -> RollResult {
    RollResult {
        die: DiceRoll{die_type: DiceType::D6, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..7),
    }
}

pub fn d4() -> RollResult {
    RollResult {
        die: DiceRoll{die_type: DiceType::D4, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..5),
    }
}

pub fn d2() -> RollResult {
    RollResult {
        die: DiceRoll{die_type: DiceType::D2, favourableness: Favourableness::Neutral(false)},
        value: random_range(1..3),
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

}