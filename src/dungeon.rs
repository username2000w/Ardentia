use crate::room::Room;

#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct Dungeon {
    pub rooms: Vec<Room>,
    pub level: i32,
    pub current_room: usize,
}

impl Dungeon {
    #[must_use]
    pub fn new(dungeon_level: i32, num_rooms: i32) -> Self {
        let mut rooms = Vec::new();

        for i in 0..num_rooms {
            let room_level = if i == num_rooms - 1 {
                dungeon_level + 1
            } else {
                dungeon_level
            };

            rooms.push(Room::new(
                format!("Room {}", i + 1),
                format!("Level {room_level}"),
                room_level,
            ));
        }

        Self {
            rooms,
            level: dungeon_level,
            current_room: 0,
        }
    }

    pub const fn is_there_rooms_left(&mut self) -> bool {
        self.rooms.len() != self.current_room + 1
    }

    pub const fn next_room(&mut self) {
        if self.rooms.len() - 1 > self.current_room {
            self.current_room += 1;
        }
    }

    pub fn get_current_room_mutable(&mut self) -> &mut Room {
        &mut self.rooms[self.current_room]
    }

    #[must_use]
    pub fn get_current_room_immutable(&self) -> &Room {
        &self.rooms[self.current_room]
    }
}
