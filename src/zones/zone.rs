use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum ZoneDifficulty {
    #[default]
    Normal,
}

impl Display for ZoneDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Normal => write!(f, "Normal"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum ZoneType {
    #[default]
    Jungle,
}

impl Display for ZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Jungle => write!(f, "Jungle"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub difficulty: ZoneDifficulty,
    pub name: String,
    pub description: String,
    pub recommended_level: i32,
    pub unique_monsters: Vec<String>,
    pub boss_name: String,
}

impl Zone {
    #[must_use]
    pub fn new(zone_type: ZoneType, difficulty: ZoneDifficulty) -> Self {
        let (name, description, recommended_level, unique_monsters, boss_name) = match zone_type {
            ZoneType::Jungle => Self::jungle_data(&difficulty),
        };

        Self {
            zone_type,
            difficulty,
            name,
            description,
            recommended_level,
            unique_monsters,
            boss_name,
        }
    }

    fn jungle_data(difficulty: &ZoneDifficulty) -> (String, String, i32, Vec<String>, String) {
        match difficulty {
            ZoneDifficulty::Normal => (
                "Jungle".to_string(),
                "Dangerous heart of the jungle".to_string(),
                1,
                vec![
                    "Slime".to_string(),
                    "Goblin".to_string(),
                    "Ogre".to_string(),
                ],
                "Giant Spider".to_string(),
            ),
        }
    }

    #[must_use]
    pub const fn get_monster_level_range(&self) -> (i32, i32) {
        match self.difficulty {
            ZoneDifficulty::Normal => (1, 4),
        }
    }

    #[must_use]
    pub fn get_available_zones() -> Vec<Self> {
        vec![Self::new(ZoneType::Jungle, ZoneDifficulty::Normal)]
    }
}
