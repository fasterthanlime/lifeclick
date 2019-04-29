#![recursion_limit = "256"]

use std::cmp;
use std::time::Duration;
use stdweb::*;
use yew::services::{IntervalService, Task};
use yew::virtual_dom::vlist::VList;
use yew::virtual_dom::vnode::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use indexmap::IndexMap;

mod units;
use units::*;

mod items;
use items::*;

mod events;
use events::*;

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

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Shop,
    Events,
    Earth,
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

    items: IndexMap<&'static ItemSpec, Item>,
    events: IndexMap<&'static EventSpec, Event>,

    tab: Tab,
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
    Purchase {
        spec: &'static ItemSpec,
        quantity: i64,
    },
    FocusTab {
        tab: Tab,
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
        let handle = interval.spawn(Duration::from_millis(150), link.send_back(|_| Msg::Tick));

        let mut model = Model {
            interval,
            job: Some(Box::new(handle)),

            due: Souls(0),
            souls: Souls(0),

            month: 0,

            // 2019 stats:
            // birth_rate: 18.5,
            // death_rate: 7.8,
            // alive: 7 * Souls::B,

            // Better starting point:
            birth_rate: 18.5,
            death_rate: 7.8,
            alive: 15 * Souls::K,

            goodness: 1.0,

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

            items: IndexMap::new(),
            events: IndexMap::new(),

            tab: Tab::Shop,
        };

        let mut add_item = |spec: &'static ItemSpec, quantity: i64| {
            let item = spec.instantiate(quantity);
            model.items.insert(item.spec, item);
        };
        add_item(&items::Sickle, 1);
        add_item(&items::Sickle2, 0);
        add_item(&items::RoboHarvest, 0);
        add_item(&items::MechaHarvest, 0);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Harvest { quantity } => {
                self.harvest(quantity);
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
            Msg::Purchase { quantity, spec } => {
                for _i in 0..quantity {
                    let item = self.items.get_mut(spec).unwrap();
                    let cost = item.cost();

                    if cost > self.souls {
                        break;
                    }
                    self.souls -= cost;
                    item.quantity += 1;
                }
                true
            }
            Msg::FocusTab { tab } => {
                self.tab = tab;
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

                self.harvest(self.souls_per_tick());

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                { self.prelude() }
                <section class="section",>
                    <div class="container",>
                        <div class="columns",>
                            <div class="column",>
                                { self.render_souls() }
                            </div>
                            <div class="column is-two-thirds",>
                                { self.render_tab_switcher() }
                                { self.render_tab_contents() }
                            </div>
                        </div>
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
            <>
                <div class="content",>
                    <p>
                        <strong>
                            { format!("{} {} ", customer.sign, customer.name) }
                        </strong>
                    </p>
                    <p>
                        { format!(" You owe {} {} souls.", customer.name, customer.owed) }
                    </p>
                </div>
                { self.render_remit_bar(kind) }
            </>
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

    fn render_souls(&self) -> Html<Self> {
        let quantity = self.souls_per_click();
        js! { document.title = @{format!("{} souls - Death Inc.", self.souls)} }
        html! {
            <>
                <h1 class="title",>{ format!("{} souls", self.souls) }</h1>
                <h2 class="subtitle",>{ format!("per month: {}", self.souls_per_tick()) }</h2>
                <div class="content",>
                    <p>
                        { format!("You have {} outstanding souls to harvest.", self.due) }
                    </p>
                    <p>
                        { format!("{} humans expire every {}.", self.deaths_per_tick(), TICK_UNIT) }
                    </p>
                    <p>
                        { format!("Each click harvests up to {} souls", self.souls_per_click()) }
                    </p>
                </div>

                <a class="button is-medium is-danger is-fullwidth", onclick=|_| Msg::Harvest{quantity},>
                    { format!("Harvest") }
                </a>
            </>
        }
    }

    fn render_tab_switcher(&self) -> Html<Self> {
        html! {
            <div class="tabs is-fullwidth",>
                <ul>
                { self.render_tab(Tab::Shop) }
                { self.render_tab(Tab::Events) }
                { self.render_tab(Tab::Earth) }
                { self.render_tab(Tab::Heaven) }
                { self.render_tab(Tab::Hell) }
                </ul>
            </div>
        }
    }

    fn render_tab_contents(&self) -> Html<Self> {
        match self.tab {
            Tab::Shop => self.render_shop(),
            Tab::Events => self.render_events(),
            Tab::Earth => self.render_earth(),
            Tab::Heaven => self.render_customer(&self.heaven),
            Tab::Hell => self.render_customer(&self.hell),
        }
    }

    fn render_events(&self) -> Html<Self> {
        if self.events.is_empty() {
            return html! {
                <div class="content",>
                    <p>
                        {"Not much going on here, earth is spinning as usual."}
                    </p>
                </div>
            };
        }

        html! {
            {for self.events.values().map(|event| {
                html! {
                    { format!("{:#?}", event) }
                }
            })}
        }
    }

    fn render_tab(&self, tab: Tab) -> Html<Self> {
        let mut class = "";
        if self.tab == tab {
            class = "is-active"
        }

        if tab == Tab::Hell && self.hell.owed.0 == 0 {
            return empty!();
        }

        html! {
            <li class=class,><a onclick=|_| Msg::FocusTab {tab},>{
                match tab {
                    Tab::Shop => html! {
                        {"Shop"}
                    },
                    Tab::Events => html! {
                        {"Events"}
                    },
                    Tab::Earth => html! {
                        {format!("Earth ({})", self.alive)}
                    },
                    Tab::Heaven => html! {
                        {format!("Heaven ({})", self.heaven.owed)}
                    },
                    Tab::Hell => html! {
                        {format!("Hell ({})", self.hell.owed)}
                    },
                }
            }</a></li>
        }
    }

    fn render_shop(&self) -> Html<Self> {
        html! {
            <>
                {for self.items.values().map(|item| {
                    self.render_item(item)
                })}
            </>
        }
    }

    fn render_item(&self, item: &Item) -> Html<Self> {
        // hide items that are too expensive
        if self.item_quantity(item.spec) == 0 && self.souls < (item.spec.initial_cost / 2) {
            return empty!();
        }

        html! {
            <div class="box",>
                <div class="subtitle",>
                    <div class="level",>
                        <div class="level-left",>
                            { item.name() }
                            { format!(" x{}", item.quantity()) }
                        </div>
                    </div>
                </div>
                <div class="content",>
                    { self.render_item_desc(item) }
                    { self.render_item_souls_per_click(item) }
                    { self.render_item_souls_per_tick(item) }
                </div>
                <div class="field has-addons",>
                    { self.render_item_purchase(item, 1) }
                    { self.render_item_purchase(item, 10) }
                    { self.render_item_purchase(item, 100) }
                </div>
            </div>
        }
    }

    fn render_item_desc(&self, item: &Item) -> Html<Self> {
        let spec = item.spec;
        if spec.desc == "" {
            return empty!();
        }

        html! {
            <p>{ spec.desc }</p>
        }
    }

    fn render_item_souls_per_click(&self, item: &Item) -> Html<Self> {
        let spec = item.spec;

        if let Some(q) = spec.souls_per_click {
            html! {
                <p>
                    { format!("Harvests {} souls / click. ", q) }
                    { if item.quantity() > 0 {
                        html! {
                            { format!("(Contributes {} SpC)", q*Souls(item.quantity())) }
                        }
                    } else { empty!() } }
                </p>
            }
        } else {
            empty!()
        }
    }

    fn render_item_souls_per_tick(&self, item: &Item) -> Html<Self> {
        let spec = item.spec;

        if let Some(q) = spec.souls_per_tick {
            html! {
                <p>
                    { format!("Harvests {} souls / {}.", q, TICK_UNIT) }
                    { if item.quantity() > 0 {
                        html! {
                            { format!("(Contributes {} SpM)", q*Souls(item.quantity())) }
                        }
                    } else { empty!() } }
                </p>
            }
        } else {
            empty!()
        }
    }

    fn render_item_purchase(&self, item: &Item, quantity: i64) -> Html<Self> {
        let spec = item.spec;
        let cost = item.cost_n(quantity);
        let disabled = cost > self.souls;
        html! {
            <p class="control is-expanded",>
                <a class="button is-danger is-fullwidth", disabled=disabled, onclick=|_| Msg::Purchase {quantity, spec},>
                    {format!("Buy {} ({} souls)", quantity, cost)}
                </a>
            </p>
        }
    }

    fn render_earth(&self) -> Html<Self> {
        html! {
            <>
                <div class="content",>
                    <p>
                        <strong>{"Earth"}</strong>
                    </p>
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
            </>
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

    fn souls_per_tick(&self) -> Souls {
        let mut total = Souls(0);
        for item in self.items.values() {
            if let Some(q) = item.spec.souls_per_tick {
                total += Souls(item.quantity) * q;
            }
        }
        total
    }

    fn souls_per_click(&self) -> Souls {
        let mut total = Souls(0);
        for item in self.items.values() {
            if let Some(q) = item.spec.souls_per_click {
                total += Souls(item.quantity) * q;
            }
        }
        total
    }

    fn item_quantity(&self, item: &ItemSpec) -> i64 {
        if let Some(item) = self.items.get(item) {
            item.quantity
        } else {
            0
        }
    }

    fn harvest(&mut self, quantity: Souls) {
        let harvested = cmp::min(self.alive, cmp::min(self.due, quantity));
        self.alive -= harvested;
        self.due -= harvested;
        self.souls += harvested;
    }
}

fn main() {
    js! { document.title = "Death Inc." }
    yew::start_app::<Model>();
}
