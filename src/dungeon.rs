use crate::{
	entity::Player,
	room::{Room, RoomResult},
};

#[derive(Debug, Default)]
pub struct Dungeon {
	rooms: Vec<Room>,
	pub level: i32,
}

impl Dungeon {
	#[must_use]
	pub fn new(level: i32, num_rooms: i32) -> Self {
		let threat = 2 + ((level - 1) * 7);

		let mut rooms = Vec::new();
		for _ in 0..num_rooms {
			rooms.push(Room::new(
				String::from("Room"),
				String::from("A room."),
				threat,
			));
		}

		Self { rooms, level }
	}

	pub fn enter(&mut self, player: &mut Player) {
		println!("You enter the level {} Dongeon.", self.level);
		for room in &mut self.rooms {
			match room.enter(player) {
				RoomResult::Sucess => println!("You killed every monster of this room..."),
				RoomResult::Died => {
					println!("You died!");
					break;
				}
				RoomResult::Ran => println!("You ran away!"),
			}
		}
	}
}
