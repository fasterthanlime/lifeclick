#![allow(non_upper_case_globals)]

use super::idgen::idgen;
use super::units::*;
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemCategory {
    Harvest,
    Initiative,
    Event,
}

#[derive(Debug)]
pub struct ItemSpec {
    pub id: i64,
    pub category: ItemCategory,
    pub name: &'static str,
    pub desc: &'static str,
    pub initial_cost: Souls,
    pub souls_per_click: Option<Souls>,
    pub souls_per_tick: Option<Souls>,
    pub birth_rate_modifier: Option<f64>,
    pub death_rate_modifier: Option<f64>,

    pub pop_multiplier: Option<f64>,
    pub pop_kill_ratio: Option<f64>,
    pub unique: bool,

    pub min_hell_favor: Option<Souls>,
    pub min_heaven_favor: Option<Souls>,
}

impl Hash for ItemSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl std::cmp::PartialEq for ItemSpec {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl std::cmp::Eq for ItemSpec {}

impl Default for ItemSpec {
    fn default() -> Self {
        Self {
            id: idgen(),
            category: ItemCategory::Harvest,
            name: "<missing>",
            desc: "",
            initial_cost: Souls(1),
            souls_per_click: None,
            souls_per_tick: None,
            birth_rate_modifier: None,
            death_rate_modifier: None,

            pop_multiplier: None,
            pop_kill_ratio: None,
            unique: false,

            min_heaven_favor: None,
            min_hell_favor: None,
        }
    }
}

const COST_INCREASE_FACTOR: f64 = 1.12;

impl ItemSpec {
    pub fn instantiate(&'static self, quantity: i64) -> Item {
        Item {
            spec: self,
            quantity,
            revealed: quantity > 0,
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
    pub revealed: bool,
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
        initial_cost: Souls(15),
        souls_per_click: Some(Souls(1)),
        ..Default::default()
    };
    pub static ref Bailiff: ItemSpec = ItemSpec {
        name: "Bailiff",
        desc: "Collecting souls was a logical next career step.",
        initial_cost: Souls(280),
        souls_per_tick: Some(Souls(30)),
        ..Default::default()
    };
    pub static ref CollectionAgency: ItemSpec = ItemSpec {
        name: "Collection agency",
        desc: "Sharing a coffee machine cuts down costs. It's about the small efficiencies!",
        initial_cost: Souls(5_000),
        souls_per_tick: Some(Souls(800)),
        ..Default::default()
    };
    pub static ref CollectionMultinational: ItemSpec = ItemSpec {
        name: "Collection multinational",
        desc: "Very efficient at collecting souls",
        initial_cost: Souls(100_000),
        souls_per_tick: Some(Souls(25_000)),
        ..Default::default()
    };
    pub static ref SurvivalInstinct: ItemSpec = ItemSpec {
        name: "Fertility rates",
        category: ItemCategory::Initiative,
        desc: "",
        initial_cost: Souls(250),
        birth_rate_modifier: Some(0.01),
        ..Default::default()
    };
    pub static ref KillerInstinct: ItemSpec = ItemSpec {
        name: "Killer instinct",
        category: ItemCategory::Initiative,
        desc: "",
        initial_cost: Souls(100),
        death_rate_modifier: Some(0.01),
        ..Default::default()
    };
    pub static ref SoulFission: ItemSpec = ItemSpec {
        name: "Soul fission",
        category: ItemCategory::Event,
        desc: "Double the population",
        initial_cost: Souls(400),
        pop_multiplier: Some(2.0),
        unique: true,
        ..Default::default()
    };
    pub static ref SoulFission2: ItemSpec = ItemSpec {
        name: "Soul fission 2",
        category: ItemCategory::Event,
        desc: "Double the population",
        initial_cost: 1 * Souls::M,
        pop_multiplier: Some(2.0),
        unique: true,
        ..Default::default()
    };
    pub static ref PlagueSmall: ItemSpec = ItemSpec {
        name: "Small Plague",
        category: ItemCategory::Event,
        desc: "Kill 90% of the population",
        initial_cost: Souls(400),
        pop_kill_ratio: Some(0.9),
        unique: true,
        ..Default::default()
    };
    pub static ref PlagueLarge: ItemSpec = ItemSpec {
        name: "Small Plague",
        category: ItemCategory::Event,
        desc: "Kill 99% of the population",
        initial_cost: 2 * Souls::M,
        pop_kill_ratio: Some(0.90),
        unique: true,
        ..Default::default()
    };
}
