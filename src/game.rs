use std::cell::RefCell;
use crate::registry::ingredient::IngredientRegistry;
use crate::unit::{LiquidUnit, MeasuringUnit};
use crate::Recipe;
use crate::food::Water;

struct Game {
    倉庫: RefCell<IngredientRegistry>,
    
}

impl Game {
    pub fn new() -> Self {
        Self {
            倉庫: RefCell::new(IngredientRegistry::new()),
        }
    }

    fn make_recipe<MU: MeasuringUnit>(&self, recipe: Recipe<MU>) {
        todo!()
    }
    
    fn tick(&self) {
        self.倉庫.borrow_mut().insert(Water, LiquidUnit::new(100));

    }
}