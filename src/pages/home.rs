use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            html!{
                <div class="container">
                    { "Welcome user " }{user.username.clone()}
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login}/>
        }
    }
    
}