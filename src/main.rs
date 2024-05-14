use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod contexts;
mod pages;
mod components;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/rustaceans")]
    Rustaceans,
    #[at("/crates")]
    Crates,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::Login => html! { <pages::login::Login /> },
        Route::NotFound => html! { <pages::not_found::NotFound /> },
        _ => html! { <pages::login::Login /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <BrowserRouter>
            <contexts::CurrentUserProvider>
                <Switch<Route> render={switch} />
            </contexts::CurrentUserProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
