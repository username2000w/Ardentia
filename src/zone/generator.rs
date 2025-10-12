use rand::Rng;

use crate::{
    monsters::balancer::{MonsterBalancer, RoomData},
    room::{Room, RoomType},
    zone::zone::Zone,
};

#[derive(Default, Debug)]
pub struct RoomGenerator {
    balancer: MonsterBalancer,
}

impl RoomGenerator {
    pub fn new() -> Self {
        Self {
            balancer: MonsterBalancer::new(),
        }
    }

    pub fn generate_room(&self, zone: &Zone, room_number: i32) -> Room {
        let room_type = self.determine_room_type(room_number);
        let room_size = self.determine_room_size(&room_type);

        let difficulty = RoomData {
            zone: zone.clone(),
            room_type: room_type.clone(),
            room_number,
        };

        let monsters = self
            .balancer
            .generate_monsters_for_room(&difficulty, room_size);

        Room::new(room_number, zone.clone(), room_type, monsters)
    }

    fn determine_room_type(&self, room_number: i32) -> RoomType {
        let mut rng = rand::rng();
        let roll: f64 = rng.random();

        // Boss room à la salle 10
        if room_number == 10 {
            return RoomType::Boss;
        }

        // Salle de trésor occasionnelle
        if room_number % 5 == 0 && roll < 0.3 {
            return RoomType::Treasure;
        }

        match room_number {
            1 => RoomType::Entrance,
            2..=4 if roll < 0.8 => RoomType::Normal,
            2..=4 => RoomType::Elite,
            5..=9 if roll < 0.7 => RoomType::Normal,
            5..=9 if roll < 0.9 => RoomType::Elite,
            5..=9 => RoomType::Treasure,
            _ if roll < 0.6 => RoomType::Normal,
            _ if roll < 0.85 => RoomType::Elite,
            _ => RoomType::Treasure,
        }
    }

    fn determine_room_size(&self, room_type: &RoomType) -> usize {
        let mut rng = rand::rng();
        match room_type {
            RoomType::Entrance => rng.random_range(1..=2),
            RoomType::Normal => rng.random_range(2..=4),
            RoomType::Elite => rng.random_range(1..=3),
            RoomType::Treasure => 1,
            RoomType::Boss => 1, // Le boss est seul
        }
    }
}
