#![allow(non_upper_case_globals)]

use super::idgen::idgen;
use super::items;
use super::items::ItemSpec;
use super::units::*;
use lazy_static::lazy_static;

use indoc::indoc;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct UpgradeSpec {
    pub id: i64,
    pub cost: Souls,
    pub name: &'static str,
    pub desc: &'static str,
    pub effects: Vec<UpgradeEffect>,
}

#[derive(Debug)]
pub struct UpgradeEffect {
    pub spec: &'static ItemSpec,
    pub spc_mod: Option<f64>,
    pub spt_mod: Option<f64>,
}

impl Default for UpgradeSpec {
    fn default() -> Self {
        Self {
            id: idgen(),
            cost: Souls(1),
            name: "<missing>",
            desc: "",
            effects: vec![],
        }
    }
}

impl Default for UpgradeEffect {
    fn default() -> Self {
        Self {
            spec: &items::ItemNone,
            spc_mod: None,
            spt_mod: None,
        }
    }
}

impl Hash for UpgradeSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl std::cmp::PartialEq for UpgradeSpec {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl std::cmp::Eq for UpgradeSpec {}

impl UpgradeSpec {
    pub fn instantiate(&'static self) -> Upgrade {
        Upgrade {
            spec: &self,
            revealed: false,
            bought: false,
        }
    }
}

#[derive(Debug)]
pub struct Upgrade {
    pub spec: &'static UpgradeSpec,
    pub revealed: bool,
    pub bought: bool,
}

// upgrade definitions
lazy_static! {
    pub static ref PaidInterns: UpgradeSpec = UpgradeSpec {
        name: "Paid interns",
        desc: indoc!(
            "
            It's not like you have a fiber of morality in your ethereal body, but..
            interns do work at least 50% harder when paid"
        ),
        cost: Souls(1_000),
        effects: vec![UpgradeEffect {
            spec: &items::Intern,
            spc_mod: Some(0.5),
            ..Default::default()
        }],
        ..Default::default()
    };
    pub static ref InternRaise1: UpgradeSpec = UpgradeSpec {
        name: "Double intern pay",
        desc: indoc!(
            "
            It's not like you have a fiber of morality in your ethereal body, but..
            interns do work at least 50% harder when paid"
        ),
        cost: Souls(10_000),
        effects: vec![UpgradeEffect {
            spec: &items::Intern,
            spc_mod: Some(0.5),
            ..Default::default()
        }],
        ..Default::default()
    };
    pub static ref ArmedBailiffs: UpgradeSpec = UpgradeSpec {
        name: "Armed bailiffs",
        desc: indoc!(
            "
            Bailiffs come armed with shotguns, increasing efficiency by 50%."
        ),
        cost: Souls(50_000),
        effects: vec![UpgradeEffect {
            spec: &items::Bailiff,
            spt_mod: Some(0.5),
            ..Default::default()
        }],
        ..Default::default()
    };
}
