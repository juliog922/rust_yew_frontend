use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::rustacean_form::RustaceanForm;
use crate::hooks::use_rustacean;
use crate::Route;
use crate::components::sidebar::SideBar;
use crate::contexts::CurrentUserContext;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub rustacean_id: i32
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
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
                            <div class="col-md-10">
                                <Suspense fallback={loading}>
                                    <RustaceansEditForm 
                                        rustacean_id={props.rustacean_id}
                                        token={token.clone()}
                                    />
                                </Suspense>
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

#[derive(PartialEq, Properties)]
pub struct RustaceanEditFormProps {
    pub rustacean_id: i32,
    pub token: AttrValue
}

#[function_component(RustaceansEditForm)]
fn rustaceans_edit_form(props: &RustaceanEditFormProps) -> HtmlResult {
    let rustacean = use_rustacean(props.token.as_str(), props.rustacean_id.clone())?;

    Ok(html! {
        <RustaceanForm rustacean={rustacean} />
    })
}