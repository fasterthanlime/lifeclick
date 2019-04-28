#[macro_use]
use std::cmp::{max, min};
use stdweb::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod units;
use units::*;

struct Model {
    clicked: bool,
    alive: Souls,
    due: Souls,
    souls: Souls,
}

enum Msg {
    DoIt,
    Harvest { quantity: Souls },
}

fn log(msg: &str) {
    js! { console.log(@{msg}) }
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            clicked: false,
            alive: 7 * Souls::B,
            due: 3 * Souls::K,
            souls: Souls(100),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.clicked = true;
                true
            }
            Msg::Harvest { quantity } => {
                let harvested = min(self.due, quantity);
                log(&format!("harvesting {}", harvested));
                self.due -= harvested;
                self.souls += harvested;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <p>
                { "Alive: " } { self.alive }
                { " (Birth rate: N/day)" }
            </p>
            <p>
                { "Due: " } { self.due }
            </p>
            <p>
                { "Souls: " } { self.souls }
                <button onclick=|_| Msg::Harvest{quantity: Souls(1)},>
                    { "Harvest 1" }
                </button>
            </p>

            // Render your model here
            <button onclick=|_| Msg::DoIt,>
                { if self.clicked { "Thanks!" } else { "Click me!" } }
            </button>
        }
    }
}

fn main() {
    js! { document.title = "lifeclick" }
    yew::start_app::<Model>();
}
