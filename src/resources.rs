use std::{collections::HashMap, ops::Index};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Resource(pub usize);

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
        resource.0 < self.resource_names.len()
    }
}
