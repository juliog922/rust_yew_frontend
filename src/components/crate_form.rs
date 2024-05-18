use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::*;

use crate::api::crates::{api_crate_create, api_crate_update, Crate};
use crate::api::rustaceans::Rustacean;
use crate::components::{
    input::Input,
    alert::Alert,
    button::Button,
    select::Select,
    textarea::Textarea
};
use crate::Route;
use crate::contexts::CurrentUserContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub a_crate: Option<Crate>,
    pub authors: Vec<Rustacean>
}

#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    let navigator = use_navigator()
        .expect("Navigator not available");
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    let name_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.name.clone();
        }
        String::default()
    });
    let name = (*name_handle).clone();

    let code_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.code.clone();
        }
        String::default()
    });
    let code = (*code_handle).clone();

    let version_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.version.clone();
        }
        String::default()
    });
    let version = (*version_handle).clone();

    let rustacean_id_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.rustacean_id.to_string();
        }
        String::default()
    });
    let rustacean_id = (*rustacean_id_handle).clone();

    let description_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            if let Some(description) = &c.description {
                return description.clone();
            }
            
        }
        String::default()
    });
    let description = (*description_handle).clone();

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let code_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            code_handle.set(input.value());
        }
    });

    let version_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            version_handle.set(input.value());
        }
    });

    let rustacean_id_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlSelectElement>();
        if let Some(input) = target {
            rustacean_id_handle.set(input.value());
        }
    });

    let description_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(input) = target {
            description_handle.set(input.value());
        }
    });

    let cloned_name = name.clone();
    let cloned_code = code.clone();
    let cloned_version = version.clone();
    let cloned_rustacean_id = rustacean_id.clone();
    let cloned_description = description.clone();

    let cloned_crate = props.a_crate.clone();

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_name = cloned_name.clone();
        let cloned_code = cloned_code.clone();
        let cloned_version = cloned_version.clone();
        let cloned_rustacean_id = cloned_rustacean_id.clone();
        let cloned_description = cloned_description.clone();

        let cloned_error_handle = error_message_handle.clone();

        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = current_user_ctx.clone();

        let cloned_crate = cloned_crate.clone();
        
        match &cloned_user_ctx.token {
            Some(token) => {
                let parsed_rustacean_id = cloned_rustacean_id.parse::<i32>();
                let cloned_token = token.clone();
                
                match parsed_rustacean_id {
                    Ok(rustacean_id) => spawn_local(async move {
                        if let Some(a_crate) = cloned_crate {
                            match api_crate_update(
                                &cloned_token, 
                                a_crate.id, 
                                cloned_name.clone(), 
                                cloned_code.clone(),
                                cloned_version.clone(),
                                rustacean_id,
                                cloned_description.clone()
                            ).await {
                                Ok( _ ) => cloned_navigator.push(&Route::Crates),
                                Err(e) => cloned_error_handle.set(e.to_string()),
                            }
                        } else {
                            match api_crate_create(
                                &cloned_token, 
                                cloned_name.clone(), 
                                cloned_code.clone(),
                                cloned_version.clone(),
                                rustacean_id,
                                cloned_description.clone()
                            ).await {
                                Ok( _ ) => cloned_navigator.push(&Route::Crates),
                                Err(e) => cloned_error_handle.set(e.to_string()),
                            }
                        }
                    }),
                    Err( _ ) => cloned_error_handle.set("Cannot parsed rustacean ID {}".to_string()),
                }
            },
            None => cloned_error_handle.set("Session expired. Please Login again".to_string()),
        }
        
        

    });
    let options = props.authors.iter().map(
        |r| (AttrValue::from(r.id.to_string()), AttrValue::from(r.name.clone()))
        ).collect::<Vec<(AttrValue, AttrValue)>>();
    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert alert_type={"danger"} message={error_message}/>
            }
            <div class="mb-3">
                <Input 
                    input_type="text" 
                    name="name" 
                    label="Name"
                    value={name}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input 
                    input_type="code" 
                    name="code" 
                    label="Code"
                    value={code}
                    onchange={code_changed}
                />
            </div>
            <div class="mb-3">
                <Input 
                    input_type="version" 
                    name="version" 
                    label="Version"
                    value={version}
                    onchange={version_changed}
                />
            </div>
            <div class="mb-3">
                <Select
                    name="author" 
                    label="Author"
                    value={rustacean_id}
                    onchange={rustacean_id_changed}
                    options={options}
                />
            </div>
            <div class="mb-3">
                <Textarea
                    name="description" 
                    label="Description"
                    value={description}
                    onchange={description_changed}
                />
            </div>
            <Button btn_type="submit" class="primary" onclick={None::<Callback<MouseEvent>>} message="Save"/>
        </form>
    }
}