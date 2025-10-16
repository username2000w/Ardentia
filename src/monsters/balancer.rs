use rand::seq::IndexedRandom;

use crate::{
    monsters::{create_monster, Monster},
    room::RoomType,
    zones::zone::{Zone},
};

#[derive(Debug, Clone)]
pub struct RoomData {
    pub zone: Zone,
    pub room_type: RoomType,
    pub room_number: i32,
}

#[derive(Default, Debug)]
pub struct MonsterBalancer;

impl MonsterBalancer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn generate_monsters_for_room(
        &self,
        room_data: &RoomData,
        room_size: usize,
    ) -> Vec<Box<dyn Monster>> {
        let mut monsters = Vec::new();

        for _ in 0..room_size {
            if let Some(monster) = generate_monster_for_room(room_data) {
                monsters.push(monster);
            }
        }

        monsters
    }
}

fn generate_monster_for_room(room_data: &RoomData) -> Option<Box<dyn Monster>> {
    let mut rng = rand::rng();

    // Salle de boss : génère le boss spécifique à la zone
    // if room_data.room_type == RoomType::Boss {
    //     return self.generate_boss(&room_data.zone);
    // }

    // Choix entre monstres génériques et spécifiques à la zone
    let monster_pool = get_monster_pool(room_data);
    let monster_name = monster_pool.choose(&mut rng)?;

    let monster_level = calculate_monster_level(room_data);
    create_monster(monster_name, monster_level)
}

// fn generate_boss(zone: &Zone) -> Option<Box<dyn Monster>> {
//     let boss_level = calculate_boss_level(zone);

//     match zone.zone_type {
//         ZoneType::Jungle => {
//             Some(Box::new(JungleBoss::new(boss_level, &zone.boss_name)))
//         }
//     }
// }

fn get_monster_pool(room_data: &RoomData) -> Vec<String> {
    let mut pool = Vec::new();

    pool.extend_from_slice(&[
        "Slime".to_string(),
        "Goblin".to_string(),
        "Ogre".to_string(),
    ]);

    for monster_name in &room_data.zone.unique_monsters {
        pool.push(monster_name.clone());
    }
    
    pool = match room_data.room_number {
        1 => vec!["Slime".to_string()],
        2..3 => vec!["Slime".to_string(), "Goblin".to_string()],
        _ => pool
    };
    
    pool
}

fn calculate_monster_level(room_data: &RoomData) -> i32 {
    let (min_level, max_level) = room_data.zone.get_monster_level_range();
    let base_level = min_level.midpoint(max_level);

    let room_bonus = match room_data.room_type {
        RoomType::Entrance => -1,
        RoomType::Normal => 0,
        RoomType::Elite => 2,
        RoomType::Treasure => 1,
        RoomType::Boss => 5,
    };

    (base_level + room_bonus).clamp(min_level, max_level)
}

// fn calculate_boss_level(zone: &Zone) -> i32 {
//     let (_, max_level) = zone.get_monster_level_range();
//     max_level + 2
// }