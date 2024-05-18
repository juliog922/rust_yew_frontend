use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::crate_form::CrateForm;
use crate::hooks::{use_crate, use_rustaceans};
use crate::Route;
use crate::components::sidebar::SideBar;
use crate::contexts::CurrentUserContext;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub crate_id: i32
}

#[function_component(CratesEdit)]
pub fn crates_edit(props: &Props) -> Html {
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
                                    <CratesEditForm 
                                        crate_id={props.crate_id}
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
pub struct CrateEditFormProps {
    pub crate_id: i32,
    pub token: AttrValue
}

#[function_component(CratesEditForm)]
fn rustaceans_edit_form(props: &CrateEditFormProps) -> HtmlResult {
    let a_crate = use_crate(props.token.as_str(), props.crate_id.clone())?;
    let rustaceans = use_rustaceans(props.token.as_str())?; 

    Ok(html! {
        <CrateForm a_crate={a_crate} authors={rustaceans}/>
    })
}