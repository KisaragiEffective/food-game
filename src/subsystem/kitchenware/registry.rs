use std::any::TypeId;
use std::collections::HashMap;
use crate::subsystem::kitchenware::definition::KitchenwareTag;
use crate::subsystem::material::definition::MaterialTag;

pub struct KitchenwareRepairElementRegistry(HashMap<TypeId, Box<dyn MaterialTag>>);

impl KitchenwareRepairElementRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
