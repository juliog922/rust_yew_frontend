use yew::prelude::*;

use crate::components::{
    header::Header,
    sidebar::SideBar};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
            <div class="row">
                <SideBar />
                <div class="col-md-8 offset-md-2">  
                    <Header />
                </div>
            </div>
        </div>
    }
}
