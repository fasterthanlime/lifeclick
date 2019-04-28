#![allow(non_upper_case_globals)]

use super::units::*;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ItemSpec {
    pub name: &'static str,
    pub desc: &'static str,
    pub initial_cost: Souls,
}

const COST_INCREASE_FACTOR: f64 = 1.12;

impl ItemSpec {
    pub fn instantiate(&'static self, quantity: i64) -> Item {
        Item {
            spec: self,
            quantity,
        }
    }

    pub fn ith_cost(&'static self, i: i64) -> Souls {
        let cost = self.initial_cost.0 as f64;
        let cost = cost * COST_INCREASE_FACTOR.powf(i as f64);
        Souls(cost as i64)
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
        self.spec.ith_cost(self.quantity)
    }

    pub fn cost_n(&self, n: i64) -> Souls {
        let mut total = Souls(0);
        for i in 0..n {
            total += self.spec.ith_cost(self.quantity + i);
        }
        total
    }

    pub fn quantity(&self) -> i64 {
        return self.quantity;
    }
}

// item definitions

pub const Sickle: ItemSpec = ItemSpec {
    name: "Sickle",
    desc: "Harvests 1 soul / click",
    initial_cost: Souls(10),
};

pub const RoboHarvest: ItemSpec = ItemSpec {
    name: "Robo Harvest",
    desc: "Harvests 1 soul / month",
    initial_cost: Souls(100),
};
