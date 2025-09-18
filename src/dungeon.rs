use crate::room::Room;

#[derive(Debug, Default)]
pub struct Dungeon {
	pub rooms: Vec<Room>,
	pub level: i32,
	pub current_room: usize,
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

		Self {
			rooms,
			level,
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

	pub fn get_current_room_immutable(&self) -> Room {
		self.rooms[self.current_room].clone()
	}
}
