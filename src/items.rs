#![allow(non_upper_case_globals)]

use super::units::*;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ItemSpec {
    pub name: &'static str,
    pub initial_cost: Souls,
}

impl ItemSpec {
    pub fn instantiate(&'static self, quantity: i64) -> Item {
        Item {
            spec: self,
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub spec: &'static ItemSpec,
    pub quantity: i64,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.spec.name
    }

    pub fn cost(&self) -> Souls {
        let cost = self.spec.initial_cost.0 as f64;
        let cost = cost * (1.42f64).powf(self.quantity as f64);
        Souls(cost as i64)
    }

    pub fn quantity(&self) -> i64 {
        return self.quantity;
    }
}

// item definitions

pub const Sickle: ItemSpec = ItemSpec {
    name: "Sickle",
    initial_cost: Souls(10),
};

pub const RoboHarvest: ItemSpec = ItemSpec {
    name: "Robo Harvest",
    initial_cost: Souls(220),
};
