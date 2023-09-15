use yew::prelude::*;

mod components;
mod pages;
mod api;

#[function_component(App)]
fn app() -> Html {
    html! {
        <pages::login::Login />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
