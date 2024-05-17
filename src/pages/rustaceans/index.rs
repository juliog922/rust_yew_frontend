use yew::prelude::*;

use crate::components::{
    header::Header,
    sidebar::SideBar,
    rustacean_list::RustaceanList};

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    let loading = html!{ <p>{"Loading ..."}</p> };
    html! {
        <div class="container-fluid bg-gradient d-flex align-items-center justify-content-center min-vh-100">
            <div class="row">
                <div class="col-sm-auto">
                <SideBar />
                </div>
                <div class="offset-md-2">
                    <Header />
                    <Suspense fallback={loading}>
                        <RustaceanList />
                    </Suspense>
                </div>
            </div>
        </div>
    }
}