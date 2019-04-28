#![recursion_limit = "256"]

use std::cmp;
use std::time::Duration;
use stdweb::*;
use yew::services::{IntervalService, Task};
use yew::virtual_dom::vlist::VList;
use yew::virtual_dom::vnode::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod units;
use units::*;

// ok, ok, I get it
const DAYS_PER_YEAR: f64 = 365.25;
const DAYS_PER_TICK: f64 = 31.0;
const TICK_UNIT: &str = "month";

struct Customer {
    kind: CustomerKind,
    name: String,
    sign: String,
    owed: Souls,
    given: Souls,
}

#[derive(Clone, Copy)]
enum CustomerKind {
    Heaven,
    Hell,
}

macro_rules! empty {
    () => {
        VNode::from(VList::new())
    };
}

struct Model {
    #[allow(dead_code)]
    interval: IntervalService,
    #[allow(dead_code)]
    job: Option<Box<Task>>,

    alive: Souls,
    due: Souls,
    souls: Souls,

    birth_rate: f64,
    death_rate: f64,

    goodness: f64,

    month: i64,

    heaven: Customer,
    hell: Customer,
    heaven_offset: f64,
}

enum Msg {
    Tick,
    Remit {
        quantity: Souls,
        target: CustomerKind,
    },
    Harvest {
        quantity: Souls,
    },
}

#[allow(dead_code)]
fn log(msg: &str) {
    js! { console.log(@{msg}) }
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(1000), link.send_back(|_| Msg::Tick));

        Model {
            interval,
            job: Some(Box::new(handle)),

            alive: 1 * Souls::K,
            due: Souls(0),
            souls: Souls(0),

            month: 0,

            // 2019 stats:
            // birth_rate: 18.5,
            // death_rate: 7.8,

            // Better starting point:
            birth_rate: 100.0,
            death_rate: 10.0,

            goodness: 0.75,

            heaven: Customer {
                kind: CustomerKind::Heaven,
                name: "Heaven".to_owned(),
                sign: "✝️".to_owned(),
                owed: Souls(0),
                given: Souls(0),
            },
            hell: Customer {
                kind: CustomerKind::Hell,
                name: "Hell".to_owned(),
                sign: "⛧️".to_owned(),
                owed: Souls(0),
                given: Souls(0),
            },
            heaven_offset: 0.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Harvest { quantity } => {
                let harvested = cmp::min(self.alive, cmp::min(self.due, quantity));
                self.alive -= harvested;
                self.due -= harvested;
                self.souls += harvested;
                true
            }
            Msg::Remit { quantity, target } => {
                let remitted = cmp::min(self.souls, quantity);
                {
                    let cus = self.customer_mut(target);
                    cus.owed -= remitted;
                    cus.given += remitted;
                }
                self.souls -= remitted;
                true
            }
            Msg::Tick => {
                let deaths = self.deaths_per_tick();
                let heaven_float = deaths.float() * self.goodness;
                let heaven_ceil = heaven_float.ceil();
                self.heaven_offset += heaven_ceil - heaven_float;
                let mut heaven_deaths = Souls(heaven_ceil as i64);
                if self.heaven_offset >= 1.0 {
                    heaven_deaths -= Souls(self.heaven_offset as i64);
                    self.heaven_offset -= self.heaven_offset.floor();
                }
                let hell_deaths = deaths - heaven_deaths;

                self.hell.owed += hell_deaths;
                self.heaven.owed += heaven_deaths;
                self.due += deaths;

                let births = self.births_per_tick();
                self.alive += births;

                self.month += 1;

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let quantity = Souls(1);

        html! {
            <>
                { self.prelude() }
                <section class="section",>
                    <div class="container",>
                        <div class="columns",>
                            <div class="column",>
                                { self.render_customer(&self.heaven) }
                            </div>
                            <div class="column",>
                                { self.render_customer(&self.hell) }
                            </div>
                        </div>
                        <div class="box",>
                            <h1 class="title",>
                              { format!("{} souls", self.souls) }
                            </h1>

                            <div class="content",>
                                <p>
                                    { format!("You have {} outstanding souls to harvest.", self.due) }
                                </p>
                                <p>
                                    { format!("{} humans expire every {}.", self.deaths_per_tick(), TICK_UNIT) }
                                </p>
                            </div>
                            <a class="button is-primary", onclick=|_| Msg::Harvest{quantity},>
                                { format!("Harvest") }
                            </a>
                        </div>
                        { self.render_population_stats() }
                    </div>
                </section>
            </>
        }
    }
}

impl Model {
    fn customer<'a>(&'a self, kind: CustomerKind) -> &'a Customer {
        match kind {
            CustomerKind::Heaven => &self.heaven,
            CustomerKind::Hell => &self.hell,
        }
    }

    fn customer_mut<'a>(&'a mut self, kind: CustomerKind) -> &'a mut Customer {
        match kind {
            CustomerKind::Heaven => &mut self.heaven,
            CustomerKind::Hell => &mut self.hell,
        }
    }

    fn prelude(&self) -> Html<Self> {
        html! {
            <>
                <script defer=true, src="https://use.fontawesome.com/releases/v5.3.1/js/all.js",></script>
                <link rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.7.4/css/bulma.min.css",/>
            </>
        }
    }

    fn render_customer(&self, customer: &Customer) -> Html<Self> {
        let kind = customer.kind;

        html! {
            <div class="box",>
                <h3 class="title",>
                    { format!("{} {} ", customer.sign, customer.name) }
                </h3>
                <div class="content",>
                    <p>
                        { format!(" You owe {} {} souls.", customer.name, customer.owed) }
                    </p>
                    <p>
                        { format!(" You've given {} souls to {} in total.", customer.given, customer.name) }
                    </p>
                </div>
                { self.render_remit_bar(kind) }
            </div>
        }
    }

    fn render_remit_bar(&self, kind: CustomerKind) -> Html<Self> {
        let customer = self.customer(kind);
        let payable = cmp::min(self.souls, customer.owed);
        let unit_quantity = cmp::min(Souls(1), payable);
        let quart_quantity = payable / 4;
        let max_quantity = payable;

        if payable.0 == 0 {
            return html! {
                <div class="field has-addons",>
                    <p class="control is-expanded",>
                        <a class="button is-fullwidth is-static",>
                            {"Can't remit"}
                        </a>
                    </p>
                </div>
            };
        }

        html! {
            <>
                <div class="field has-addons",>
                    { self.render_remit(kind, unit_quantity) }
                    { self.render_remit(kind, quart_quantity) }
                    { self.render_remit(kind, max_quantity) }
                </div>
            </>
        }
    }

    fn render_remit(&self, kind: CustomerKind, quantity: Souls) -> Html<Self> {
        if quantity.0 == 0 {
            return empty!();
        }

        html! {
            <p class="control is-expanded",>
                <a class="button is-fullwidth", onclick=|_| Msg::Remit{quantity, target: kind},>
                    { format!("Remit {}", quantity) }
                </a>
            </p>
        }
    }

    fn render_population_stats(&self) -> Html<Self> {
        html! {
            <div class="box",>
                <h1 class="title",>{"World"}</h1>
                <div class="content",>
                    <p>
                        { format!("{:.0}% of the population is virtuous.", (self.goodness*100.0)) }
                    </p>

                    <p>
                        { format!("There are {} humans alive right now.", self.alive) }
                    </p>
                    <p>
                        { format!("{} humans are born every {}.", self.births_per_tick(), TICK_UNIT) }
                    </p>
                </div>
            </div>
        }
    }

    fn births_per_tick(&self) -> Souls {
        Souls(
            (self.alive.float() / 1000.0 * self.birth_rate / DAYS_PER_YEAR * DAYS_PER_TICK).ceil()
                as i64,
        )
    }

    fn deaths_per_tick(&self) -> Souls {
        Souls(
            (self.alive.float() / 1000.0 * self.death_rate / DAYS_PER_YEAR * DAYS_PER_TICK).ceil()
                as i64,
        )
    }
}

fn main() {
    js! { document.title = "Death Inc." }
    yew::start_app::<Model>();
}
