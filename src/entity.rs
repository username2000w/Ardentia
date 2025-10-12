use std::{
    fmt::{self, Display},
    io::{self},
};

use rand::Rng;

use crate::monsters::Monster;

#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
    pub max_health: i32,
    pub health: i32,
    pub attack: i32,
    pub defence: i32,
    pub speed: i32,

    pub weapon: Option<Weapon>,
}

#[derive(Debug)]
pub enum Action {
    Attack,
    Run,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub attack_value: i32,
    pub rarity: Rarity,
}

#[derive(Debug, Clone)]
pub enum WeaponType {
    Sword,
    Dagger,
    Axe,
}

#[derive(Debug, Clone)]
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

impl Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A {} {}", self.rarity, self.weapon_type)
    }
}

impl Player {
    #[must_use]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            max_health: 10,
            health: 10,
            attack: 1,
            defence: 0,
            speed: 1,

            weapon: None,
        }
    }

    #[allow(clippy::missing_panics_doc)]
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

    pub fn attack(&self, target: &mut dyn Monster) {
        let target_stats = target.get_stats();

        let damage = match self.weapon.as_ref() {
            Some(weapon) => (self.attack + weapon.attack_value) - target_stats.defence,
            None => self.attack - target_stats.defence,
        };

        let damage: i32 = match damage {
            x if x < 0 => 0,
            x => x,
        };

        target.take_damage(damage);
    }

    #[must_use]
    pub fn get_attack(&self) -> i32 {
        let mut attack_value = self.attack;

        if let Some(x) = self.weapon.clone() {
            attack_value += x.attack_value;
        }

        attack_value
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
            self.name, self.health, self.attack, self.defence, self.speed
        )
    }
}

// pub fn monster_based_on_threat(threat: i32) -> Result<Self, Error> {
// 	match threat {
// 		1 => Ok(Self::new(&MonsterType::Slime, 1)),
// 		2 => Ok(Self::new(&MonsterType::Goblin, 1)),
// 		3 => Ok(Self::new(&MonsterType::Ogre, 1)),
// 		_ => Err(Error::other("Monster not found")),
// 	}
// }
