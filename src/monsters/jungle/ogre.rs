use crate::{
    entity::Player,
    monsters::{Monster, MonsterStats},
};

#[derive(Debug)]
pub struct Ogre {
    stats: MonsterStats,
}

impl Ogre {
    #[must_use]
    pub fn new(level: i32) -> Self {
        Self {
            stats: MonsterStats {
                name: String::from("Ogre"),
                level,
                health: 15 + level * 3,
                attack: 3 + level * 2,
                defence: 1 + level,
                speed: level / 2,
            },
        }
    }
}

impl Monster for Ogre {
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
