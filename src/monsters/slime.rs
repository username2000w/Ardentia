use crate::monsters::{Monster, MonsterStats};

#[derive(Debug)]
pub struct Slime {
    stats: MonsterStats,
}

impl Slime {
    #[must_use]
    pub fn new(level: i32) -> Self {
        Self {
            stats: MonsterStats {
                name: String::from("Slime"),
                level,
                health: 4 + level,
                attack: level,
                defence: 0,
                speed: level,
            },
        }
    }
}

impl Monster for Slime {
    fn is_alive(&self) -> bool {
        self.stats.health > 0
    }

    fn attack(&self, target: &mut crate::entity::Player) {
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
