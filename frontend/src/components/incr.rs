use yew::prelude::*;
// debug tools
use gloo::console;

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
    SubOne
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                console::log!("+1!");
            }
            Msg::SubOne => {
                self.value -= 1;
                console::log!("-1!");
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1 class="text-3xl font-bold underline">{"counter"}</h1>
                <button class="font-bold rounded bg-blue-500 text-white shadow-lg hover:shadow-xl hover:bg-red-500 py-2 px-4" onclick={self.link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                <button class="font-bold rounded bg-blue-500 text-white shadow-lg hover:shadow-xl hover:bg-red-500 py-2 px-4" onclick={self.link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
