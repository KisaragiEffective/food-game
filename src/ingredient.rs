use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashSet;
use crate::registry::ingredient::IngredientRegistry;
use crate::unit::MeasuringUnit;

pub trait IngredientTag: Default {
    type MeasuringUnit: MeasuringUnit;
}

pub trait IntermediateIngredient: IngredientTag {
    type Ingredients;
    type Cooked: IntermediateIngredient;

    /// 推移律を満たすような実装が必要。すなわち、
    fn has_a_ingredient_of<T: IngredientTag + Any + 'static>(cache: &RefCell<IngredientRequirementComputationCache>) -> bool;

    fn try_make(ingredient: &IngredientRegistry, quantity: Self::MeasuringUnit) -> Result<(Self::Cooked, Self::MeasuringUnit), ()> where Self::MeasuringUnit: Sized;
}

pub struct IngredientRequirementComputationCache(HashSet<TypeId>);

impl IngredientRequirementComputationCache {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert<T: IngredientTag + 'static>(&mut self) {
        self.0.insert(TypeId::of::<T>());
    }

    pub fn contains<T: IngredientTag + 'static>(&self) -> bool {
        self.0.contains(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};
    use std::cell::RefCell;
    use std::sync::mpsc;
    use std::time::Duration;
    use std::hint::black_box;
    use crate::ingredient::{IngredientRequirementComputationCache, IngredientTag, IntermediateIngredient};
    use crate::registry::ingredient::IngredientRegistry;
    use crate::unit::SolidUnit;

    macro_rules! mm {
        ($sel:ident, $ty1:ident, $ty2:ident) => {
            #[derive(Default)]
            struct $sel;

            impl IngredientTag for $sel { type MeasuringUnit = SolidUnit; }

            impl IntermediateIngredient for $sel {
                type Ingredients = ($ty1, $ty2);
                type Cooked = Self;

                #[cfg_attr(test, inline(never))]
                fn has_a_ingredient_of<T: IngredientTag + Any + 'static>(cache: &RefCell<IngredientRequirementComputationCache>) -> bool {
                    black_box(TypeId::of::<T>() == TypeId::of::<$ty1>()) ||
                    black_box(TypeId::of::<T>() == TypeId::of::<$ty2>()) ||
                    black_box($ty1::has_a_ingredient_of::<T>(cache)) ||
                    black_box($ty2::has_a_ingredient_of::<T>(cache))
                }

                fn try_make(_ingredient: &IngredientRegistry, _quantity: Self::MeasuringUnit) -> Result<(Self::Cooked, Self::MeasuringUnit), ()> where Self::MeasuringUnit: Sized {
                    unimplemented!()
                }
            }
        };

        ($sel:ident, $ty1:ident) => {
            #[derive(Default)]
            struct $sel;

            impl IngredientTag for $sel { type MeasuringUnit = SolidUnit; }

            impl IntermediateIngredient for $sel {
                type Ingredients = ($ty1,);
                type Cooked = Self;

                #[cfg_attr(test, inline(never))]
                fn has_a_ingredient_of<T: IngredientTag + Any + 'static>(cache: &RefCell<IngredientRequirementComputationCache>) -> bool {
                    TypeId::of::<T>() == TypeId::of::<$ty1>()
                }

                fn try_make(_ingredient: &IngredientRegistry, _quantity: Self::MeasuringUnit) -> Result<(Self::Cooked, Self::MeasuringUnit), ()> where Self::MeasuringUnit: Sized {
                    unimplemented!()
                }
            }
        };
        ($sel:ident) => {
            #[derive(Default)]
            struct $sel;

            impl IngredientTag for $sel { type MeasuringUnit = SolidUnit; }

            impl IntermediateIngredient for $sel {
                type Ingredients = ();
                type Cooked = Self;

                fn has_a_ingredient_of<T: IngredientTag + Any + 'static>(_cache: &RefCell<IngredientRequirementComputationCache>) -> bool {
                    false
                }

                fn try_make(_ingredient: &IngredientRegistry, _quantity: Self::MeasuringUnit) -> Result<(Self::Cooked, Self::MeasuringUnit), ()> where Self::MeasuringUnit: Sized {
                    unimplemented!()
                }
            }
        };
    }

    mm!(I1);
    mm!(I2, I1);
    mm!(I3, I2, I1);
    mm!(I4, I3, I2);
    mm!(I5, I4, I3);
    mm!(I6, I5, I4);
    mm!(I7, I6, I5);
    mm!(I8, I7, I6);
    mm!(I9, I8, I7);
    mm!(I10, I9, I8);
    mm!(I11, I10, I9);
    mm!(I12, I11, I10);
    mm!(I13, I12, I11);
    mm!(I14, I13, I12);
    mm!(I15, I14, I13);
    mm!(I16, I15, I14);
    mm!(I17, I16, I15);
    mm!(I18, I17, I16);
    mm!(I19, I18, I17);
    mm!(I20, I19, I18);
    mm!(I21, I20, I19);
    mm!(I22, I21, I20);
    mm!(I23, I22, I21);
    mm!(I24, I23, I22);
    mm!(I25, I24, I23);
    mm!(I26, I25, I24);
    mm!(I27, I26, I25);
    mm!(I28, I27, I26);
    mm!(I29, I28, I27);
    mm!(I30, I29, I28);
    mm!(I31, I30, I29);
    mm!(I32, I31, I30);
    mm!(I33, I32, I31);
    mm!(I34, I33, I32);
    mm!(I35, I34, I33);
    mm!(I36, I35, I34);
    mm!(I37, I36, I35);
    mm!(I38, I37, I36);
    mm!(I39, I38, I37);
    mm!(I40, I39, I38);
    mm!(I41, I40, I39);
    mm!(I42, I41, I40);
    mm!(I43, I42, I41);
    mm!(I44, I43, I42);
    mm!(I45, I44, I43);
    mm!(I46, I45, I44);
    mm!(I47, I46, I45);
    mm!(I48, I47, I46);
    mm!(I49, I48, I47);
    mm!(I50, I49, I48);

    #[test]
    fn avoid_exponent_time() {
        std::thread::scope(|scope| {
            let (sender, receiver) = mpsc::channel();

            let _ = scope.spawn(move || {
                let rc = RefCell::new(IngredientRequirementComputationCache::new());
                let a = I50::has_a_ingredient_of::<I1>(&rc);
                sender.send(a).expect("cannot send");
            });

            let res = receiver.recv_timeout(Duration::from_millis(5000));
            assert!(res.expect("must not be exponent time"), "I50's dependency tree must include I1");
        });
    }
}
