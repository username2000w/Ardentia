use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ZoneDifficulty {
    #[default]
    Normal,
}

impl Display for ZoneDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZoneDifficulty::Normal => write!(f, "Normal"),
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
            ZoneType::Jungle => write!(f, "Jungle"),
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
                "Jungle Profonde".to_string(),
                "Cœur dangereux de la jungle".to_string(),
                1,
                vec![
                    "Slime".to_string(),
                    "Goblin".to_string(),
                    "Ogre".to_string(),
                ],
                "Araignée Géante".to_string(),
            ),
        }
    }

    pub fn get_monster_level_range(&self) -> (i32, i32) {
        match self.difficulty {
            ZoneDifficulty::Normal => (1, 5),
        }
    }

    pub fn get_available_zones() -> Vec<Zone> {
        vec![Zone::new(ZoneType::Jungle, ZoneDifficulty::Normal)]
    }
}
