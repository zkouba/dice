use crate::skills::skills::Skill;

#[derive(Clone)]
pub struct UsefulItem<'a> {
    pub name: String,
    pub helps_with_skill: &'a Skill,
}
