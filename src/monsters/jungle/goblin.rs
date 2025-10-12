use crate::{
    entity::Player,
    monsters::{Monster, MonsterStats},
};

#[derive(Debug)]
pub struct Goblin {
    stats: MonsterStats,
}

impl Goblin {
    #[must_use]
    pub fn new(level: i32) -> Self {
        Self {
            stats: MonsterStats {
                name: String::from("Goblin"),
                level,
                health: 8 + level * 2,
                attack: 2 + level,
                defence: level / 2,
                speed: 3 + level * 2,
            },
        }
    }
}

impl Monster for Goblin {
    fn is_alive(&self) -> bool {
        self.stats.health > 0
    }

    fn attack(&self, target: &mut Player) {
        let damage = self.stats.attack - target.defence;

        let damage = match damage {
            x if x < 0 => 1,
            x => x,
        };

        target.health -= damage;
    }

    fn get_stats(&self) -> MonsterStats {
        self.stats.clone()
    }

    fn take_damage(&mut self, damage: i32) {
        self.stats.health -= damage;
    }
}
