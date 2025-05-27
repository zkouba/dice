use crate::{
    character::experience::Experience, 
    error::error::TOR2Error
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum WeaponType {
    Axe,
    Bow,
    Spear,
    Sword,
}

#[derive(Clone, Copy)]
pub struct CombatProficiency {
    pub level: u8,
}

#[derive(Clone)]
pub struct WarGear<'a> {
    pub current_weapon: Option<&'a Weapon>,
    pub weapons: Vec<Weapon>,
    pub armour: Option<Armour>,
    pub helm: Option<Armour>,
    pub shield: Option<Shield>,
}

#[derive(Clone, Copy)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: u8,
    pub injury: u8,
    pub load: u8,

    pub is_held_in_two_hands: bool,
    pub is_keen: bool,
    pub is_fell: bool,
    pub is_grievous: bool,
}

#[derive(Clone)]
pub struct Armour {
    pub armour_type: String,
    pub protection_dice: u8,
    pub protection_bonus: u8,
    pub load: u8,
}

#[derive(Clone)]
pub struct Shield {
    pub shield_type: String,
    pub parry: u8,
    pub load: u8,
}

impl CombatProficiency {

    pub fn increase_by(&mut self, increase: u8, experience: &mut Experience) -> Result<(), TOR2Error> {
        let cost : i16 = match self.level {
            0 => match increase {
                1 => 2,
                2 => 6,
                3 => 12,
                _ => -1,
            },
            1 => match increase {
                1 => 4,
                2 => 10,
                _ => -1,
            }, 
            2 => match increase {
                1 => 6,
                _ => -1,
            }, 
            _ => -1,
        };
        if cost < 0 {
            return Err(TOR2Error::new_standalone(format!("illegal combat proficiency increase from {} by {}", self.level, increase)));
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

impl Weapon {

    pub fn new_short_sword() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Sword,
            damage: 3,
            injury: 16,
            load: 1,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_sword() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Sword,
            damage: 4,
            injury: 16,
            load: 2,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_long_sword_singlehanded() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Sword,
            damage: 5,
            injury: 16,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_long_sword_doublehanded() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Sword,
            damage: 5,
            injury: 18,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_short_spear() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Spear,
            damage: 3,
            injury: 14,
            load: 2,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_spear_singlehanded() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Spear,
            damage: 4,
            injury: 14,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_spear_doublehanded() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Spear,
            damage: 4,
            injury: 16,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_great_spear() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Spear,
            damage: 5,
            injury: 16,
            load: 4,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_axe() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Axe,
            damage: 5,
            injury: 18,
            load: 2,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_long_hafted_axe_singlehand() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Axe,
            damage: 6,
            injury: 18,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: false,
        };
    }

    pub fn new_long_hafted_axe_doublehand() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Axe,
            damage: 6,
            injury: 20,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_great_axe() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Axe,
            damage: 7,
            injury: 20,
            load: 4,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_mattock() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Axe,
            damage: 7,
            injury: 18,
            load: 3,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_bow() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Bow,
            damage: 3,
            injury: 14,
            load: 2,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }

    pub fn new_great_bow() -> Weapon {
        return Weapon{
            weapon_type: WeaponType::Bow,
            damage: 4,
            injury: 16,
            load: 4,
            is_fell: false,
            is_grievous: false,
            is_keen: false,
            is_held_in_two_hands: true,
        };
    }
}

impl Armour {
    pub fn new_leather_shirt() -> Armour {
        return Armour{
            armour_type: String::from("leather shirt"),
            protection_dice: 1,
            protection_bonus: 0,
            load: 3,
        };
    }

    pub fn new_leather_corslet() -> Armour {
        return Armour{
            armour_type: String::from("leather corslet"),
            protection_dice: 2,
            protection_bonus: 0,
            load: 6,
        };
    }

    pub fn new_mail_shirt() -> Armour {
        return Armour{
            armour_type: String::from("mail shirt"),
            protection_dice: 3,
            protection_bonus: 0,
            load: 9,
        };
    }

    pub fn new_coat_mail() -> Armour {
        return Armour{
            armour_type: String::from("coat of mail"),
            protection_dice: 4,
            protection_bonus: 0,
            load: 12,
        };
    }

    pub fn new_helm() -> Armour {
        return Armour{
            armour_type: String::from("helm"),
            protection_dice: 1,
            protection_bonus: 0,
            load: 4,
        };
    }
}

impl Shield {
    pub fn new_buckler() -> Shield {
        return Shield{
            shield_type: String::from("buckler"),
            parry: 1,
            load: 2,
        };
    }

    pub fn new_shield() -> Shield {
        return Shield{
            shield_type: String::from("shield"),
            parry: 2,
            load: 4,
        };
    }

    pub fn new_great_shield() -> Shield {
        return Shield{
            shield_type: String::from("great shield"),
            parry: 3,
            load: 6,
        };
    }
}