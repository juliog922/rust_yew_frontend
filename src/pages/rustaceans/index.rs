use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::{
    header::Header,
    sidebar::SideBar,
    rustacean_list::RustaceanList};
use crate::contexts::CurrentUserContext;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    match &current_user_ctx.token {
        Some(token) => {
            let loading = html!{ <p>{"Loading ..."}</p> };
            html! {
                <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
                    <div class="row">
                        <div class="col-sm-auto">
                        <SideBar />
                        </div>
                        <div class="offset-md-2">
                            <Header />
                            <Suspense fallback={loading}>
                                <RustaceanList token={token.clone()}/>
                            </Suspense>
                        </div>
                    </div>
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login}/>
        }
    }
}