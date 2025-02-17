use std::{
    iter::Map,
    fmt,
};

use crate::{
    itinerary::{useful_items::UsefulItem, war_gear::{CombatProficiency, WarGear}}, 
    skills::skills::{CharacterAttributes, Skill},
    character::experience::{Experience, Reward, Virtue},
};

pub struct Character<'a> {
    pub name: String,
    pub culture: Culture,
    pub calling: Calling,
    
    pub attributes: CharacterAttributes,
    pub skills: Map<String, Skill<'a>>,
    pub combat_proficiencies: Map<String, CombatProficiency>,

    pub parry_base: u8,
    pub endurance_max: u8,
    pub hope_max: u8,

    pub current_state: CharactersCurrentState,
    pub current_conditions: CharacterConditions,

    pub war_grear: WarGear<'a>,
    pub useful_items: Vec<UsefulItem<'a>>,

    pub experience: Experience, 
    pub rewards: Vec<Reward<'a>>,
    pub virtues: Vec<Virtue<'a>>,
}

pub struct CharacterConditions {
    pub is_weary: bool,
    pub is_miserable: bool,
    pub is_wounded: bool,
}

pub struct CharactersCurrentState {
    pub endurance_current: u8,
    pub hope_current: u8,
    pub load: u8,
    pub fatigue: u8,
    pub shadow: u8,
    pub shadow_scars: u8,
    pub treasure: u32,
    pub fellowship_points: u8,
    pub valor: u8,
    pub wisdom: u8,
}

#[derive(Debug)]
pub enum Culture {
    DwarfOfDurinsFolk,
    Bardling,
    ManOfBree,
    ElfOfLindon,
    HobbitOfShire,
    HobbitOfBree,
    RangerOfTheNorth,
}

impl fmt::Display for Culture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Calling {
    TreasureHunter,
    Scholar,
    Champion,
    Capitan,
    Warden,
    Messenger,
}

impl fmt::Display for Calling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum DistinctiveFeature {
    Cunning,
    Faithful,
    Lordly,
    TrueHearted,
    Wary,
}


impl fmt::Display for DistinctiveFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct CharacterBuilder<'a> {
    pub name: Option<String>,
    pub culture: Culture,
    pub calling: Option<Calling>,
    
    pub attributes: Option<CharacterAttributes>,
    pub skills: Map<String, Skill<'a>>,
    pub combat_proficiencies: Map<String, CombatProficiency>,

    pub parry_base: u8,
    pub endurance_max: u8,
    pub hope_max: u8,

    pub current_state: CharactersCurrentState,
    pub current_conditions: CharacterConditions,

    pub war_grear: WarGear<'a>,
    pub useful_items: Vec<UsefulItem<'a>>,

    pub experience: Experience, 
    pub rewards: Vec<Reward<'a>>,
    pub virtues: Vec<Virtue<'a>>,
}

// impl Character<'_> {

//     pub fn new_dwarf_builder<'a>() -> CharacterBuilder<'a> {
//         return CharacterBuilder{
//             name: None,
//             culture: Culture::DwarfOfDurinsFolk,
//             calling: None,
//             attributes: None,
//             // TODO
//         };
//     }

// }