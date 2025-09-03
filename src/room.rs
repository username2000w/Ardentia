use core::fmt;
use std::io;

use rand::Rng;

use crate::entity::{Action, Monster, Player, Weapon, WeaponType};

#[derive(Debug)]
pub struct Room {
	name: String,
	description: String,
	difficulty: Difficulty,
	monsters: Vec<Monster>,
	treasure: Option<Treasure>,
}

#[derive(Debug)]
enum Difficulty {
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
	pub fn display(&self) {
		println!("============================");
		println!("📍 Room : {}", self.name);
		println!("============================\n");
		println!("{}", self.description);
		println!("\n🔹 Difficulty : {}\n", self.difficulty);

		println!("👹 Monsters :");
		if self.monsters.is_empty() {
			println!("  - No monsters here.");
		} else {
			for monster in &self.monsters {
				println!("  - {} (Level {})", monster.name, monster.level);
			}
		}

		println!("\n💰 Treasure :");
		match &self.treasure {
			Some(t) => {
				match &t.weapon {
					Some(w) => println!("  - A {} weapon", w.rarity),
					None => println!(""),
				}
				println!("  - Treasure: {} golds", t.gold);
			}
			None => println!("  - Aucun trésor ici."),
		}

		println!("\n============================\n");
	}

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

		Self {
			name,
			description,
			difficulty,
			monsters,
			treasure: Some(Treasure::new()),
		}
	}

	pub fn enter(&mut self, player: &mut Player) -> RoomResult {
		println!("You enter a room.\n");

		self.display();

		println!("Continue...");

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("error: unable to read user input");

		print!("\x1bc\x1b[1;1H"); // clear screen

		for monster in self.monsters.iter_mut() {
			println!("A level {} {} appears!\n", monster.level, monster.name);

			while monster.is_alive() {
				display_fight_screen(player, monster);

				let action = player.select_action();

				print!("\x1bc\x1b[1;1H"); // clear screen
				if player.speed > monster.speed {
					match action {
						Action::Attack => {
							player.attack(monster);
						}
						Action::Run => return RoomResult::Ran,
					}

					if monster.is_alive() {
						monster.attack(player);
					}
				} else {
					monster.attack(player);

					match action {
						Action::Attack => {
							player.attack(monster);
						}
						Action::Run => return RoomResult::Ran,
					}
				}

				if player.is_dead() {
					return RoomResult::Died;
				}
			}

			println!("You killed the {}!", monster.name);
		}

		if let Some(treasure) = self.treasure.take() {
			if let Some(weapon) = treasure.weapon {
				println!("You found a weapon!"); //, treasure.weapon.as_ref().unwrap().name);
				println!(
					"Your attack: {} -> New attack: {}",
					player.attack
						+ player
							.weapon
							.as_ref()
							.unwrap_or(&Weapon::empty())
							.attack_value,
					player.attack + weapon.attack_value
				);
				println!("Do you want to equip it? (y/N)");

				let mut input = String::new();
				io::stdin()
					.read_line(&mut input)
					.expect("error: unable to read user input");

				match input.trim() {
					"y" => player.equip(weapon),
					_ => println!("You decide not to equip the weapon."),
				}
			}

			if let Some(health_potion) = treasure.health_potion {
				println!("You found a health potion!");
				println!(
					"Your health: {} -> New health: {}",
					player.health,
					player.health + health_potion.heal_amount
				);
				player.health += health_potion.heal_amount;
			}
		}

		RoomResult::Sucess
	}
}

#[derive(Debug)]
struct Treasure {
	gold: i32,
	weapon: Option<Weapon>,
	health_potion: Option<HealthPotion>,
}

impl Treasure {
	fn new() -> Self {
		Self {
			gold: rand::rng().random_range(10..50),
			weapon: Some(Weapon::new(WeaponType::Sword)),
			// armor: None,
			health_potion: None,
		}
	}
}

#[derive(Debug)]
struct HealthPotion {
	heal_amount: i32,
}

fn display_fight_screen(player: &Player, monster: &Monster) {
	println!(" ==============================================");
	println!("  *--*-              BATTLE              *-*--* ");
	println!(" ==============================================\n");

	println!(
		"===== {} =====\t\t\t===== {} =====",
		player.name, monster.name
	);
	println!(
		" Health: {}\t\t\t Health: {}",
		player.health, monster.health
	);
	println!(
		" Attack: {}\t\t\t Attack: {}",
		player.attack
			+ player
				.weapon
				.as_ref()
				.unwrap_or(&Weapon::empty())
				.attack_value,
		monster.attack
	);
	println!(
		" Defense: {}\t\t\t Defense: {}",
		player.defense, monster.defense
	);
	println!(" Speed: {}\t\t\t Speed: {}\n", player.speed, monster.speed);
}
