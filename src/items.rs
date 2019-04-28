#![allow(non_upper_case_globals)]

use super::units::*;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Item {
    pub name: &'static str,
    pub initial_cost: Souls,
}

impl Item {
    pub fn instantiate(&'static self, quantity: i64) -> ItemState {
        ItemState {
            item: self,
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct ItemState {
    pub item: &'static Item,
    pub quantity: i64,
}

impl ItemState {
    pub fn name(&self) -> &str {
        &self.item.name
    }

    pub fn cost(&self) -> Souls {
        let cost = self.item.initial_cost.0 as f64;
        let cost = cost * (1.2f64).powf(self.quantity as f64);
        Souls(cost as i64)
    }

    pub fn quantity(&self) -> i64 {
        return self.quantity;
    }
}

pub const Sickle: Item = Item {
    name: "Sickle",
    initial_cost: Souls(10),
};
