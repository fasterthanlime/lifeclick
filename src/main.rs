use std::cmp::{max, min};
use std::time::Duration;
use stdweb::*;
use yew::services::{IntervalService, Task};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod units;
use units::*;

struct Customer {
    name: String,
    owed: Souls,
}

struct Model {
    interval: IntervalService,
    job: Option<Box<Task>>,

    alive: Souls,
    due: Souls,
    souls: Souls,

    birth_rate: Souls,
    death_rate: Souls,

    heaven: Customer,
    hell: Customer,
}

enum Msg {
    Tick,
    Harvest { quantity: Souls },
}

fn log(msg: &str) {
    js! { console.log(@{msg}) }
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_secs(1), link.send_back(|_| Msg::Tick));

        Model {
            interval,
            job: Some(Box::new(handle)),

            alive: 7 * Souls::B,
            due: Souls(0),
            souls: Souls(0),

            birth_rate: 360 * Souls::K,
            death_rate: 151 * Souls::K,

            heaven: Customer {
                name: "Heaven".to_owned(),
                owed: Souls(0),
            },
            hell: Customer {
                name: "Hell".to_owned(),
                owed: Souls(0),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Harvest { quantity } => {
                let harvested = min(self.due, quantity);
                self.due -= harvested;
                self.souls += harvested;
                true
            }
            Msg::Tick => {
                let deaths = self.death_rate;
                let hell_deaths = deaths / 2;
                let heaven_deaths = deaths - hell_deaths;

                self.hell.owed += hell_deaths;
                self.heaven.owed += heaven_deaths;
                self.alive -= deaths;
                self.due += deaths;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>
                    { format!("Alive: {} ", self.alive) }
                    { format!(" (Birth rate: {} / day)", self.birth_rate) }
                </p>
                <p>
                    { format!("Due: {} ", self.due) }
                    { format!(" (Death rate: {} / day)", self.death_rate) }
                </p>
                <p>
                    { format!("Souls: {} ", self.souls) }
                    <button onclick=|_| Msg::Harvest{quantity: Souls(1)},>
                        { "Harvest 1" }
                    </button>
                </p>

                { self.render_customer(&self.heaven) }
                { " | " }
                { self.render_customer(&self.hell) }
            </div>
        }
    }
}

impl Model {
    fn render_customer(&self, customer: &Customer) -> Html<Self> {
        html! {
            <>
                { format!("{} ", customer.name) }
                { format!(" You owe us {} souls", customer.owed) }
            </>
        }
    }
}

fn main() {
    js! { document.title = "lifeclick" }
    yew::start_app::<Model>();
}
