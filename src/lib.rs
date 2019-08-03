#![recursion_limit = "128"]

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                if self.value > 0 {
                    self.value = self.value - 1;
                    self.console.log("minus one");
                }
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <main>
                <h1>{ "Rust Counter" }</h1>
                <p>{ "Value: " }{ self.value }</p>
                <nav>
                    <button onclick=|_| Msg::Increment>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment])>{ "Increment Twice" }</button>
                </nav>
            </main>
        }
    }
}
