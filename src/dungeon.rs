use crate::room::Room;

#[derive(Debug, Default)]
pub struct Dungeon {
	pub rooms: Vec<Room>,
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
}
