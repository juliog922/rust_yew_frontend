use yew::prelude::*;
use yew_router::prelude::*;

use crate::{api::crates::Crate, components::{
    crate_form::CrateForm, 
    sidebar::SideBar
}, contexts::CurrentUserContext, hooks::use_rustaceans, Route};

#[function_component(CratesAdd)]
pub fn crates_add() -> Html {
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
                        <div class="col-sm-auto offset-md-2">
                            <Suspense fallback={loading}>
                                <CrateAddForm token={token.clone()} />
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
pub struct CrateAddFormProps {
    pub token: AttrValue
}

#[function_component(CrateAddForm)]
fn crate_add_form(props: &CrateAddFormProps) -> HtmlResult {
    let rustaceans = use_rustaceans(props.token.as_str())?;

    Ok(html! {
        <CrateForm a_crate={None::<Crate>} authors={rustaceans} />
    })
}