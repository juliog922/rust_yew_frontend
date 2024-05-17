use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::rustaceans::api_rustacean_delete;
use crate::components::{
    alert::Alert,
    sidebar::SideBar,
    button::Button
};
use crate::Route;
use crate::contexts::CurrentUserContext;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub rustacean_id: i32
}

#[function_component(RustaceansDelete)]
pub fn rustaceans_delete(props: &Props) -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    let navigator = use_navigator()
        .expect("Navigator not available");

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();
    

    match &current_user_ctx.token {
        Some(token) => {
            let cloned_id = props.rustacean_id.clone();
            let cloned_token = token.to_owned();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                let cloned_navigator = navigator.clone();
                let cloned_error_message = error_message_handle.clone();
                let cloned_token = cloned_token.clone();
                spawn_local(async move {
                    match api_rustacean_delete(&cloned_token, cloned_id).await {
                        Ok(()) => cloned_navigator.push(&Route::Rustaceans),
                        Err(e) => cloned_error_message.set(e.to_string()),
                    }
                });
            });
                html! {
                    <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
                        <div class="row">
                            <div class="col-sm-auto">
                            <SideBar />
                            </div>
                            <div class="card shadow-lg">
                                if error_message.len() > 0 {
                                    <Alert alert_type={"danger"} message={error_message}/>
                                }
                                <p>
                                {"Are you sure you want to delete rustacean #"}
                                {props.rustacean_id.clone()}
                                </p>
                                <Button btn_type={None::<String>} class="danger" onclick={onclick} message="Delete"/>
                            </div>
                        </div>
                    </div>
                }
            },
            None => html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
}