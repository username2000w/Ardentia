use crate::{
    room::Room,
    zones::{generator::RoomGenerator, zone::Zone},
};

#[derive(Default, Debug)]
pub struct Dungeon {
    pub current_zone: Zone,
    pub current_room_number: i32,
    pub room_generator: RoomGenerator,
    pub is_active: bool,
    pub current_room: Room,
}

const MAX_MONSTER_NUMBER: i32 = 10;

impl Dungeon {
    #[must_use]
    pub fn new(zone: Zone) -> Self {
        Self {
            current_zone: zone,
            current_room_number: 1,
            room_generator: RoomGenerator::new(),
            current_room: Room::default(),
            is_active: true,
        }
    }

    pub fn start(&mut self) {
        self.current_room = self
            .room_generator
            .generate_room(&self.current_zone, self.current_room_number);
    }

    #[must_use]
    pub fn generate_current_room(&self) -> Room {
        self.room_generator
            .generate_room(&self.current_zone, self.current_room_number)
    }

    pub fn next_room(&mut self) {
        self.current_room_number += 1;

        self.current_room = self
            .room_generator
            .generate_room(&self.current_zone, self.current_room_number);
    }

    pub const fn complete_zone(&mut self) {
        self.is_active = false;
    }

    pub const fn handle_player_death(&mut self) {
        self.is_active = false;
    }

    pub const fn is_there_rooms_left(&mut self) -> bool {
        self.current_room_number < MAX_MONSTER_NUMBER
    }

    pub const fn get_current_room_mutable(&mut self) -> &mut Room {
        &mut self.current_room
    }

    #[must_use]
    pub const fn get_current_room_immutable(&self) -> &Room {
        &self.current_room
    }
}
