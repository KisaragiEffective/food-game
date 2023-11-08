use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::any;
use crate::ingredient::IngredientTag;
use crate::unit::MeasuringUnit;

/// Express associative array with key of I and type of I::MeasuringUnit where I is Ingredient.
#[derive(Clone)]
pub struct IngredientRegistry {
    // Map<I, I::MeasuringUnit>
    inner: HashMap<TypeId, Box<dyn MeasuringUnit>>
}

impl IngredientRegistry {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    pub fn insert<I: IngredientTag + Any>(&mut self, key: I, value: I::MeasuringUnit) {
        // SAFETY: function signature enforces safety contract to caller.
        unsafe {
            self.insert_unchecked(key.type_id(), Box::new(value));
        }
    }

    /// # Safety
    /// It leads to memory corruption if `GetActualMeasuringUnit<typeof(key)> != typeof(value)`.
    pub unsafe fn insert_unchecked(&mut self, key: TypeId, value: Box<dyn MeasuringUnit>) {
        self.inner.insert(key.type_id(), value);
    }

    pub fn get<I: IngredientTag + Any>(&self, key: &I) -> Option<&I::MeasuringUnit> {
        self.inner.get(&key.type_id()).map(|x| unsafe { any::downcast_ref_unchecked(x) })
    }

    pub fn get_raw(&self, key: &TypeId) -> Option<&dyn MeasuringUnit> {
        self.inner.get(key).map(|x| x.as_ref())
    }
}

impl IntoIterator for IngredientRegistry {
    type Item = RegistryEntryGuard;
    type IntoIter = RegistryIterator;

    fn into_iter(self) -> Self::IntoIter {
        RegistryIterator(self.inner.into_iter())
    }
}

pub struct RegistryIterator(std::collections::hash_map::IntoIter<TypeId, Box<dyn MeasuringUnit>>);

impl Iterator for RegistryIterator {
    type Item = RegistryEntryGuard;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, value)| RegistryEntryGuard { key, value })
    }
}

/// invariant: typeof(self.key)::MeasuringUnit == typeof(self.value)
pub struct RegistryEntryGuard {
    key: TypeId,
    value: Box<dyn MeasuringUnit>
}

impl RegistryEntryGuard {
    pub fn into_key(self) -> TypeId {
        self.key
    }

    pub fn into_value(self) -> Box<dyn MeasuringUnit> {
        self.value
    }

    pub fn into_entry_raw(self) -> (TypeId, Box<dyn MeasuringUnit>) {
        (self.key, self.value)
    }

    pub fn try_into_entry<I: IngredientTag + Any>(self) -> Option<(I, Box<I::MeasuringUnit>)> {
        if TypeId::of::<I>() == self.key {
            // this should be optimized away
            let raw = Box::into_raw(self.value);
            // SAFETY: type invariant ensures this.
            let value = unsafe {
                Box::from_raw(raw.cast())
            };

            Some((I::default(), value))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use crate::ingredient::IngredientTag;
    use crate::registry::IngredientRegistry;
    use crate::unit::{LiquidUnit, SolidUnit};
    use crate::{CookedRice, Rice, Water};

    // #[cfg(miri)]
    #[test]
    #[should_panic]
    pub fn a() {
        #[derive(Default)]
        struct ActuallyLiquid;

        impl IngredientTag for ActuallyLiquid {
            type MeasuringUnit = LiquidUnit;
        }

        let mut registry = IngredientRegistry::new();
        unsafe { registry.insert_unchecked(TypeId::of::<ActuallyLiquid>(), Box::new(SolidUnit::new(1))); }

        assert_eq!(registry.get(&ActuallyLiquid).unwrap().into_inner(), 1);
    }

    #[test]
    pub fn b() {
        let mut now = IngredientRegistry::new();
        now.insert(Water, LiquidUnit::new(100));
        now.insert(Rice, SolidUnit::new(100));

        assert!(CookedRice::required_minimal_ingredients().can_make_from(&now));
    }
}