use crate::entity::Player;

pub trait Monster {
    fn is_alive(&self) -> bool;
    fn attack(&self, target: &mut Player);
    fn get_stats(&self) -> MonsterStats;
    fn take_damage(&mut self, damage: i32);
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

pub mod slime;
