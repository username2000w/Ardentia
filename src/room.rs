use core::fmt;
use std::io;

use rand::Rng;

use crate::entity::{Action, Monster, Player, Weapon, WeaponType};

pub struct Room {
    name: String,
    description: String,
    difficulty: Difficulty,
    monsters: Vec<Monster>,
    treasure: Option<Treasure>,
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

pub enum RoomResult {
    Sucess,
    Died,
    Ran,
}

impl Room {
    pub fn new(name: String, description: String, threat: i32) -> Room {
        let mut actual_threat = threat;
        let mut monsters: Vec<Monster> = Vec::new();

        while actual_threat > 0 {
            let num = rand::rng().random_range(1..(if actual_threat > 3 { actual_threat } else { actual_threat + 2 }));
            let monster = Monster::monster(num);

            match monster {
                Ok(m) => {
                    monsters.push(m);
                    actual_threat -= num
                },
                Err(_) => continue,
            }
        }

        let difficulty = match actual_threat {
            x if x < 0 => Difficulty::Medium,
            x if x < -3 => Difficulty::Hard,
            _ => Difficulty::Easy,
        };

        print!("Difficulty: {}\n", difficulty);

        Room {
            name,
            description,
            difficulty,
            monsters,
            treasure: Some(Treasure::new()),
        }
    }

    pub fn enter(&mut self, player: &mut Player) -> RoomResult {
        println!("You enter the {} {}.\n{}\n", self.name, self.difficulty, self.description);

        for monster in self.monsters.iter_mut() {
            println!("A level {} {} appears!\n", monster.level, monster.name);

            while monster.is_alive() {
                display_fight_screen(player, monster);

                if player.speed > monster.speed {
                    let action = player.select_action();

                    print!("\x1bc\x1b[1;1H"); // clear screen

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

                    let action = player.select_action();

                    print!("\x1bc\x1b[1;1H"); // clear screen

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
                println!("Your weapon attack: {} -> New weapon attack: {}", player.weapon.as_ref().unwrap_or(&Weapon::empty()).attack_value, weapon.attack_value);
                println!("Do you want to equip it? (y/n) (default: n)");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");

                match input.trim() {
                    "y" => player.weapon = Some(weapon),
                    _ => println!("You decide not to equip the weapon."),
                }
            }

            if let Some(health_potion) = treasure.health_potion {
                println!("You found a health potion!");
                println!("Your health: {} -> New health: {}", player.health, player.health + health_potion.heal_amount);
                player.health += health_potion.heal_amount;
            }
        }

        RoomResult::Sucess
    }
}

struct Treasure {
    weapon: Option<Weapon>,
    health_potion: Option<HealthPotion>,
}

impl Treasure {
    fn new() -> Treasure {
        Treasure {
            weapon: Some(Weapon::new(WeaponType::Sword)),
            // armor: None,
            health_potion: None,
        }
    }
}

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
        " Attack: {} (+{})\t\t Attack: {}",
        player.attack, player.weapon.as_ref().unwrap_or(&Weapon::empty()).attack_value, monster.attack
    );
    println!(
        " Defense: {}\t\t\t Defense: {}",
        player.defense, monster.defense
    );
    println!(" Speed: {}\t\t\t Speed: {}\n", player.speed, monster.speed);
}