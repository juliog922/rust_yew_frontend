use yew::prelude::*;
use yew_router::prelude::*;
use chrono::NaiveDateTime;

use crate::components::button::Button;
use crate::Route;
use crate::contexts::{CurrentUserContext,
     CurrentUserDispatchActions,
      CurrentUserActions};

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            let date_str = match NaiveDateTime::parse_from_str(&user.created_at, "%Y-%m-%dT%T%.6f") {
                Ok(date_time) => date_time.format("%d/%m/%Y").to_string(),
                Err(_) => "Unknown".to_string(), // Handle error case gracefully
            };
            let cloned_user_ctx = current_user_ctx.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                cloned_user_ctx.dispatch(CurrentUserDispatchActions{
                    action_type: CurrentUserActions::LoginFail,
                    login_response: None,
                    me_response: None
                });
            });
            html!{
                <div class="card shadow-lg">
                    <div class="card-body text-center">
                        <h1 class="card-title display-3 fw-bold mb-4">{"Welcome, "}{&user.username}</h1>
                        <p class="lead mb-3">{"Your ID is: "}{&user.id}</p>
                        <p class="mb-4">{"Creation Date: "}{date_str}</p>
                    </div>
                    <Button btn_type={None::<String>} class="danger" onclick={onclick} message="Logout"/>
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login}/>
        }
    }
}
