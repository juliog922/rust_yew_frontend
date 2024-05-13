use yew::prelude::*;

mod api;
mod pages;
mod components;

#[function_component(App)]
fn app() -> Html {
    html!{
        <pages::login::Login />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
