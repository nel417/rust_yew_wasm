use std::ops::Deref;
use yew::prelude::*;
use crate::components::text_input::TextInput;

use crate::components::custom_button::CustomButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[derive(Default, Clone)]
pub struct Data {
   pub todo: String,
}


#[function_component(TodoForm)]
pub fn todo_form(props: &Props) -> Html {

    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let todo_changed = Callback::from(move |todo| {
        cloned_state.set(
            Data{todo, ..cloned_state.deref().clone()}
        )
    });

    let onsubmit_clone = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();

        let data = cloned_state.deref().clone();
        onsubmit_clone.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
        <p>{"Todo: "}{&state.todo}</p>
        <TextInput name="Todo" handle_onchange={todo_changed}/>
        <CustomButton label="Add Todo!!!!!!"  />
        </form>
    }
}