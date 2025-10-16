use std::fmt::Debug;

use crate::{
    entity::Player,
    monsters::jungle::{goblin::Goblin, ogre::Ogre, slime::Slime},
};

pub mod balancer;
pub mod jungle;

pub trait Monster {
    fn is_alive(&self) -> bool;
    fn attack(&self, target: &mut Player);
    fn get_stats(&self) -> MonsterStats;
    fn take_damage(&mut self, damage: i32);
}

impl Debug for dyn Monster {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Monster{{{}}}", self.is_alive())
    }
}

#[derive(Debug, Clone)]
pub struct MonsterStats {
    pub name: String,
    pub level: i32,
    pub health: i32,
    pub attack: i32,
    pub defence: i32,
    pub speed: i32,
}

#[must_use]
pub fn create_monster(name: &str, level: i32) -> Option<Box<dyn Monster>> {
    match name {
        "Slime" => Some(Box::new(Slime::new(level))),
        "Goblin" => Some(Box::new(Goblin::new(level))),
        "Ogre" => Some(Box::new(Ogre::new(level))),

        _ => None,
    }
}