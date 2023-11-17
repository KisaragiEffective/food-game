use std::any::Any;
use crate::ingredient::{IngredientTag, IntermediateIngredient};
use crate::Recipe;
use crate::registry::ingredient::IngredientRegistry;
use crate::unit::{LiquidUnit, SolidUnit};

#[derive(Default)]
pub struct Rice;

impl IngredientTag for Rice {
    type MeasuringUnit = SolidUnit;
}

#[derive(Default)]
pub struct Water;

impl IngredientTag for Water {
    type MeasuringUnit = LiquidUnit;
}

/// ごはん
#[derive(Default)]
pub struct CookedRice;

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