use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use stdweb::*;

struct Model {
    console: ConsoleService,
    clicked: bool
}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            clicked: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.clicked = true;
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
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