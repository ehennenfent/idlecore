use std::ops::Index;

use crate::{Resource, ResourceMap};

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
