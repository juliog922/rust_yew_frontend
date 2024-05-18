use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub options: Vec<(AttrValue, AttrValue)>,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    let value = props.value.clone();
    
    html!{
        <>
            <label for={html_id.clone()}>{props.label.clone()}</label>
            <select
                id={html_id}
                class="form-control"
                name={props.name.clone()}
                onchange={props.onchange.clone()}
            >
                <option value="" selected={value.is_empty()}>{"Select an option"}</option>
                {
                    props.options.clone().into_iter().map(|(val, label)| {
                        let selected = *val == value;
                        html! {
                            <option value={val.clone()} selected={selected}>{label.clone()}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </>
    }
}