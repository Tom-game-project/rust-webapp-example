use yew::{function_component, html, Html, Properties};
use yew::classes;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub title: String,
    pub contents: String,
    pub brief: String
}

#[function_component]
fn TransactionShowcase(props: &Props) -> Html {
    html! {
        <div class={classes!("p-6", "flex", "justify-between")}>
            <h2 class={classes!("text-lg", "font-semibold", "mb-2")}>
                { &props.title }
            </h2>
            <div class={classes!("flex", "space-x-6", "text-gray-600")}>
                <p><strong>{"brief:"}</strong> { &props.brief }</p>
            </div>
        </div>
    }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
pub fn ShowcaseTable() -> Html {
    let lst = vec!["hello", "world" , "my"];
    // Request::get();
    html! {
        <div class={classes!("bg-white", "shadow-md", "rounded-lg", "divide-y", "divide-gray-300")}>
        {
            lst.iter().map(
                |inner| html!{
                    <TransactionShowcase 
                        title={inner.clone()}
                        contents={inner.clone()}
                        brief={inner.clone()}
                    />
                }
            ).collect::<Html>()
        }
        </div>
    }
}
