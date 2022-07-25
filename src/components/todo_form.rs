use gloo::console::log;
use yew::prelude::*;
use crate::components::text_input::TextInput;

use crate::components::custom_button::CustomButton;



#[function_component(TodoForm)]
pub fn todo_form() -> Html {





    let todo_state = use_state(|| "no todo set".to_owned());
    let cloned_todo_state = todo_state.clone();
    let todo_changed = Callback::from(move |todo| {
        cloned_todo_state.set(todo)
    });









    html! {
        <form>
        <TextInput name="Todo" handle_onchange={todo_changed}/>
        <CustomButton label="Add Todo!!!!!!" />
        <p>{"Todo: "}{&*todo_state}</p>
        </form>
    }
}