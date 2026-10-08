use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    // resources required to make the recipe, consumed by producing it
    pub ingredients: HashMap<Resource, i64>,
    // resources produced by the recipe
    pub outputs: HashMap<Resource, i64>,
    // resources required to produce the recipe, not consumed
    pub requires: HashMap<Resource, i64>,
    // ticks needed to fully produce the recipe
    pub ticks: i32,
}
