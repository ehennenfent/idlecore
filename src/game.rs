use std::collections::HashMap;

use crate::{Recipe, ResourceMap, StateVec};

pub struct Game {
    pub resources: ResourceMap,
    pub recipes: HashMap<String, Recipe>,
    pub inventory: StateVec,
    pub time: i64,
}

impl Game {
    pub fn new(resources: ResourceMap) -> Self {
        let inventory = StateVec::empty(&resources);
        Self {
            resources,
            recipes: HashMap::new(),
            inventory,
            time: 0,
        }
    }

    pub fn add_recipe(&mut self, name: &str, recipe: Recipe) -> Result<(), String> {
        if self.recipes.contains_key(name) {
            return Err(format!("Recipe {} already exists", name));
        }
        let all_valid_ingredients = recipe
            .ingredients
            .keys()
            .all(|key| self.resources.has_resource(*key));
        let all_valid_outputs = recipe
            .outputs
            .keys()
            .all(|key| self.resources.has_resource(*key));
        let all_valid_requires = recipe
            .requires
            .keys()
            .all(|key| self.resources.has_resource(*key));
        if !all_valid_ingredients || !all_valid_outputs || !all_valid_requires {
            return Err(format!("Recipe {} has invalid resources", name));
        }
        self.recipes.insert(name.to_string(), recipe);
        Ok(())
    }
}
