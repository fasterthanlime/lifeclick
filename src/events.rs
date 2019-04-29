#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct EventSpec {
    pub name: &'static str,
    pub desc: &'static str,
}

#[derive(Debug)]
pub struct Event {
    pub spec: &'static EventSpec,
}

// event definitions

pub const HelloFromHell: EventSpec = EventSpec {
    name: "Hello from hell",
    desc: "Hi! Dark Lord here, we've noticed some humans have started straying from the path of light. No biggie, just send them straight to us.",
};
