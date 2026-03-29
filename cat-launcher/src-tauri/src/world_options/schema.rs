use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
  Boolean,
  Integer,
  Float,
  String,
  Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionSchema {
  pub name: String,
  pub option_type: OptionType,
  pub min: Option<f64>,
  pub max: Option<f64>,
}

pub fn get_known_options() -> Vec<OptionSchema> {
  vec![
    OptionSchema {
      name: "BLACK_ROAD".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "ETERNAL_SEASON".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "CONSTRUCTION_SCALING".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(1000.0),
    },
    OptionSchema {
      name: "SEASON_LENGTH".to_string(),
      option_type: OptionType::Integer,
      min: Some(14.0),
      max: Some(127.0),
    },
    OptionSchema {
      name: "WORLD_END".to_string(),
      option_type: OptionType::Enum(vec![
        "reset".to_string(),
        "delete".to_string(),
        "query".to_string(),
        "keep".to_string(),
      ]),
      min: None,
      max: None,
    },
    OptionSchema {
      name: "ITEM_SPAWNRATE".to_string(),
      option_type: OptionType::Float,
      min: Some(0.01),
      max: Some(10.0),
    },
    OptionSchema {
      name: "SPAWN_DENSITY".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(50.0),
    },
    OptionSchema {
      name: "EVOLUTION_INVERSE_MULTIPLIER".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(100.0),
    },
    OptionSchema {
      name: "ETERNAL_TIME_OF_DAY".to_string(),
      option_type: OptionType::Enum(vec![
        "normal".to_string(),
        "day".to_string(),
        "night".to_string(),
      ]),
      min: None,
      max: None,
    },
    OptionSchema {
      name: "NPC_SPAWNTIME".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(100.0),
    },
    OptionSchema {
      name: "MONSTER_RESILIENCE".to_string(),
      option_type: OptionType::Integer,
      min: Some(1.0),
      max: Some(1000.0),
    },
    OptionSchema {
      name: "META_PROGRESS".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "MONSTER_SPEED".to_string(),
      option_type: OptionType::Integer,
      min: Some(1.0),
      max: Some(1000.0),
    },
    OptionSchema {
      name: "INITIAL_DAY".to_string(),
      option_type: OptionType::Integer,
      min: Some(-1.0),
      max: Some(999.0),
    },
    OptionSchema {
      name: "VEHICLE_SPAWNRATE".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(5.0),
    },
    OptionSchema {
      name: "CARRION_SPAWNRATE".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(10.0),
    },
    OptionSchema {
      name: "SPECIALS_DENSITY".to_string(),
      option_type: OptionType::Float,
      min: Some(0.01),
      max: Some(10.0),
    },
    OptionSchema {
      name: "SPAWN_DELAY".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(9999.0),
    },
    OptionSchema {
      name: "SPAWN_ANIMAL_DENSITY".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(50.0),
    },
    OptionSchema {
      name: "CITY_SIZE".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(16.0),
    },
    OptionSchema {
      name: "MONSTER_UPGRADE_FACTOR".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(100.0),
    },
    OptionSchema {
      name: "STARTING_NPC".to_string(),
      option_type: OptionType::Enum(vec![
        "never".to_string(),
        "always".to_string(),
        "scenario".to_string(),
      ]),
      min: None,
      max: None,
    },
    OptionSchema {
      name: "SPECIALS_SPACING".to_string(),
      option_type: OptionType::Integer,
      min: Some(-1.0),
      max: Some(72.0),
    },
    OptionSchema {
      name: "CITY_SPACING".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(8.0),
    },
    OptionSchema {
      name: "WANDER_SPAWNS".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "VEHICLE_DAMAGE".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(10.0),
    },
    OptionSchema {
      name: "CRAFTING_SPEED_MULT".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(1000.0),
    },
    OptionSchema {
      name: "GROWTH_SCALING".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(1000.0),
    },
    OptionSchema {
      name: "DEFAULT_REGION".to_string(),
      option_type: OptionType::Enum(vec!["default".to_string()]),
      min: None,
      max: None,
    },
    OptionSchema {
      name: "RANDOM_NPC".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "INITIAL_TIME".to_string(),
      option_type: OptionType::Integer,
      min: Some(0.0),
      max: Some(23.0),
    },
    OptionSchema {
      name: "VEHICLE_LOCKS".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "RAD_MUTATION".to_string(),
      option_type: OptionType::Boolean,
      min: None,
      max: None,
    },
    OptionSchema {
      name: "NPC_DENSITY".to_string(),
      option_type: OptionType::Float,
      min: Some(0.0),
      max: Some(100.0),
    },
    OptionSchema {
      name: "CHARACTER_POINT_POOLS".to_string(),
      option_type: OptionType::Enum(vec![
        "any".to_string(),
        "multi_pool".to_string(),
        "no_freeform".to_string(),
      ]),
      min: None,
      max: None,
    },
    OptionSchema {
      name: "RESTOCK_DELAY_MULT".to_string(),
      option_type: OptionType::Float,
      min: Some(0.01),
      max: Some(10.0),
    },
  ]
}
