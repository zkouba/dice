use crate::skills::skills::Skill;

pub struct UsefulItem<'a> {
    pub name: String,
    pub helps_with_skill: &'a Skill<'a>,
}
