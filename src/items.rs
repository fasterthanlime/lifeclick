use super::units::*;

pub struct Item {
    pub name: String,
    pub initial_cost: Souls,
}

pub struct ItemState {
    pub item: Item,
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
