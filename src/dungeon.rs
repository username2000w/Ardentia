use crate::{entity::Player, room::{Room, RoomResult}};

pub struct Dungeon {
    rooms: Vec<Room>,
    level: i32,
}

impl Dungeon {
    pub fn new(level: i32, num_rooms: i32) -> Dungeon {
        let threat = 2 + ((level - 1) * 7);

        let mut rooms = Vec::new();
        for _ in 0..num_rooms {
            rooms.push(Room::new(String::from("Room"), String::from("A room."), threat));
        }

        Dungeon {
            rooms,
            level,
        }
    }

    pub fn enter(&mut self, player: &mut Player) {
        println!("You enter the level {} Dongeon.", self.level);
        for room in self.rooms.iter_mut() {
            match room.enter(player) {
                RoomResult::Sucess => println!("You win!"),
                RoomResult::Died => println!("You died!"),
                RoomResult::Ran => println!("You ran away!"),
            }
        }
    }
}