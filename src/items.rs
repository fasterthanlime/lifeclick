#![allow(non_upper_case_globals)]

use super::idgen::idgen;
use super::units::*;
use super::upgrades::UpgradeEffect;
use super::Model;
use indoc::indoc;
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemCategory {
    Harvest,
    Finance,
    Initiatives,
    Events,
    Upgrades,
}

#[derive(Debug)]
pub struct ItemSpec {
    pub id: i64,
    pub category: ItemCategory,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: Souls,
    pub spc: Option<Souls>,
    pub spt: Option<Souls>,
    pub br_mod: Option<f64>,
    pub dr_mod: Option<f64>,
    pub remit_mod: Option<f64>,
    pub interest: Option<f64>,

    pub min_hell_favor: Option<Souls>,
    pub min_heaven_favor: Option<Souls>,

    // Unique effects
    pub unique: bool,
    pub pop_multiplier: Option<f64>,
    pub pop_kill_ratio: Option<f64>,
}

pub struct Stats {
    pub base: Souls,
    pub effective: Souls,
    pub bonus: f64,
}

impl Stats {
    pub fn multiply(&self, quantity: i64) -> Souls {
        return Souls(self.effective.0 * quantity);
    }
}

impl ItemSpec {
    fn bonus(&self, model: &Model, f: fn(effect: &UpgradeEffect) -> Option<f64>) -> f64 {
        let mut bonus = 1.0f64;
        if let Some(effects) = model.effects.get(self) {
            for effect in effects {
                if let Some(modo) = f(effect) {
                    bonus += modo;
                }
            }
        }
        bonus
    }

    fn effective(
        &self,
        model: &Model,
        qt: Option<Souls>,
        f: fn(effect: &UpgradeEffect) -> Option<f64>,
    ) -> Option<Stats> {
        if let Some(base) = qt {
            let bonus = self.bonus(model, f);
            let effective = Souls((base.0 as f64 * bonus) as i64);
            Some(Stats {
                base,
                effective,
                bonus,
            })
        } else {
            None
        }
    }

    pub fn get_spc(&self, model: &Model) -> Option<Stats> {
        self.effective(model, self.spc, |fx| fx.spc_mod)
    }

    pub fn get_spt(&self, model: &Model) -> Option<Stats> {
        self.effective(model, self.spt, |fx| fx.spt_mod)
    }
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
            cost: Souls(1),
            spc: None,
            spt: None,
            br_mod: None,
            dr_mod: None,

            pop_multiplier: None,
            pop_kill_ratio: None,
            unique: false,

            min_heaven_favor: None,
            min_hell_favor: None,

            interest: None,
            remit_mod: None,
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
        let cost = self.cost.0 as f64;
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
}

// item definitions

lazy_static! {
    pub static ref ItemNone: ItemSpec = ItemSpec {
        ..Default::default()
    };
    //////////////////////////////////////////////////////
    // Harvest
    //////////////////////////////////////////////////////
    pub static ref Intern: ItemSpec = ItemSpec {
        name: "Intern",
        category: ItemCategory::Harvest,
        desc: "A pair of extra sickle-wielding hands.",
        cost: Souls(25),
        spc: Some(Souls(1)),
        ..Default::default()
    };
    pub static ref Bailiff: ItemSpec = ItemSpec {
        name: "Bailiff",
        category: ItemCategory::Harvest,
        desc: "Collecting souls was a logical next career step.",
        cost: Souls(1_500),
        spt: Some(Souls(30)),
        ..Default::default()
    };
    pub static ref CollectionAgency: ItemSpec = ItemSpec {
        name: "Collection agency",
        category: ItemCategory::Harvest,
        desc: "Sharing a coffee machine cuts down costs. It's about the small efficiencies!",
        cost: Souls(120_000),
        spt: Some(Souls(5_000)),
        ..Default::default()
    };
    pub static ref CollectionMultinational: ItemSpec = ItemSpec {
        name: "Collection multinational",
        category: ItemCategory::Harvest,
        desc: "Very efficient at collecting souls",
        cost: Souls(2_000_000),
        spt: Some(Souls(25_000)),
        ..Default::default()
    };
    //////////////////////////////////////////////////////
    // Finance
    //////////////////////////////////////////////////////
    pub static ref Banker: ItemSpec = ItemSpec {
        name: "Banker",
        category: ItemCategory::Finance,
        desc: "Increases your total souls by 1% every month.",
        cost: Souls(30_000),
        interest: Some(0.01),
        ..Default::default()
    };
    pub static ref Accountant: ItemSpec = ItemSpec {
        name: "Accountant",
        category: ItemCategory::Finance,
        desc: "Remits 2% more souls on every transaction... on paper",
        cost: Souls(45_000),
        remit_mod: Some(0.01),
        ..Default::default()
    };
    //////////////////////////////////////////////////////
    // Initiatives
    //////////////////////////////////////////////////////
    pub static ref SurvivalInstinct: ItemSpec = ItemSpec {
        name: "Fertility rates",
        category: ItemCategory::Initiatives,
        desc: "",
        cost: Souls(250),
        br_mod: Some(0.01),
        ..Default::default()
    };
    pub static ref KillerInstinct: ItemSpec = ItemSpec {
        name: "Killer instinct",
        category: ItemCategory::Initiatives,
        desc: "",
        cost: Souls(100),
        dr_mod: Some(0.01),
        ..Default::default()
    };
    //////////////////////////////////////////////////////
    // Events
    //////////////////////////////////////////////////////
    pub static ref SoulFission: ItemSpec = ItemSpec {
        name: "Soul fission",
        category: ItemCategory::Events,
        desc: "Double the population",
        cost: Souls(400),
        pop_multiplier: Some(2.0),
        unique: true,
        ..Default::default()
    };
    pub static ref SoulFission2: ItemSpec = ItemSpec {
        name: "Soul fission 2",
        category: ItemCategory::Events,
        desc: "Double the population",
        cost: Souls(50_000),
        pop_multiplier: Some(2.0),
        unique: true,
        ..Default::default()
    };
    pub static ref PlagueSmall: ItemSpec = ItemSpec {
        name: "Small Plague",
        category: ItemCategory::Events,
        desc: indoc!(
            "
            Kill 90% of the population.
            
            More souls for you!"
        ),
        cost: Souls(400),
        pop_kill_ratio: Some(0.9),
        unique: true,
        ..Default::default()
    };
    pub static ref PlagueLarge: ItemSpec = ItemSpec {
        name: "Large Plague",
        category: ItemCategory::Events,
        desc: indoc!(
            "
            Kill 99% of the population.
            
            Tough luck!"
        ),
        cost: Souls(80_000),
        pop_kill_ratio: Some(0.99),
        unique: true,
        ..Default::default()
    };
}
