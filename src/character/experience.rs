use crate::{
    itinerary::war_gear::{Armour, Shield, Weapon}, 
    skills::skills::Attribute
};

#[derive(Clone, Copy)]
pub enum Reward<'a> {
    Keen(&'a Weapon),
    Fell(&'a Weapon),
    Grievous(&'a Weapon),
    CloseFitting(&'a Armour),
    CunningMake(&'a Armour),
    Reinforced(&'a Shield),
}

#[derive(Clone, Copy)]
pub enum Virtue<'a> {
    Confidence,
    DourHanded,
    Hardiness,
    Mastery,
    Nimbleness,
    Prowess(&'a Attribute),
}

#[derive(Clone, Copy)]
pub struct Experience {
    pub skill_points: u8,
    pub adventure_points: u8,
}