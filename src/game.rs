use std::collections::HashMap;

use crate::{Recipe, ResourceMap, StateVec};

const EPSILON: f64 = 1e-10;
fn float_ge(a: f64, b: f64) -> bool {
    a >= b - EPSILON
}

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

    pub fn tick(&mut self, recipe_name: &str) -> Result<(), String> {
        let recipe = self
            .recipes
            .get(recipe_name)
            .ok_or(format!("Recipe {} not found", recipe_name))?;
        let has_all_requires = recipe
            .requires
            .iter()
            .all(|(resource, amount)| float_ge(self.inventory[*resource], *amount));
        if !has_all_requires {
            return Err(format!("Recipe {} is missing prerequisites", recipe_name));
        }
        let has_all_ingredients = recipe
            .ingredients
            .iter()
            .all(|(resource, amount)| float_ge(self.inventory[*resource], *amount / recipe.ticks as f64));
        if !has_all_ingredients {
            return Err(format!("Recipe {} is missing ingredients", recipe_name));
        }
        let mut new_state = self.inventory.clone();
        for (resource, amount) in recipe.ingredients.iter() {
            new_state[*resource] -= *amount / recipe.ticks as f64;
        }
        for (resource, amount) in recipe.outputs.iter() {
            new_state[*resource] += amount / recipe.ticks as f64;
        }
        self.inventory = new_state;

        self.time += 1;
        Ok(())
    }
}
