use std::ops::Mul;
use dyn_clone::DynClone;

pub trait MeasuringUnit: DynClone {
    fn as_u64(&self) -> Option<u64>;
}

dyn_clone::clone_trait_object!(MeasuringUnit);

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct SolidUnit(u64);

impl SolidUnit {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl Mul<u64> for SolidUnit {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        SolidUnit::new(self.0 * rhs)
    }
}

impl MeasuringUnit for SolidUnit {
    fn as_u64(&self) -> Option<u64> {
        Some(self.0)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct LiquidUnit(u64);

impl LiquidUnit {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn zero() -> Self {
        Self::new(0)
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl Mul<u64> for LiquidUnit {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl MeasuringUnit for LiquidUnit {
    fn as_u64(&self) -> Option<u64> {
        Some(self.0)
    }
}

