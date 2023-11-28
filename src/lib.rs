mod utils;
mod unit;
mod registry;
mod game;
mod any;
mod save;
mod subsystem;

use std::any::Any;
use wasm_bindgen::prelude::*;
use crate::subsystem::ingredient::definition::{IngredientTag, IntermediateIngredient};
use subsystem::ingredient::registry::IngredientRegistry;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, food-game!");
}

/// 食材を調理する場合、どの材料がどれだけ要るのか
pub struct Recipe<MU> {
    required_ingredients: IngredientRegistry,
    measuring_unit: MU,
}

impl<MU> Recipe<MU> {
    fn new(registry: IngredientRegistry, measuring_unit: MU) -> Self {
        Self {
            required_ingredients: registry,
            measuring_unit
        }
    }

    fn registry(&self) -> &IngredientRegistry {
        &self.required_ingredients
    }

    fn registry_mut(&mut self) -> &mut IngredientRegistry {
        &mut self.required_ingredients
    }

    fn can_make_from(&self, actual: &IngredientRegistry) -> bool {
        // TODO: impl IntoIter for &IngredientRegistry
        for registry_entry in self.required_ingredients.clone() {
            let (k, required) = registry_entry.into_entry_raw();
            if let Some(actual) = actual.get_raw(&k) {
                if actual.as_u64() < required.as_u64() {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
