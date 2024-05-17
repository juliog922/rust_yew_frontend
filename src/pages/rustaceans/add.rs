use yew::prelude::*;

use crate::components::{
    sidebar::SideBar,
    rustacean_form::RustaceanForm};

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {
        <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
            <div class="row">
                <div class="col-sm-auto">
                <SideBar />
                </div>
                <div class="col-md-10">
                    <RustaceanForm />
                </div>
            </div>
        </div>
    }
}