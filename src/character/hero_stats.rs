use std::{
    collections::HashMap, fmt, iter::Map
};

use crate::{
    character::{
        dwarfs::DwarfCharacterAttributeBuilder,
        experience::{Experience, Reward, Virtue}
    }, 
    itinerary::{
        useful_items::UsefulItem, 
        war_gear::{CombatProficiency, WarGear, WeaponType}
    }, 
    skills::skills::{Attribute, CharacterAttribute, CharacterSkill, Skill},
};

pub struct Character<'a> {
    pub name: String,
    pub culture: Culture,
    pub calling: Calling,
    pub distinctive_features: Vec<DistinctiveFeature>,
    
    pub attributes: HashMap<CharacterAttribute, Attribute>,
    pub skills: Map<String, Skill>,
    pub combat_proficiencies: Map<String, CombatProficiency>,

    pub parry_base: u8,
    pub endurance_max: u8,
    pub hope_max: u8,
    pub valor: u8,
    pub wisdom: u8,

    pub current_state: CharactersCurrentState,
    pub current_conditions: CharacterConditions,

    pub war_grear: WarGear<'a>,
    pub useful_items: Vec<UsefulItem<'a>>,

    pub experience: Experience, 
    pub rewards: Vec<Reward<'a>>,
    pub virtues: Vec<Virtue<'a>>,
}

#[derive(Clone, Copy)]
pub struct CharacterConditions {
    pub is_weary: bool,
    pub is_miserable: bool,
    pub is_wounded: bool,
}

#[derive(Clone, Copy)]
pub struct CharactersCurrentState {
    pub endurance_current: u8,
    pub hope_current: u8,
    pub load: u8,
    pub fatigue: u8,
    pub shadow: u8,
    pub shadow_scars: u8,
    pub treasure: u32,
    pub fellowship_points: u8,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum DistinctiveFeature {
    Bold,
    Burglary,
    Cunning,
    Eager,
    EnemyLore,
    Faithful,
    Fair,
    FairSpoken,
    Fierce,
    FolkLore,
    Generous,
    Honourable,
    Inquisitive,
    KeenEyed,
    Lordly,
    Leadership,
    Merry,
    Patient,
    Proud,
    RhymesOfLore,
    Rustic,
    Secretive,
    ShadowLore,
    Stern,
    Subtle,
    Swift,
    Tall,
    TrueHearted,
    Wary,
    Wilful,
}


impl fmt::Display for DistinctiveFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct CharacterBuilder<'a> {
    pub name: Option<String>,
    pub culture: Culture,
    pub calling: Option<Calling>,
    pub distinctive_features: Vec<DistinctiveFeature>,
    
    pub attributes: HashMap<CharacterAttribute, Attribute>,
    pub skills: HashMap<CharacterSkill, Skill>,
    pub combat_proficiencies: HashMap<WeaponType, CombatProficiency>,

    pub parry_base: u8,
    pub endurance_max: u8,
    pub hope_max: u8,
    pub valor: u8,
    pub wisdom: u8,

    pub current_state: CharactersCurrentState,
    pub current_conditions: CharacterConditions,

    pub war_grear: WarGear<'a>,
    pub useful_items: Vec<UsefulItem<'a>>,

    pub experience: Experience, 
    pub rewards: Vec<Reward<'a>>,
    pub virtues: Vec<Virtue<'a>>,
}

impl Character<'_> {

    // pub fn new_dwarf_builder<'a>() -> DwarfCharacterAttributeBuilder<'a> {
    //     // TODO !!!
    //     let builder = CharacterBuilder{
    //         name: None,
    //         culture: Culture::DwarfOfDurinsFolk,
    //         calling: None,
    //         distinctive_features: Vec::new(),
    //         attributes: HashMap::new(),
    //         skills: HashMap::new(),
    //         combat_proficiencies: HashMap::new(),
    //         parry_base: 0,
    //         hope_max: 0,
    //         endurance_max: 0,
    //         valor: 0,
    //         wisdom: 0,
    //         current_state: CharactersCurrentState{
    //             fatigue: 0,
    //             endurance_current: 0,
    //             fellowship_points: 0,
    //             hope_current: 0,
    //             load: 0,
    //             shadow: 0,
    //             shadow_scars: 0,
    //             treasure: 0,
    //         },
    //         current_conditions: CharacterConditions{
    //             is_miserable: false,
    //             is_weary: false,
    //             is_wounded: false,
    //         },
    //         experience: Experience {
    //             skill_points: 10,
    //             adventure_points: 0,
    //         },
    //         rewards: Vec::new(),
    //         virtues: Vec::new(),
    //         useful_items: Vec::new(),
    //         war_grear: WarGear{
    //             current_weapon: None,
    //             weapons: Vec::new(),
    //         },
    //     };
    //     return DwarfCharacterAttributeBuilder{builder};
    // }

}
