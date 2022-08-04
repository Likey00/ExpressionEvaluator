use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub placeholder: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = {
        let handle_onchange = props.handle_onchange.clone();
        
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            handle_onchange.emit(value);
        })
    };

    html!{
        <input type="text"
            class="red-text text-lighten-2"
            name={props.name.clone()} 
            onchange={onchange} 
            placeholder={props.placeholder.clone()}
        />
    }
}