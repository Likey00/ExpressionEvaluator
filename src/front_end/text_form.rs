use yew::prelude::*;
use crate::front_end::text_input::TextInput;
use std::ops::Deref;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>
}

#[function_component(TextForm)]
pub fn text_form(props: &Props) -> Html {
    let state = use_state(|| String::new());

    let eq_changed = {
        let cloned_state = state.clone();
        Callback::from(move |eq| cloned_state.set(eq))
    };

    let onsubmit = {
        let form_onsubmit = props.onsubmit.clone();
        let cloned_state = state.clone();

        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            form_onsubmit.emit(cloned_state.deref().clone());
        })
    };

    html!{
        <form class="col s12" onsubmit={onsubmit}>
            <TextInput
                name="equation"
                placeholder="Enter Equation Here!" 
                handle_onchange={eq_changed} 
            />
            <button 
                class="btn waves-effect waves-light col s2 offset-s5 red lighten-2" 
                type="submit" 
                name="action"
            >{"Submit"}</button>
        </form>
    }
}