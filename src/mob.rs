use serde::Deserialize;
use std::{fs, path::Path};
use bracket_lib::prelude::*;
use crate::{entity::Entity, map::Map};

#[derive(Debug, Deserialize, Clone)]
pub struct MobData {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub description: String,
    pub visuals: Visuals,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Visuals {
    pub glyph: char,
    pub color: (u8, u8, u8),  // Use a tuple for color
}

pub fn load_mob_data(dir_path: &str) -> Result<MobData, Box<dyn std::error::Error>> {
    let data_path = Path::new(dir_path).join("data.json");
    let contents = fs::read_to_string(data_path)?;
    let mob_data: MobData = serde_json::from_str(&contents)?;
    Ok(mob_data)
}

#[derive(Clone)]
pub struct Mob {
    pub entity: Entity,
    mob_data: MobData,
    HP: i32,
    max_HP: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    ai_function: Option<fn(&mut Mob)>,
}

impl Mob {
    pub fn new(x: i32, y: i32, mob_data: &MobData) -> Self {
        let entity = Entity::new(
            x,
            y,
            mob_data.visuals.glyph as u16,
            mob_data.visuals.color,
            BLACK,
        );

        Mob {
            entity,
            mob_data: mob_data.clone(),  // Clone the MobData here
            HP: mob_data.hp,
            max_HP: mob_data.hp,
            attack: mob_data.attack,
            defense: mob_data.defense,
            speed: mob_data.speed,
            ai_function: None,
        }
    }


    pub fn draw(&self, ctx: &mut BTerm, map: &Map) {
        self.entity.draw(ctx, map);
    }

    pub fn set_ai(&mut self, ai_function: fn(&mut Mob)) {
        self.ai_function = Some(ai_function);
    }

    pub fn execute_ai(&mut self) {
        if let Some(ai_function) = self.ai_function {
            ai_function(self);
        }
    }

    // Add mob-specific methods here
}
