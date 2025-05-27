use crate::{
    character::hero_stats::{Calling, CharacterBuilder, DistinctiveFeature}, 
    error::error::TOR2Error, 
    itinerary::{
        useful_items::UsefulItem, 
        war_gear::{Armour, CombatProficiency, Shield, Weapon, WeaponType}
    }, 
    skills::skills::{Attribute, CharacterAttribute, CharacterSkill, Skill},
};


pub struct DwarfCharacterAttributeBuilder<'a> {
    pub builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterCulturalCombatProficiencyBuilder<'a> {
    pub builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterSkillBuilder<'a> {
    builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterCallingBuilder<'a> {
    builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterFavouredSkillBuilder<'a> {
    cultural_fav_skill_chosen: bool,
    pub builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterDistFeaturesBuilder<'a> {
    feature_cnt: u8,
    builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterInventoryBuilder<'a> {
    builder: CharacterBuilder<'a>,
}

pub struct DwarfCharacterVirtuesAndRewardsBuilder<'a> {
    virtue_set: bool,
    reward_set: bool,
    builder: CharacterBuilder<'a>,
}

pub enum DwarfAttributes {
    /// Strength, Heart, Wits: 7, 2, 5
    Option_1,

    /// Strength, Heart, Wits: 7, 3, 4
    Option_2,

    /// Strength, Heart, Wits: 6, 3, 5
    Option_3,

    /// Strength, Heart, Wits: 6, 4, 4
    Option_4,

    /// Strength, Heart, Wits: 5, 4, 5
    Option_5,

    /// Strength, Heart, Wits: 6, 2, 6
    Option_6,
}

impl DwarfCharacterAttributeBuilder<'_> {
    pub fn with_attributes(&mut self, attr_option: DwarfAttributes) -> DwarfCharacterCulturalCombatProficiencyBuilder<'_> {
        match attr_option {
            DwarfAttributes::Option_1 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 7));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 2));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 5));
            },
            DwarfAttributes::Option_2 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 7));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 3));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 4));
            },
            DwarfAttributes::Option_3 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 6));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 3));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 5));
            },
            DwarfAttributes::Option_4 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 6));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 4));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 4));
            },
            DwarfAttributes::Option_5 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 5));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 4));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 5));
            },
            DwarfAttributes::Option_6 => {
                self.builder.attributes.insert(CharacterAttribute::Strength, Attribute::new(CharacterAttribute::Strength, 6));
                self.builder.attributes.insert(CharacterAttribute::Heart, Attribute::new(CharacterAttribute::Heart, 2));
                self.builder.attributes.insert(CharacterAttribute::Wits, Attribute::new(CharacterAttribute::Wits, 6));
            },
        }
        self.builder.endurance_max = self.builder.attributes.get(&CharacterAttribute::Strength).unwrap().rating + 22;
        self.builder.hope_max = self.builder.attributes.get(&CharacterAttribute::Heart).unwrap().rating + 8;
        self.builder.parry_base = self.builder.attributes.get(&CharacterAttribute::Wits).unwrap().rating + 10;
        let mut cpBuilder = DwarfCharacterCulturalCombatProficiencyBuilder{
            builder: self.builder.clone(),
        };

        cpBuilder.builder.skills.insert(CharacterSkill::Awe, Skill{
            name: CharacterSkill::Awe, target_attribute: CharacterAttribute::Strength, level: 2, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Athletics, Skill{
            name: CharacterSkill::Athletics, target_attribute: CharacterAttribute::Strength, level: 1, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Awareness, Skill{
            name: CharacterSkill::Awareness, target_attribute: CharacterAttribute::Strength, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Hunting, Skill{
            name: CharacterSkill::Hunting, target_attribute: CharacterAttribute::Strength, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Song, Skill{
            name: CharacterSkill::Song, target_attribute: CharacterAttribute::Strength, level: 1, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Craft, Skill{
            name: CharacterSkill::Craft, target_attribute: CharacterAttribute::Strength, level: 2, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Craft, Skill{
            name: CharacterSkill::Craft, target_attribute: CharacterAttribute::Strength, level: 2, is_favoured: false,
        });

        cpBuilder.builder.skills.insert(CharacterSkill::Enharten, Skill{
            name: CharacterSkill::Enharten, target_attribute: CharacterAttribute::Heart, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Travel, Skill{
            name: CharacterSkill::Travel, target_attribute: CharacterAttribute::Heart, level: 3, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Insight, Skill{
            name: CharacterSkill::Insight, target_attribute: CharacterAttribute::Heart, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Healing, Skill{
            name: CharacterSkill::Healing, target_attribute: CharacterAttribute::Heart, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Courtesy, Skill{
            name: CharacterSkill::Courtesy, target_attribute: CharacterAttribute::Heart, level: 1, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Battle, Skill{
            name: CharacterSkill::Battle, target_attribute: CharacterAttribute::Heart, level: 1, is_favoured: false,
        });

        cpBuilder.builder.skills.insert(CharacterSkill::Persuade, Skill{
            name: CharacterSkill::Persuade, target_attribute: CharacterAttribute::Wits, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Stealth, Skill{
            name: CharacterSkill::Stealth, target_attribute: CharacterAttribute::Wits, level: 0, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Scan, Skill{
            name: CharacterSkill::Scan, target_attribute: CharacterAttribute::Wits, level: 3, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Explore, Skill{
            name: CharacterSkill::Explore, target_attribute: CharacterAttribute::Wits, level: 2, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Riddle, Skill{
            name: CharacterSkill::Riddle, target_attribute: CharacterAttribute::Wits, level:2, is_favoured: false,
        });
        cpBuilder.builder.skills.insert(CharacterSkill::Lore, Skill{
            name: CharacterSkill::Lore, target_attribute: CharacterAttribute::Wits, level: 1, is_favoured: false,
        });

        return cpBuilder;
    }
}

impl DwarfCharacterCulturalCombatProficiencyBuilder<'_> {
    pub fn with_cultural_combat_proficiency(&mut self, weapon_type: WeaponType) -> Result<DwarfCharacterSkillBuilder, TOR2Error> {
        match weapon_type {
            WeaponType::Axe => {
                self.builder.combat_proficiencies.insert(WeaponType::Axe, CombatProficiency{level: 2});
                self.builder.combat_proficiencies.insert(WeaponType::Bow, CombatProficiency{level: 0});
                self.builder.combat_proficiencies.insert(WeaponType::Spear, CombatProficiency{level: 0});
                self.builder.combat_proficiencies.insert(WeaponType::Sword, CombatProficiency{level: 0});
                return Ok(DwarfCharacterSkillBuilder{builder: self.builder.clone()});
            } 
            WeaponType::Sword => {
                self.builder.combat_proficiencies.insert(WeaponType::Axe, CombatProficiency{level: 0});
                self.builder.combat_proficiencies.insert(WeaponType::Bow, CombatProficiency{level: 0});
                self.builder.combat_proficiencies.insert(WeaponType::Spear, CombatProficiency{level: 0});
                self.builder.combat_proficiencies.insert(WeaponType::Sword, CombatProficiency{level: 2});
                return Ok(DwarfCharacterSkillBuilder{builder: self.builder.clone()});
            },
            _ => {
                return Err(TOR2Error::new_standalone("weapon type not native to Dwarfs of Durin's folk".to_owned()));
            }
        }
    }
}

impl DwarfCharacterSkillBuilder<'_> {
    pub fn increase_skill(&mut self, skill_name: CharacterSkill) -> Result<&Self, TOR2Error> {
        let skill = self.builder.skills.get_mut(&skill_name).unwrap();
        skill.increase_by(1, &mut self.builder.experience)?;
        return Ok(self);
    }

    pub fn set_favoured_skill(&mut self, skill: CharacterSkill) -> Result<&Self, TOR2Error> {
        match skill {
            CharacterSkill::Craft => {
                if self.builder.skills.get(&CharacterSkill::Travel).unwrap().is_favoured {
                    return Err(TOR2Error::new_standalone("can choose only one skill as favoured due to culture".to_owned()));
                }
                self.builder.skills.get_mut(&CharacterSkill::Craft).unwrap().is_favoured = true;
                return Ok(self);
            },
            CharacterSkill::Travel => {
                if self.builder.skills.get(&CharacterSkill::Craft).unwrap().is_favoured {
                    return Err(TOR2Error::new_standalone("can choose only one skill as favoured due to culture".to_owned()));
                }
                self.builder.skills.get_mut(&CharacterSkill::Travel).unwrap().is_favoured = true;
                return Ok(self);
            },
            _ => {
                return Err(TOR2Error::new_standalone(format!("the {} skill cannot by favoured by dwarfs", skill.to_string())));
            }
        }
        return Ok(self);
    }

    pub fn increase_combat_proficiency(&mut self, weapon_type: WeaponType) -> Result<&Self, TOR2Error> {
        let combat_proficiency = self.builder.combat_proficiencies.get_mut(&weapon_type).unwrap();
        combat_proficiency.increase_by(1, &mut self.builder.experience)?;
        return Ok(self);
    }

    pub fn next_step(&self) -> DwarfCharacterCallingBuilder {
        return DwarfCharacterCallingBuilder{
            builder: self.builder.clone(),
        };
    }
}

impl DwarfCharacterCallingBuilder<'_> {
    pub fn with_calling(&mut self, calling: Calling) -> DwarfCharacterFavouredSkillBuilder {
        self.builder.calling = Some(calling);
        self.builder.distinctive_features.push(
            match calling {
                Calling::Capitan =>         DistinctiveFeature::Leadership,
                Calling::Champion =>        DistinctiveFeature::EnemyLore,
                Calling::Messenger =>       DistinctiveFeature::FolkLore,
                Calling::Scholar =>         DistinctiveFeature::RhymesOfLore,
                Calling::TreasureHunter =>  DistinctiveFeature::Burglary,
                Calling::Warden =>          DistinctiveFeature::ShadowLore,
            }
        );
        return DwarfCharacterFavouredSkillBuilder{cultural_fav_skill_chosen: false, builder: self.builder.clone()};
    }
}

impl DwarfCharacterFavouredSkillBuilder<'_> {
    pub fn with_favoured_skill(&mut self, favoured_skill: CharacterSkill) -> Result<DwarfCharacterDistFeaturesBuilder, TOR2Error> {
        match favoured_skill {
            CharacterSkill::Craft | CharacterSkill::Travel => {
                if self.cultural_fav_skill_chosen {
                    return Err(TOR2Error::new_standalone("favoured skill based on culture already chosen".to_owned()));
                }
                self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                self.cultural_fav_skill_chosen = true;
            },
            CharacterSkill::Battle | CharacterSkill::Enharten | CharacterSkill::Persuade => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::Capitan {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf captain", favoured_skill).to_owned()));
                }
            },
            CharacterSkill::Athletics | CharacterSkill::Awe | CharacterSkill::Hunting => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::Champion {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf champion", favoured_skill).to_owned()));
                }
            },
            CharacterSkill::Courtesy | CharacterSkill::Song | CharacterSkill::Travel => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::Messenger {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf messenger", favoured_skill).to_owned()));
                }
            },
            CharacterSkill::Craft | CharacterSkill::Lore | CharacterSkill::Riddle => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::Scholar {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf scholar", favoured_skill).to_owned()));
                }
            },
            CharacterSkill::Explore | CharacterSkill::Scan | CharacterSkill::Stealth => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::TreasureHunter {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf treasure hunter", favoured_skill).to_owned()));
                }
            },
            CharacterSkill::Awareness | CharacterSkill::Healing | CharacterSkill::Insight => {
                if self.builder.calling.is_some() && self.builder.calling.clone().unwrap() == Calling::Warden {
                    self.builder.skills.get_mut(&favoured_skill).unwrap().is_favoured = true;
                } else {
                    return Err(TOR2Error::new_standalone(format!("{} skill can't be favoured for a dwarf warden", favoured_skill).to_owned()));
                }
            },
        }
        return Ok(DwarfCharacterDistFeaturesBuilder{feature_cnt: 0, builder: self.builder.clone()});
    }
}

impl DwarfCharacterDistFeaturesBuilder<'_> {
    pub fn with_distinctive_feature(&mut self, dist_feature: DistinctiveFeature) -> Result<&mut Self, TOR2Error> {
        if self.feature_cnt >= 2 {
            return Err(TOR2Error::new_standalone("maximum number of distinctive features already chosen".to_owned()));
        }
        match dist_feature {
            DistinctiveFeature::Cunning | 
            DistinctiveFeature::Fierce | 
            DistinctiveFeature::Lordly | 
            DistinctiveFeature::Proud | 
            DistinctiveFeature::Secretive | 
            DistinctiveFeature::Stern | 
            DistinctiveFeature::Wary | 
            DistinctiveFeature::Wilful => {
                if self.builder.distinctive_features.contains(&dist_feature) {

                }
                self.builder.distinctive_features.push(dist_feature);
                self.feature_cnt += 1;
                return Ok(self);
            },
            _ => {
                return Err(TOR2Error::new_standalone("invalid distinctive feature for a Dwarf of Durin's folk".to_owned()));
            }
        }
    }

    pub fn next_step(&self) -> Result<DwarfCharacterInventoryBuilder, TOR2Error> {
        if self.feature_cnt < 2 {
            return Err(TOR2Error::new_standalone("not enough distinctive features have been chosen".to_owned()));
        }
        return Ok(DwarfCharacterInventoryBuilder{
            builder: self.builder.clone(),
        });
    }
}

impl DwarfCharacterInventoryBuilder<'_> {
    pub fn with_weapon(&mut self, weapon: Weapon) -> Result<&mut Self, TOR2Error> {
        if self.builder.combat_proficiencies.get(&weapon.weapon_type).unwrap().level < 1 {
            return Err(TOR2Error::new_standalone(
                "starting war gear can contain only weapon types for which the charater has a combat proficiency".to_owned()
            ));
        }
        for equipped_weapon in self.builder.war_grear.weapons.clone() {
            if equipped_weapon.weapon_type == weapon.weapon_type {
                return Err(TOR2Error::new_standalone("character already has a weapon of this type".to_owned()));
            }
        }
        self.builder.war_grear.weapons.push(weapon);
        return Ok(self);
    }

    pub fn with_armour(&mut self, armour: Armour) -> Result<&mut Self, TOR2Error> {
        // self.builder.war_grear.
        // TODO !!!
        return Ok(self);
    }

    pub fn with_shield(&mut self, shield: Shield) -> Result<&mut Self, TOR2Error> {

        return Ok(self);
    }

    pub fn with_useful_item(&mut self, useful_item: UsefulItem) -> Result<&mut Self, TOR2Error> {

        return Ok(self);
    }

    pub fn next_step(&self) -> Result<DwarfCharacterVirtuesAndRewardsBuilder, TOR2Error> {
        return Ok(DwarfCharacterVirtuesAndRewardsBuilder{
            virtue_set: false, reward_set: false, builder: self.builder.clone(),
        });
    }
}