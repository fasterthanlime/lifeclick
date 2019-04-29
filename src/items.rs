#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use super::units::*;

use lazy_static::lazy_static;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ItemSpec {
    pub name: &'static str,
    pub desc: &'static str,
    pub initial_cost: Souls,
    pub souls_per_click: Option<Souls>,
    pub souls_per_tick: Option<Souls>,
}

impl Default for ItemSpec {
    fn default() -> Self {
        Self {
            name: "<missing>",
            desc: "",
            initial_cost: Souls(1),
            souls_per_click: None,
            souls_per_tick: None,
        }
    }
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

lazy_static! {
    pub static ref Sickle: ItemSpec = ItemSpec {
        name: "Sickle",
        desc: "The simplest tools are sometimes the most effective.",
        initial_cost: Souls(10),
        souls_per_click: Some(Souls(1)),
        ..Default::default()
    };
    pub static ref Sickle2: ItemSpec = ItemSpec {
        name: "Double-edged sickle",
        desc: "A slight improvement on the original design, allows you to collect two souls in a single sweep.",
        initial_cost: Souls(100),
        souls_per_click: Some(Souls(2)),
        ..Default::default()
    };
    pub static ref RoboHarvest: ItemSpec = ItemSpec {
        name: "Bailiff",
        desc: "Collecting souls was a logical next career step.",
        initial_cost: Souls(500),
        souls_per_tick: Some(Souls(1)),
        ..Default::default()
    };
    pub static ref MechaHarvest: ItemSpec = ItemSpec {
        name: "Collection agency",
        desc: "Sharing a coffee machine cuts down costs. It's about the small efficiencies!",
        initial_cost: Souls(5000),
        souls_per_tick: Some(Souls(7)),
        ..Default::default()
    };
}
