use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod contexts;
mod pages;
mod hooks;
mod components;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/rustaceans")]
    Rustaceans,
    #[at("/rustaceans/add")]
    RustaceansAdd,
    #[at("/rustaceans/:id/edit")]
    RustaceansEdit {id: i32},
    #[at("/rustaceans/:id/delete")]
    RustaceansDelete {id: i32},
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
        Route::Rustaceans => html! { <pages::rustaceans::index::Rustaceans /> },
        Route::RustaceansAdd => html! { <pages::rustaceans::add::RustaceansAdd /> },
        Route::RustaceansEdit { id } => html! { <pages::rustaceans::edit::RustaceansEdit rustacean_id={id}/> },
        Route::RustaceansDelete { id } => html! { <pages::rustaceans::delete::RustaceansDelete rustacean_id={id}/> },
        _ => html! { <pages::home::Home /> }
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
