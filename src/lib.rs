use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Resource(usize);

#[derive(Debug, Clone)]
pub struct ResourceMap {
    pub resource_names: Vec<String>,
    pub resource_indices: HashMap<String, Resource>,
}

impl Index<Resource> for ResourceMap {
    type Output = String;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.resource_names[index.0]
    }
}

impl Default for ResourceMap {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceMap {
    pub fn new() -> Self {
        Self {
            resource_names: vec![],
            resource_indices: HashMap::new(),
        }
    }

    pub fn create_resource(&mut self, name: &str) -> Resource {
        self.resource_names.push(name.to_string());
        let resource = Resource(self.resource_names.len() - 1);
        self.resource_indices.insert(name.to_string(), resource);
        resource
    }

    pub fn get_resource(&self, name: &str) -> Resource {
        self.resource_indices[name]
    }

    pub fn has_resource_by_name(&self, name: &str) -> bool {
        self.resource_indices.contains_key(name)
    }

    pub fn has_resource(&self, resource: Resource) -> bool {
        resource.0 > 0 && resource.0 < self.resource_names.len()
    }
}

pub struct Recipe {
    // resources required to make the recipe, consumed by producing it
    pub ingredients: HashMap<Resource, f64>,
    // resources produced by the recipe
    pub outputs: HashMap<Resource, f64>,
    // resources required to produce the recipe, not consumed
    pub requires: HashMap<Resource, f64>,
    // ticks needed to fully produce the recipe
    pub ticks: i32,
}

pub struct StateVec {
    pub resources: Vec<f64>,
}

impl StateVec {
    pub fn new(resources: Vec<f64>) -> Self {
        Self { resources }
    }

    pub fn empty(resources: &ResourceMap) -> Self {
        Self {
            resources: vec![0.0; resources.resource_names.len()],
        }
    }
}

impl Index<Resource> for StateVec {
    type Output = f64;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.resources[index.0]
    }
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
}
