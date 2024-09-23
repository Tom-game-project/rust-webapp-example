use yew::html::Scope;
use yew::prelude::*;
use gloo::console;

use crate::components::showcase::ShowcaseTable;
//

pub struct MyComponent{
    link:Scope<Self>,
    value:i32
}

pub enum Msg{
    AddOne,
    SubOne,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            value:0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
         // Create an ephemeral callback
        let incr = self.link.callback(move |_| Msg::AddOne);
        let decr = self.link.callback(move |_| Msg::SubOne);

        html! {
            <div>
                <h1 class="text-3xl font-bold underline">{"counter"}</h1>
                <button class="font-bold rounded bg-blue-500 text-white shadow-lg hover:shadow-xl hover:bg-red-500 py-2 px-4" onclick={decr}>{ "-1" }</button>
                <button class="font-bold rounded bg-blue-500 text-white shadow-lg hover:shadow-xl hover:bg-red-500 py-2 px-4" onclick={incr}>{ "+1" }</button>
                <ShowcaseTable/>
                <p>{ self.value }</p>
            </div>
        }
    }
}