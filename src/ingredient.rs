use std::any::Any;
use crate::registry::IngredientRegistry;
use crate::unit::MeasuringUnit;

pub trait IngredientTag: Default {
    type MeasuringUnit: MeasuringUnit;
}

pub trait IntermediateIngredient: IngredientTag {
    type Ingredients;
    type Cooked: IntermediateIngredient;

    /// 推移律を満たすような実装が必要。すなわち、
    fn has_a_ingredient_of<T: IngredientTag + Any + 'static>() -> bool;

    fn try_make(ingredient: &IngredientRegistry, quantity: Self::MeasuringUnit) -> Result<(Self::Cooked, Self::MeasuringUnit), ()> where Self::MeasuringUnit: Sized;
}
