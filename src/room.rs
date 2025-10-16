use core::fmt;
use std::fmt::{Debug, Display};

use rand::Rng;

use crate::{
    entity::{Weapon, WeaponType},
    monsters::{jungle::slime::Slime, Monster},
    zones::zone::Zone,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomType {
    Entrance, // Salle d'entrée - moins de monstres
    Normal,   // Salle normale - combats standards
    Elite,    // Salle d'élite - monstres plus forts
    Treasure, // Salle de trésor - peu de monstres mais bon loot
    Boss,     // Salle de boss - combat unique contre le boss
}

#[derive(Debug)]
pub struct Room {
    pub room_number: i32,
    pub zone: Zone,
    pub room_type: RoomType,
    pub monsters: Vec<Box<dyn Monster>>,
    pub treasures: Vec<Treasure>,
    pub is_cleared: bool,
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
    pub fn new(
        room_number: i32,
        zone: Zone,
        room_type: RoomType,
        monsters: Vec<Box<dyn Monster>>,
    ) -> Self {
        Self {
            room_number,
            zone,
            room_type,
            monsters,
            treasures: vec![Treasure::default()],
            is_cleared: false,
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

impl Default for Room {
    fn default() -> Self {
        Self {
            room_number: 1,
            zone: Zone::default(),
            room_type: RoomType::Entrance,
            monsters: vec![Box::new(Slime::new(1))],
            treasures: vec![Treasure::default()],
            is_cleared: true,
            current_monster: 0,
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
        Self::new(
            Some(Weapon::new(WeaponType::Sword)),
            Some(rand::rng().random_range(10..50)),
            None
        )
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

#[derive(Debug, Clone)]
pub struct HealthPotion {
    heal_amount: i32,
}

impl Display for HealthPotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A potion that heals {}hp", self.heal_amount)
    }
}
