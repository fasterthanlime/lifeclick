#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use super::idgen::idgen;
use indoc::indoc;
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct EventSpec {
    pub id: i64,
    pub name: &'static str,
    pub desc: &'static str,
}

#[derive(Debug)]
pub struct Event {
    pub spec: &'static EventSpec,
    pub consumed: bool,
}

impl Hash for EventSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl std::cmp::PartialEq for EventSpec {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl std::cmp::Eq for EventSpec {}

impl Default for EventSpec {
    fn default() -> Self {
        Self {
            name: "<untitled>",
            desc: "",
            id: idgen(),
        }
    }
}

impl EventSpec {
    pub fn instantiate(&'static self) -> Event {
        Event {
            spec: self,
            consumed: false,
        }
    }
}

// event definitions

lazy_static! {
    pub static ref Welcome: EventSpec = EventSpec {
        name: "Welcome to Death Inc.",
        desc: indoc!(
            "
        You are Death.

        Every day, humans die, leaving a corpse and a soul.
        
        Your job is to collect souls, so their loved ones can bury the corpse.
        
        Let's get started, yes?"
        ),
        ..Default::default()
    };
    pub static ref HelloFromHell: EventSpec = EventSpec {
        name: "Hello from hell",
        desc: indoc!(
            "
        Hi!

        We've noticed some humans have started straying from the path of light.
        
        No biggie, just send them straight to us!
        
        Thanks,
        - Dark Lord"
        ),
        ..Default::default()
    };
}
