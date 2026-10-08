use std::collections::HashMap;

use crate::{Recipe, ResourceMap, StateVec};

pub struct Game {
    pub resources: ResourceMap,
    pub recipes: HashMap<String, Recipe>,
    pub inventory: StateVec,
    pub time: i64,
    pub recipe_progress: Option<(String, i64, i64)>,
}

impl Game {
    pub fn new(resources: ResourceMap) -> Self {
        let inventory = StateVec::empty(&resources);
        Self {
            resources,
            recipes: HashMap::new(),
            inventory,
            time: 0,
            recipe_progress: None,
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

    fn has_all_requires(&self, recipe: &Recipe) -> bool {
        recipe
            .requires
            .iter()
            .all(|(resource, amount)| self.inventory[*resource] >= *amount)
    }

    fn has_all_ingredients(&self, recipe: &Recipe) -> bool {
        recipe
            .ingredients
            .iter()
            .all(|(resource, amount)| self.inventory[*resource] >= *amount)
    }

    pub fn can_do_recipe(&self, recipe: &Recipe) -> bool {
        self.has_all_requires(recipe) && self.has_all_ingredients(recipe)
    }

    pub fn tick(&mut self, recipe_name: &str) -> Result<(), String> {
        self.tick_n(recipe_name, 1)
    }

    pub fn tick_n(&mut self, recipe_name: &str, n_ticks: u64) -> Result<(), String> {
        let recipe = self
            .recipes
            .get(recipe_name)
            .ok_or(format!("Recipe {} not found", recipe_name))?;
        let can_do_recipe = self.can_do_recipe(recipe);
        if !can_do_recipe {
            return Err(format!("Recipe {} is missing prerequisites", recipe_name));
        }

        let next_progress = match &self.recipe_progress {
            Some((progress_recipe_name, progress, ticks))
                if progress_recipe_name == recipe_name =>
            {
                (
                    recipe_name.to_string(),
                    progress + n_ticks as i64,
                    recipe.ticks as i64,
                )
            }
            Some((_, _, _)) | None => {
                (recipe_name.to_string(), n_ticks as i64, recipe.ticks as i64)
            }
        };

        self.recipe_progress = match next_progress {
            (_, progress, ticks) if progress >= ticks => {
                let mut new_state = self.inventory.clone();
                for (resource, amount) in recipe.ingredients.iter() {
                    new_state[*resource] -= *amount;
                }
                for (resource, amount) in recipe.outputs.iter() {
                    new_state[*resource] += amount;
                }
                self.inventory = new_state;

                Some((recipe_name.to_string(), 0, recipe.ticks as i64))
            }
            _ => Some(next_progress),
        };

        self.time += n_ticks as i64;
        Ok(())
    }
}
