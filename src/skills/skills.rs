use std::fmt;

use crate::{character::experience::Experience, error::error::TOR2Error};


#[derive(Debug)]
pub enum CharacterSkill {
    Awe,
    Athletics,
    Awareness,
    Hunting,
    Song,
    Craft,

    Enharten,
    Travel,
    Insight,
    Healing,
    Courtesy,
    Battle,

    Persuade,
    Stealth,
    Scan,
    Explore,
    Riddle,
    Lore,
}

impl fmt::Display for CharacterSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct CharacterAttributes {
    strength: Attribute,
    heart: Attribute,
    wits: Attribute,
}

pub struct Attribute {
    rating: u8,
    target_number: u8,
}

pub struct Skill<'a> {
    pub name: String,
    target_attribute: &'a Attribute,
    level: u8,
}

impl <'a: 'b, 'b> Skill<'b> {
    pub fn new(name: String, target_attribute: &'a Attribute) -> Self {
        return Self{
            name,
            target_attribute,
            level: 0,
        };
    }

    pub fn increase_by(&mut self, increase: u8, experience: &mut Experience) -> Result<(), TOR2Error> {
        let cost : i16 = match self.level {
            0 => match increase {
                1 => 1,
                2 => 3,
                3 => 6,
                4 => 11,
                _ => -1,
            },
            1 => match increase {
                1 => 2,
                2 => 5,
                3 => 10,
                _ => -1,
            }, 
            2 => match increase {
                1 => 3,
                2 => 8,
                _ => -1,
            }, 
            3 => match increase {
                1 => 5,
                _ => -1,
            }, 
            _ => -1,
        };
        if cost < 0 {
            return Err(TOR2Error::new_standalone(format!("illegal skill increase from {} by {}", self.level, increase)));
        }

        if cost > (experience.skill_points as i16) {
            return Err(TOR2Error::new_standalone(format!("not enough skill points (available: {}, required: {})", experience.skill_points, cost)));
        }

        let cost_b = cost as u8;
        self.level += increase;
        experience.skill_points -= cost_b;

        return Ok(());
    }
}

impl Attribute {
    pub fn new(rating: u8) -> Self {
        return Self{
            rating,
            target_number: 20 - rating,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::character::experience::Experience;

    use super::{Attribute, CharacterSkill, Skill};

    #[test]
    fn test_skill_increase_incremental() {
        let wits = Attribute::new(5);
        let experience = &mut Experience{
            skill_points: 10,
            adventure_points: 0,
        };
        let mut skill = Skill::new(CharacterSkill::Stealth.to_string(), &wits);
        assert_eq!(skill.level, 0);
        skill.increase_by(1, experience).unwrap();
        assert_eq!(skill.level, 1);
        assert_eq!(experience.skill_points, 9);

        skill.increase_by(1, experience).unwrap();
        assert_eq!(skill.level, 2);
        assert_eq!(experience.skill_points, 7);

        skill.increase_by(1, experience).unwrap();
        assert_eq!(skill.level, 3);
        assert_eq!(experience.skill_points, 4);

        let res = skill.increase_by(1, experience);
        assert!(res.is_err());
    }
}
