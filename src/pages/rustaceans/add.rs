use yew::prelude::*;

use crate::components::{
    rustacean_form::RustaceanForm, 
    sidebar::SideBar
};

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {
        <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
            <div class="row">
                <div class="col-sm-auto">
                <SideBar />
                </div>
                <div class="col-sm-auto offset-md-2">
                    <RustaceanForm rustacean={None} />
                </div>
            </div>
        </div>
    }
}