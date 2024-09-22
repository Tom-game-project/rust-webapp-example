use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}