use core::fmt;

use rand::Rng;

use crate::entity::{Monster, Weapon, WeaponType};

#[derive(Debug)]
pub struct Room {
	pub name: String,
	pub description: String,
	pub difficulty: Difficulty,
	pub monsters: Vec<Monster>,
	pub treasures: Vec<Treasure>,
	pub current_monster: usize,
}

pub trait TreasureUtils {
	fn treasure_len(&self) -> u16;
}

impl TreasureUtils for Vec<Treasure> {
	fn treasure_len(&self) -> u16 {
		let mut res = 0;

		for treasure in self {
			if treasure.weapon.is_some()
				|| treasure.gold.is_some()
				|| treasure.health_potion.is_some()
			{
				res += 1;
			}
		}
		res
	}
}

#[derive(Debug)]
pub enum Difficulty {
	Easy,
	Medium,
	Hard,
}

impl fmt::Display for Difficulty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Easy => write!(f, "Easy"),
			Self::Medium => write!(f, "Medium"),
			Self::Hard => write!(f, "Hard"),
		}
	}
}

#[derive(Debug)]
pub enum RoomResult {
	Sucess,
	Died,
	Ran,
}

impl Room {
	#[must_use]
	pub fn new(name: String, description: String, threat: i32) -> Self {
		let mut actual_threat = threat;
		let mut monsters: Vec<Monster> = Vec::new();

		while actual_threat > 0 {
			let num = rand::rng().random_range(
				1..(if actual_threat > 3 {
					actual_threat
				} else {
					actual_threat + 2
				}),
			);
			let monster = Monster::monster_based_on_threat(num);

			if let Ok(m) = monster {
				monsters.push(m);
				actual_threat -= num;
			}
		}

		let difficulty = match actual_threat {
			x if x < 0 => Difficulty::Medium,
			x if x < -3 => Difficulty::Hard,
			_ => Difficulty::Easy,
		};

		let treasures = vec![Treasure::new(
			Some(Weapon::new(WeaponType::Sword)),
			Some(rand::random_range(0..50)),
			None,
		)];

		Self {
			name,
			description,
			difficulty,
			monsters,
			treasures,
			current_monster: 0,
		}
	}

	pub const fn monster_slain(&mut self) -> bool {
		if self.monsters.len() > self.current_monster {
			self.current_monster += 1;
			false
		} else {
			true
		}
	}
}

#[derive(Debug, Clone)]
pub struct Treasure {
	pub weapon: Option<Weapon>,
	pub gold: Option<u32>,
	pub health_potion: Option<HealthPotion>,
}

impl Treasure {
	const fn new(
		weapon: Option<Weapon>,
		gold: Option<u32>,
		health_potion: Option<HealthPotion>,
	) -> Self {
		Self {
			weapon,
			gold,
			health_potion,
		}
	}
}

impl Default for Treasure {
	fn default() -> Self {
		Self {
			weapon: Some(Weapon::new(WeaponType::Sword)),
			gold: Some(rand::rng().random_range(10..50)),
			health_potion: None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct HealthPotion {
	_heal_amount: i32,
}
