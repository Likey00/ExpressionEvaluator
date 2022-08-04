mod front_end;
mod back_end;

use yew::prelude::*;
use std::ops::Deref;

use front_end::text_form::TextForm;
use back_end::expression_utils::evaluate_expression;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| String::new());

    let form_submit = {
        let cloned_state = state.clone();

        Callback::from(move |eq: String| {
            cloned_state.set(match evaluate_expression(&eq) {
                Ok(num) => {
                    let large_num = 1_000_000_000_000_000.0;

                    format!("The answer is {}", (num*large_num).round()/large_num)
                        .to_owned()
                },
                Err(e) => e.to_owned(),
            });
        })
    };
    
    html! {
        <div class="container">
            <div class="row">
                <h1 class="center red-text text-lighten-2">{"Expression Evaluator"}</h1>
            </div>
            <div class="row">
                <TextForm onsubmit={form_submit} />
            </div>
            <div class="divider"></div>
            <div class="row">
                <h2 class="center red-text text-lighten-2">{state.clone().deref()}</h2>
            </div>
        </div>
    }
}