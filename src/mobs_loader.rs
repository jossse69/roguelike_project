use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::mob::{Mob, MobData};
use crate::map::Map;

pub struct MobLoader {
    mob_folders: Vec<PathBuf>,
}

impl MobLoader {
    pub fn new() -> Self {
        // List all mob folders in the "data/mobs" directory
        let mut mob_folders = Vec::new();
        let mobs_dir = Path::new("data/mobs");
        if let Ok(entries) = fs::read_dir(mobs_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_type().ok().map_or(false, |ft| ft.is_dir()) {
                        mob_folders.push(entry.path());
                    }
                }
            }
        }

        MobLoader {
            mob_folders,
        }
    }

    pub fn load_mobs(&self) -> Vec<Mob> {
        let mut mobs = Vec::new();

        for folder in &self.mob_folders {
            if let Ok(mob_data) = self.load_mob_data(folder) {
                if let Some(ai_function) = self.load_ai_function(folder) {
                    let mut mob = Mob::new(0, 0, &mob_data.clone());
                    mob.set_ai(ai_function);
                    mobs.push(mob);
                } else {
                    eprintln!("AI module not found for mob: {}", mob_data.name);
                    // Handle other cases or provide a default action
                }
            }
        }

        mobs
    }

    fn load_mob_data(&self, folder: &Path) -> Result<MobData, Box<dyn std::error::Error>> {
        let data_path = folder.join("data.json");
        let contents = fs::read_to_string(data_path)?;
        let mob_data: MobData = serde_json::from_str(&contents)?;
        Ok(mob_data)
    }

    fn load_ai_function(&self, folder: &Path) -> Option<fn(&mut Mob)> {
        let ai_path = folder.join("ai.rs");
        if ai_path.exists() {
            match self.load_ai_module(&ai_path) {
                Ok(ai_function) => Some(ai_function),
                Err(err) => {
                    eprintln!("Failed to load AI module: {:?}", err);
                    None
                }
            }
        } else {
            eprintln!("AI module not found for folder: {:?}", folder);
            None
        }
    }

    fn load_ai_module(&self, ai_module_path: &Path) -> Result<fn(&mut Mob), Box<dyn std::error::Error>> {
        // Load and compile the AI module from the file
        let ai_module_code = fs::read_to_string(ai_module_path)?;
        // You would need to compile the code here and return the AI function
        // For simplicity, I'll just provide a stub function
        let ai_function: fn(&mut Mob) = |_| {
            // Implement your AI logic here
        };

        Ok(ai_function)
    }
}
