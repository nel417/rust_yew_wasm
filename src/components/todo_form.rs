use std::ops::Deref;
use yew::prelude::*;
use crate::components::text_input::TextInput;

use crate::components::custom_button::CustomButton;

// #[derive(Properties, PartialEq)]
// pub struct Props {
//     onsubmit: Callback<()>,
// }

#[derive(Default, Clone)]
struct Data {
   pub todo: String,
}


#[function_component(TodoForm)]
pub fn todo_form() -> Html {

    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let todo_changed = Callback::from(move |todo| {
        // let mut data = cloned_state.deref().clone();
        // data.todo = todo;
        // cloned_state.set(data)
        cloned_state.set(
            Data{todo, ..cloned_state.deref().clone()}
        )
    });


    html! {
        <div>
        <p>{"Todo: "}{&state.todo}</p>
        <TextInput name="Todo" handle_onchange={todo_changed}/>
        <CustomButton label="Add Todo!!!!!!"  />
        </div>
    }
}