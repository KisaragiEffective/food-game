use std::any::TypeId;
use std::collections::HashMap;
use crate::kitchenware::KitchenwareTag;
use crate::material::MaterialTag;

pub struct KitchenwareRepairElementRegistry(HashMap<TypeId, Box<dyn MaterialTag>>);

impl KitchenwareRepairElementRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
