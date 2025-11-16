use crate::{Recipe, Resource, ResourceMap};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    resources: Vec<String>,
    recipes: HashMap<String, RecipeConfig>,
}

#[derive(Debug, Deserialize)]
struct RecipeConfig {
    name: String,
    ingredients: HashMap<String, f64>,
    outputs: HashMap<String, f64>,
    requires: HashMap<String, f64>,
    ticks: i32,
}

pub fn load_resources_and_recipes(
    config_path: &str,
) -> Result<(ResourceMap, Vec<(String, String, Recipe)>), Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;

    // Create resource map
    let mut resource_map = ResourceMap::new();
    for resource_name in config.resources {
        resource_map.create_resource(&resource_name);
    }

    // Create recipes with their display names
    let mut recipes = Vec::new();
    for (recipe_key, recipe_config) in config.recipes {
        let recipe = Recipe {
            ingredients: convert_resource_map(&recipe_config.ingredients, &resource_map)?,
            outputs: convert_resource_map(&recipe_config.outputs, &resource_map)?,
            requires: convert_resource_map(&recipe_config.requires, &resource_map)?,
            ticks: recipe_config.ticks,
        };
        recipes.push((recipe_config.name, recipe_key, recipe));
    }

    Ok((resource_map, recipes))
}

fn convert_resource_map(
    string_map: &HashMap<String, f64>,
    resource_map: &ResourceMap,
) -> Result<HashMap<Resource, f64>, Box<dyn std::error::Error>> {
    let mut result = HashMap::new();
    for (resource_name, amount) in string_map {
        if !resource_map.has_resource_by_name(resource_name) {
            return Err(format!("Unknown resource: {}", resource_name).into());
        }
        let resource = resource_map.get_resource(resource_name);
        result.insert(resource, *amount);
    }
    Ok(result)
}
