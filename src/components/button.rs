use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub btn_type: Option<String>,
    pub class: String,
    pub message: String,
    pub onclick: Option<Callback<MouseEvent>>
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let btn_type = props.btn_type.clone();
    html!{
        <button type={btn_type} class={format!("btn btn-{}", &props.class)} onclick={onclick}>
            {&props.message}
        </button>
    }
}