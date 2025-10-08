use core::fmt;
use std::fmt::Display;

use rand::Rng;

use crate::entity::{Monster, MonsterType, Weapon, WeaponType};

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
pub enum Difficulty {
	#[default]
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
	pub fn new(name: String, description: String, room_level: i32) -> Self {
		let mut monsters: Vec<Monster> = Vec::new();

		let num_monsters = match room_level {
			1 => rand::random_range(1..=2),     // Niveau 1 : 1-2 monstres
			2 => rand::random_range(1..=3),     // Niveau 2 : 1-3 monstres
			3 => rand::random_range(2..=3),     // Niveau 3 : 2-3 monstres
			4 => rand::random_range(2..=4),     // Niveau 4 : 2-4 monstres
			_ => rand::random_range(3..=5),     // Niveau 5+ : 3-5 monstres
		};
		
		let available_monsters = match room_level {
			1 => vec![MonsterType::Slime],
			2 => vec![MonsterType::Slime, MonsterType::Goblin],
			_ => vec![MonsterType::Slime, MonsterType::Goblin, MonsterType::Ogre],
		};
		
		for _ in 0..num_monsters {
			let monster_type = available_monsters[rand::random_range(0..available_monsters.len())];
			
			let base_level = room_level as u32;
			let level_variation = match room_level {
				1 => 0,    							// Niveau 1 : pas de variation
				2..=3 => rand::random_range(0..=1),	// Niveau 2-3 : ±1 niveau
				_ => rand::random_range(0..=2),		// Niveau 4+ : ±2 niveaux
			};
			
			let monster_level = (base_level as i32 + level_variation - 1).max(1);
			
			monsters.push(Monster::new(&monster_type, monster_level));
		}

		let difficulty = Difficulty::Easy;

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

	pub const fn monster_slain(&mut self) {
		self.current_monster += 1;
	}

	pub const fn is_empty(&mut self) -> bool {
		self.monsters.len() == self.current_monster
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

pub trait WeaponUtils {
	fn get_weapon(&self) -> Option<Weapon>;
	fn contains_weapon(&self) -> bool;
}

impl WeaponUtils for Vec<Treasure> {
	fn get_weapon(&self) -> Option<Weapon> {
		for treasure in self {
			if treasure.weapon.is_some() {
				return treasure.weapon.clone();
			}
		}
		None
	}

	fn contains_weapon(&self) -> bool {
		for treasure in self {
			if treasure.weapon.is_some() {
				return true;
			}
		}
		false
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
	heal_amount: i32,
}

impl Display for HealthPotion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "A potion that heals {}hp", self.heal_amount)
	}
}
