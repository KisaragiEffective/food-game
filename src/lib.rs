mod utils;
mod unit;
mod registry;
mod ingredient;
mod game;
mod any;

use std::any::Any;
use wasm_bindgen::prelude::*;
use crate::ingredient::{IngredientTag, IntermediateIngredient};
use registry::ingredient::IngredientRegistry;
use crate::unit::{LiquidUnit, SolidUnit};

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

#[derive(Default)]
struct Rice;

impl IngredientTag for Rice {
    type MeasuringUnit = SolidUnit;
}

#[derive(Default)]
struct Water;

impl IngredientTag for Water {
    type MeasuringUnit = LiquidUnit;
}

/// ごはん
#[derive(Default)]
struct CookedRice;

impl IngredientTag for CookedRice { type MeasuringUnit = SolidUnit; }

impl IntermediateIngredient for CookedRice {
    type Ingredients = ((Rice, Water));
    type Cooked = Self;

    fn has_a_ingredient_of<T: IngredientTag + Any + 'static>() -> bool {
        let a = core::any::TypeId::of::<T>();
        let b = core::any::TypeId::of::<Rice>();
        let c = core::any::TypeId::of::<Water>();

        a == b || a == c
    }

    /// TODO: これ多分「即時に出来上がる」ということはなくて、`Result<Box<dyn Future<Output=Self>, ()>`
    fn try_make(ingredient: &IngredientRegistry, quantity: Self::MeasuringUnit) -> Result<(Self, Self::MeasuringUnit), ()> {
        let minimal_unit = Self::required_minimal_ingredients();
        let rice_actual = *ingredient.get(&Rice).ok_or(())?;
        let water_actual = *ingredient.get(&Water).ok_or(())?;
        let m = quantity.into_inner() % minimal_unit.measuring_unit.into_inner();
        let times = quantity.into_inner() / minimal_unit.measuring_unit.into_inner();
        let times = if m == 0 {
            times
        } else {
            times + 1
        };

        let rice_requires = minimal_unit.required_ingredients.get(&Rice).ok_or(())?.clone() * times;
        let water_requires = minimal_unit.required_ingredients.get(&Water).ok_or(())?.clone() * times;

        if rice_actual > rice_requires && water_actual > water_requires {
            Ok((Self, quantity))
        } else {
            Err(())
        }
    }
}

impl CookedRice {
    /// 最小単位で調理する場合、どの材料がどれだけ要るのか
    fn required_minimal_ingredients() -> Recipe<<Self as IngredientTag>::MeasuringUnit> {
        let mut x = IngredientRegistry::new();
        x.insert(Rice, SolidUnit::new(1));
        x.insert(Water, LiquidUnit::new(1));

        Recipe::new(x, SolidUnit::new(1))
    }
}

/// 食材を調理する場合、どの材料がどれだけ要るのか
struct Recipe<MU> {
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
