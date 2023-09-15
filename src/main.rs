use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World!" }</h1>
            <p>{ "This is a paragraph." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}