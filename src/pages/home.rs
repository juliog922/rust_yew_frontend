use yew::prelude::*;
use yew_router::prelude::*;
use chrono::NaiveDateTime;

use crate::components::sidebar::SideBar;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    match &current_user_ctx.user {
        Some(user) => {
            let date_str = match NaiveDateTime::parse_from_str(&user.created_at, "%Y-%m-%dT%T%.6f") {
                Ok(date_time) => date_time.format("%d/%m/%Y").to_string(),
                Err(_) => "Unknown".to_string(), // Handle error case gracefully
            };

            html! {
                <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
                    <div class="row">
                        <div class="col-md-8 offset-md-2">
                            <SideBar />
                            <div class="card shadow-lg">
                                <div class="card-body text-center">
                                    <h1 class="card-title display-3 fw-bold mb-4">{"Welcome, "}{&user.username}</h1>
                                    <p class="lead mb-3">{"Your ID is: "}{&user.id}</p>
                                    <p class="mb-4">{"Creation Date: "}{date_str}</p>
                                </div>
                            </div>
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