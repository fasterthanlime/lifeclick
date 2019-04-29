#![recursion_limit = "256"]

use std::cmp;
use std::time::Duration;
use stdweb::*;
use yew::services::{IntervalService, Task};
use yew::virtual_dom::vlist::VList;
use yew::virtual_dom::vnode::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use indexmap::IndexMap;

mod idgen;

mod units;
use units::*;

mod items;
use items::{Item, ItemCategory, ItemSpec};

mod events;
use events::{Event, EventSpec};

// ok, ok, I get it
const DAYS_PER_YEAR: f64 = 365.25;
const DAYS_PER_TICK: f64 = 31.0;
const TICK_UNIT: &str = "month";

struct Customer {
    kind: CustomerKind,
    name: String,
    sign: String,
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
    Earth,
    Heaven,
    Hell,
}

macro_rules! empty {
    () => {
        VNode::from(VList::new())
    };
}

macro_rules! delta {
    ($q:expr) => {{
        let q = $q;
        if q >= 0.into() {
            format!("+{}", q)
        } else {
            format!("{}", q)
        }
    }};
}

struct Model {
    #[allow(dead_code)]
    interval: IntervalService,
    #[allow(dead_code)]
    job: Option<Box<Task>>,

    alive: Souls,
    due: Souls,
    souls: Souls,

    base_birth_rate: f64,
    base_death_rate: f64,

    goodness: f64,

    month: i64,

    heaven: Customer,
    hell: Customer,

    items: IndexMap<&'static ItemSpec, Item>,
    events: IndexMap<&'static EventSpec, Event>,

    tab: Tab,
    item_category: ItemCategory,

    cheat: bool,
}

enum Msg {
    Tick,
    Remit {
        quantity: Souls,
        target: CustomerKind,
    },
    Harvest,
    Purchase {
        spec: &'static ItemSpec,
        quantity: i64,
    },
    FocusTab {
        tab: Tab,
    },
    FocusItemCategory {
        category: ItemCategory,
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
        let millis: u64 = if cheat_enabled() { 50 } else { 100 };
        let handle = interval.spawn(Duration::from_millis(millis), link.send_back(|_| Msg::Tick));

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
            base_birth_rate: 1.0,
            base_death_rate: 0.1,
            alive: 120 * Souls::K,

            goodness: 1.0,

            heaven: Customer {
                kind: CustomerKind::Heaven,
                name: "Heaven".to_owned(),
                sign: "✝️".to_owned(),
                given: Souls(0),
            },
            hell: Customer {
                kind: CustomerKind::Hell,
                name: "Hell".to_owned(),
                sign: "⛧️".to_owned(),
                given: Souls(0),
            },

            items: IndexMap::new(),
            events: IndexMap::new(),

            tab: Tab::Shop,
            item_category: ItemCategory::Harvest,

            cheat: cheat_enabled(),
        };

        let mut add_item = |spec: &'static ItemSpec, quantity: i64| {
            let item = spec.instantiate(quantity);
            model.items.insert(item.spec, item);
        };
        add_item(&items::Sickle, 1);
        add_item(&items::Bailiff, 0);
        add_item(&items::CollectionAgency, 0);
        add_item(&items::CollectionMultinational, 0);
        add_item(&items::SurvivalInstinct, 0);
        add_item(&items::KillerInstinct, 0);
        add_item(&items::SoulFission, 0);
        add_item(&items::PlagueSmall, 0);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Harvest => {
                self.harvest(self.souls_per_click());
                true
            }
            Msg::Remit { quantity, target } => {
                let remitted = cmp::min(self.souls, quantity);
                {
                    let cus = self.customer_mut(target);
                    cus.given += remitted;
                }
                self.souls -= remitted;
                true
            }
            Msg::Purchase { quantity, spec } => {
                for _i in 0..quantity {
                    let new_quantity = {
                        let item = self.items.get_mut(spec).unwrap();
                        let cost = item.cost();

                        if cost > self.souls {
                            break;
                        }
                        self.souls -= cost;
                        item.quantity += 1;
                        item.quantity
                    };
                    self.apply_buy_effects(spec, new_quantity);
                }
                true
            }
            Msg::FocusTab { tab } => {
                self.tab = tab;
                true
            }
            Msg::FocusItemCategory { category } => {
                self.item_category = category;
                true
            }
            Msg::Tick => {
                let deaths = self.deaths_per_tick();

                self.due += deaths;
                self.alive -= deaths;

                let births = self.births_per_tick();
                self.alive += births;

                self.month += 1;

                self.harvest(self.souls_per_tick());
                self.update_items_reveal();

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
                        { format!(" You've given {} {} souls.", customer.name, customer.given) }
                    </p>
                </div>
                { self.render_remit_bar(kind) }
            </>
        }
    }

    fn render_remit_bar(&self, kind: CustomerKind) -> Html<Self> {
        let customer = self.customer(kind);
        let payable = self.souls;
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
        js! { document.title = @{format!("{} souls - Death Inc.", self.souls)} }
        html! {
            <>
                <h1 class="title",>{ format!("{} souls", self.souls) }</h1>
                <h2 class="subtitle",>{ format!("per month: {}", self.souls_per_tick()) }</h2>
                <div class="content",>
                    { if self.cheat {
                        html! {
                            <div class="message",>
                                <div class="message-body",>
                                    {"Cheats are enabled"}
                                </div>
                            </div>
                        }
                    } else { empty!() } }
                </div>

                <a class="button is-medium is-danger is-fullwidth", onclick=|_| Msg::Harvest,>
                    { format!("Harvest {}", self.souls_per_click()) }
                </a>

                <div style="min-height: 1em",/>

                <div class="message",>
                    <div class="message-body",>
                        <p>
                            { format!("Population: {} ({} / {})", self.alive, delta!(self.births_per_tick() - self.deaths_per_tick()), TICK_UNIT) }
                        </p>
                        <p>
                            { format!("Corpses: {} ({} / {})", self.due, self.deaths_per_tick(), TICK_UNIT) }
                        </p>
                    </div>
                </div>

                { self.render_extinction() }
                { self.render_events() }
            </>
        }
    }

    fn render_extinction(&self) -> Html<Self> {
        let delta = self.births_per_tick() - self.deaths_per_tick();
        if delta < Souls(0) {
            html! {
                <div class="message is-danger",>
                    <div class="message-body",>
                        <p>
                            {"Earth population is declining"}
                        </p>
                    </div>
                </div>
            }
        } else {
            empty!()
        }
    }

    fn render_events(&self) -> Html<Self> {
        html! {
            {for self.events.values().map(|event| {
                { self.render_event(event) }
            })}
        }
    }

    fn render_event(&self, event: &Event) -> Html<Self> {
        html! {
            <div class="message is-info",>
                <div class="message-body",>
                    {format!("{:#?}", event.spec)}
                </div>
            </div>
        }
    }

    fn render_tab_switcher(&self) -> Html<Self> {
        html! {
            <div class="tabs is-fullwidth",>
                <ul>
                { self.render_tab(Tab::Shop) }
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
            Tab::Earth => self.render_earth(),
            Tab::Heaven => self.render_customer(&self.heaven),
            Tab::Hell => self.render_customer(&self.hell),
        }
    }

    fn render_tab(&self, tab: Tab) -> Html<Self> {
        let mut class = "";
        if self.tab == tab {
            class = "is-active"
        }

        html! {
            <li class=class,><a onclick=|_| Msg::FocusTab {tab},>{
                match tab {
                    Tab::Shop => html! {
                        {"Shop"}
                    },
                    Tab::Earth => html! {
                        {"Earth"}
                    },
                    Tab::Heaven => html! {
                        {"Heaven"}
                    },
                    Tab::Hell => html! {
                        {"Hell"}
                    },
                }
            }</a></li>
        }
    }

    fn render_shop_menu_category(&self, category: ItemCategory) -> Html<Self> {
        let class = if self.item_category == category {
            "is-active"
        } else {
            ""
        };

        html! {
            <li>
                <a class=class, onclick=|_| Msg::FocusItemCategory {category},>
                    { format!("{:#?}", category) }
                </a>
            </li>
        }
    }

    fn render_shop(&self) -> Html<Self> {
        html! {
            <div class="columns",>
                <div class="column is-one-quarter",>
                    <div class="menu",>
                        <ul class="menu-list",>
                            { self.render_shop_menu_category(ItemCategory::Harvest) }
                            { self.render_shop_menu_category(ItemCategory::Initiative) }
                            { self.render_shop_menu_category(ItemCategory::Event) }
                        </ul>
                    </div>
                </div>
                <div class="column",>
                    {for self.items.values().filter(|item| item.revealed && item.spec.category == self.item_category).map(|item| {
                        self.render_item(item)
                    })}
                </div>
            </div>
        }
    }

    fn render_item(&self, item: &Item) -> Html<Self> {
        html! {
            <div class="box",>
                <div class="subtitle",>
                    <div class="level",>
                        <div class="level-left",>
                            { item.name() }
                            { self.render_item_quantity(item) }
                        </div>
                    </div>
                </div>
                <div class="content",>
                    { self.render_item_desc(item) }
                    { self.render_item_souls_per_click(item) }
                    { self.render_item_souls_per_tick(item) }
                    { self.render_item_birth_rate(item) }
                    { self.render_item_death_rate(item) }
                </div>
                { self.render_item_buybar(item) }
            </div>
        }
    }

    fn render_item_quantity(&self, item: &Item) -> Html<Self> {
        if item.spec.unique {
            return empty!();
        }

        html! {
            { format!(" x{}", item.quantity()) }
        }
    }

    fn render_item_buybar(&self, item: &Item) -> Html<Self> {
        if item.spec.unique {
            if item.quantity() > 0 {
                html! {
                    <div class="field has-addons",>
                        <p class="control is-expanded",>
                            <a class="button is-static is-fullwidth",>
                                {"Bought"}
                            </a>
                        </p>
                    </div>
                }
            } else {
                html! {
                    <div class="field has-addons",>
                        { self.render_item_purchase(item, 1) }
                    </div>
                }
            }
        } else {
            html! {
                <div class="field has-addons",>
                    { self.render_item_purchase(item, 1) }
                    { self.render_item_purchase(item, 10) }
                    { self.render_item_purchase(item, 100) }
                </div>
            }
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

    fn render_item_birth_rate(&self, item: &Item) -> Html<Self> {
        let spec = item.spec;

        if let Some(q) = spec.birth_rate_modifier {
            html! {
                <p>
                    { format!("Effect: Birth rate {}%", delta!((q*100.0) as i64)) }
                </p>
            }
        } else {
            empty!()
        }
    }

    fn render_item_death_rate(&self, item: &Item) -> Html<Self> {
        let spec = item.spec;

        if let Some(q) = spec.death_rate_modifier {
            html! {
                <p>
                    { format!("Effect: Death rate {}%", delta!((q*100.0) as i64)) }
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
                        { format!("{} humans are born every {}. (Rate: {:.2} / year / 1000 population)", self.births_per_tick(), TICK_UNIT, self.effective_birth_rate()) }
                    </p>
                    <p>
                        { format!("{} humans expire every {}. (Rate {:.2} / year / 1000 population)", self.deaths_per_tick(), TICK_UNIT, self.effective_death_rate()) }
                    </p>
                </div>
            </>
        }
    }

    fn effective_birth_rate(&self) -> f64 {
        let base = self.base_birth_rate;
        let mut factor = 1.0;
        for item in self.items.values() {
            if let Some(q) = item.spec.birth_rate_modifier {
                factor += q * item.quantity() as f64;
            }
        }
        base * factor
    }

    fn effective_death_rate(&self) -> f64 {
        let base = self.base_death_rate;
        let mut factor = 1.0;
        for item in self.items.values() {
            if let Some(q) = item.spec.death_rate_modifier {
                factor += q * item.quantity() as f64;
            }
        }
        base * factor
    }

    fn births_per_tick(&self) -> Souls {
        Souls(
            (self.alive.float() / 1000.0 * self.effective_birth_rate() / DAYS_PER_YEAR
                * DAYS_PER_TICK)
                .ceil() as i64,
        )
    }

    fn deaths_per_tick(&self) -> Souls {
        Souls(
            (self.alive.float() / 1000.0 * self.effective_death_rate() / DAYS_PER_YEAR
                * DAYS_PER_TICK)
                .ceil() as i64,
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
        if self.cheat {
            return Souls::B;
        }

        let mut total = Souls(0);
        for item in self.items.values() {
            if let Some(q) = item.spec.souls_per_click {
                total += Souls(item.quantity) * q;
            }
        }
        total
    }

    #[allow(dead_code)]
    fn item_quantity(&self, item: &ItemSpec) -> i64 {
        if let Some(item) = self.items.get(item) {
            item.quantity
        } else {
            0
        }
    }

    fn harvest(&mut self, quantity: Souls) {
        let harvested = cmp::min(self.alive, cmp::min(self.due, quantity));
        self.due -= harvested;
        self.souls += harvested;
    }

    fn update_items_reveal(&mut self) {
        for item in self.items.values_mut() {
            if !item.revealed {
                item.revealed = {
                    if item.quantity() > 0 {
                        true
                    } else if self.souls >= item.spec.initial_cost / 2 {
                        true
                    } else {
                        false
                    }
                };
            }
        }
    }

    fn apply_buy_effects(&mut self, spec: &ItemSpec, new_quantity: i64) {
        if let Some(mult) = spec.pop_multiplier {
            self.alive = Souls(((self.alive.0 as f64) * mult) as i64);
        }
        if let Some(r) = spec.pop_kill_ratio {
            let deaths = Souls(((self.alive.0 as f64) * r) as i64);
            self.alive -= deaths;
            self.due += deaths;
        }

        if spec.id == items::Bailiff.id && new_quantity == 1 {
            let ev = events::Event {
                spec: &events::HelloFromHell,
            };
            self.events.insert(ev.spec, ev);
        }
    }
}

fn cheat_enabled() -> bool {
    let cheat = js! { return document.location.hash === "#cheat" };
    if let stdweb::Value::Bool(cheat) = cheat {
        cheat
    } else {
        false
    }
}

fn main() {
    js! { document.title = "Death Inc." }
    yew::start_app::<Model>();
}
