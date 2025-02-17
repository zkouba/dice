pub enum WeaponTypes {
    Axe,
    Bow,
    Spear,
    Sword,
}

pub struct CombatProficiency {
    pub level: u8,
}

pub struct WarGear<'a> {
    pub current_weapon: &'a Weapon,
    pub weapons: Vec<Weapon>,
}

pub struct Weapon {
    pub weapon_type: String,
    pub damage: u8,
    pub injury: u8,
    pub load: u8,

    pub is_held_in_two_hands: bool,
    pub is_keen: bool,
    pub is_fell: bool,
    pub is_grievous: bool,
}

pub struct Armour {
    pub armour_type: String,
    pub protection_dice: u8,
    pub protection_bonus: u8,
    pub load: u8,
}

pub struct Shield {
    pub shield_type: String,
    pub parry: u8,
    pub load: u8,
}