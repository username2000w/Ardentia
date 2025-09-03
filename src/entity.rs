use std::{
	fmt,
	io::{self, Error},
};

use rand::Rng;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub health: i32,
	pub attack: i32,
	pub defense: i32,
	pub speed: i32,

	pub weapon: Option<Weapon>,
}

#[derive(Debug)]
pub enum Action {
	Attack,
	Run,
}

#[derive(Debug)]
pub struct Weapon {
	pub name: String,
	pub weapon_type: WeaponType,
	pub attack_value: i32,
	pub rarity: Rarity,
}

#[derive(Debug)]
pub enum WeaponType {
	Sword,
	Dagger,
	Axe,
}

#[derive(Debug)]
pub enum Rarity {
	Common,
	Rare,
	Epic,
	Legendary,
	Mythical,
}

impl fmt::Display for WeaponType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Sword => write!(f, "Sword"),
			Self::Dagger => write!(f, "Dagger"),
			Self::Axe => write!(f, "Axe"),
		}
	}
}

impl fmt::Display for Rarity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Common => write!(f, "common"),
			Self::Rare => write!(f, "rare"),
			Self::Epic => write!(f, "epic"),
			Self::Legendary => write!(f, "legendary"),
			Self::Mythical => write!(f, "mythical"),
		}
	}
}

impl Weapon {
	#[must_use]
	pub fn new(weapon_type: WeaponType) -> Self {
		let prefix = ["Broken", "Rusty", "", "Sharp"];

		let weapon_prefix = prefix[rand::rng().random_range(0..prefix.len())];

		let name = format!("{} {}", weapon_prefix, WeaponType::Sword);

		let mut attack_value = match weapon_type {
			WeaponType::Sword => 10,
			WeaponType::Dagger => 5,
			WeaponType::Axe => 15,
		};

		attack_value = match weapon_prefix {
			"Broken" => attack_value / 2,
			"Rusty" => attack_value - 2,
			"Sharp" => attack_value + 2,
			_ => attack_value,
		};

		let rarity = Rarity::Common;

		Self {
			name,
			weapon_type,
			attack_value,
			rarity,
		}
	}

	#[must_use]
	pub const fn empty() -> Self {
		Self {
			name: String::new(),
			weapon_type: WeaponType::Sword,
			attack_value: 0,
			rarity: Rarity::Common,
		}
	}
}

impl Player {
	#[must_use]
	pub const fn new(name: String) -> Self {
		Self {
			name,
			health: 100,
			attack: 10,
			defense: 5,
			speed: 5,

			weapon: None,
		}
	}

	#[must_use]
	pub fn select_action(&self) -> Action {
		println!("What do you want to do?");
		println!("1. Attack");
		println!("2. Run");
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("error: unable to read user input");

		match input.trim() {
			"2" => Action::Run,
			_ => Action::Attack,
		}
	}

	pub fn attack(&self, target: &mut Monster) {
		let damage = match self.weapon.as_ref() {
			Some(weapon) => (self.attack + weapon.attack_value) - target.defense,
			None => self.attack - target.defense,
		};

		let damage: i32 = match damage {
			x if x < 0 => 0,
			x => x,
		};

		target.health -= damage;

		println!(
			"{} attacks {} for {} damage",
			self.name, target.name, damage
		);
	}

	#[must_use]
	pub const fn is_dead(&self) -> bool {
		self.health <= 0
	}

	pub fn equip(&mut self, weapon: Weapon) {
		self.weapon = Some(weapon);
	}
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"===== {} =====\nHealth: {}\nAttack: {}\nDefense: {}\nSpeed: {}\n",
			self.name, self.health, self.attack, self.defense, self.speed
		)
	}
}

#[derive(Debug)]
pub struct Monster {
	pub name: String,
	pub level: i32,
	pub health: i32,
	pub attack: i32,
	pub defense: i32,
	pub speed: i32,
}
#[derive(Debug)]
pub enum MonsterType {
	Slime,
	Goblin,
	Ogre,
}

impl Monster {
	#[must_use]
	fn new(monster_type: &MonsterType, level: i32) -> Self {
		match monster_type {
			MonsterType::Slime => Self {
				name: String::from("Slime"),
				level,
				health: level * 10,
				attack: level * 2,
				defense: level,
				speed: level,
			},
			MonsterType::Goblin => Self {
				name: String::from("Goblin"),
				level,
				health: level * 20,
				attack: level * 3,
				defense: level,
				speed: level * 5,
			},
			MonsterType::Ogre => Self {
				name: String::from("Ogre"),
				level,
				health: level * 30,
				attack: level * 4,
				defense: level * 2,
				speed: level,
			},
		}
	}

	pub fn monster_based_on_threat(threat: i32) -> Result<Self, Error> {
		match threat {
			1 => Ok(Self::new(&MonsterType::Slime, 1)),
			2 => Ok(Self::new(&MonsterType::Goblin, 1)),
			3 => Ok(Self::new(&MonsterType::Ogre, 1)),
			_ => Err(Error::other("Monster not found")),
		}
	}
}

impl Monster {
	#[must_use]
	pub const fn is_alive(&self) -> bool {
		self.health > 0
	}

	pub fn attack(&self, target: &mut Player) {
		let damage = self.attack - target.defense;

		let damage = match damage {
			x if x < 0 => 1,
			x => x,
		};

		target.health -= damage;

		println!(
			"{} attacks {} for {} damage",
			self.name, target.name, damage
		);
	}
}

impl fmt::Display for Monster {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"===== {} =====\nHealth: {}\nAttack: {}\nDefense: {}\nSpeed: {}\n",
			self.name, self.health, self.attack, self.defense, self.speed
		)
	}
}
