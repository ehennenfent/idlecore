use std::ops::{Index, IndexMut};

use crate::{Resource, ResourceMap};

#[derive(Debug, Clone)]
pub struct StateVec {
    pub resources: Vec<i64>,
}

impl StateVec {
    pub fn new(resources: Vec<i64>) -> Self {
        Self { resources }
    }

    pub fn empty(resources: &ResourceMap) -> Self {
        Self {
            resources: vec![0i64; resources.resource_names.len()],
        }
    }
}

impl Index<Resource> for StateVec {
    type Output = i64;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.resources[index.0]
    }
}

impl IndexMut<Resource> for StateVec {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self.resources[index.0]
    }
}
