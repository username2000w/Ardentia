use std::{fmt, io::{self, Error}};

use rand::Rng;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,

    pub weapon: Option<Weapon>,
}

pub enum Action {
    Attack,
    Run,
}

pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub attack_value: i32,
}

pub enum WeaponType {
    Sword,
    Dagger,
    Axe,
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeaponType::Sword => write!(f, "Sword"),
            WeaponType::Dagger => write!(f, "Dagger"),
            WeaponType::Axe => write!(f, "Axe"),
        }
    }
}

impl Weapon {
    pub fn new(weapon_type: WeaponType) -> Weapon {
        let prefix = vec!["Broken", "Rusty", "", "Sharp"];

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
        
        Weapon {
            name,
            weapon_type,
            attack_value,
        }
    }

    pub fn empty() -> Weapon {
        Weapon {
            name: String::from(""),
            weapon_type: WeaponType::Sword,
            attack_value: 0,
        }
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            health: 100,
            attack: 10,
            defense: 5,
            speed: 5,

            weapon: None,
        }
    }

    pub fn select_action(&self) -> Action {
        println!("What do you want to do?");
        println!("1. Attack");
        println!("2. Run");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.trim() {
            "1" => Action::Attack,
            "2" => Action::Run,
            _ => Action::Attack,
        }
    }

    pub fn attack(&self, target: &mut Monster) {
        let damage: i32;

        match self.weapon.as_ref() {
            Some(weapon) => {
                damage = (self.attack + weapon.attack_value) - target.defense;
            }
            None => {
                damage = self.attack - target.defense;
            }
        }

        let damage: i32 = match damage {
            x if x < 0 => 0,
            x => x as i32,
        };

        target.health -= damage as i32;

        println!(
            "{} attacks {} for {} damage",
            self.name, target.name, damage
        );
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
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

pub struct Monster {
    pub name: String,
    pub level: i32,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

pub enum MonsterType {
    Slime,
    Goblin,
    Ogre,
}

impl Monster {
    fn new(monster_type: MonsterType, level: i32) -> Monster {
        match monster_type {
            MonsterType::Slime => Monster {
                name: String::from("Slime"),
                level,
                health: level * 10,
                attack: level * 2,
                defense: level,
                speed: level,
            },
            MonsterType::Goblin => Monster {
                name: String::from("Goblin"),
                level,
                health: level * 20,
                attack: level * 3,
                defense: level,
                speed: level * 5,
            },
            MonsterType::Ogre => Monster {
                name: String::from("Ogre"),
                level,
                health: level * 30,
                attack: level * 4,
                defense: level * 2,
                speed: level,
            },
        }
    }

    pub fn monster(threat: i32) -> Result<Monster, Error> {
        match threat {
            1 => Ok(Monster::new(MonsterType::Slime, 1)),
            2 => Ok(Monster::new(MonsterType::Goblin, 1)),
            3 => Ok(Monster::new(MonsterType::Ogre, 1)),
            _ => Err(Error::new(io::ErrorKind::Other, "Monster not found")),
        }
    }
}

impl Monster {
    pub fn is_alive(&self) -> bool {
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