use yew::prelude::*;
use crate::components::text_input::TextInput;

use crate::components::custom_button::CustomButton;



#[function_component(TodoForm)]
pub fn todo_form() -> Html {
    html! {
        <form>
        <TextInput name="Todo" />
        <CustomButton label="Add Todo!!!!!!" />
        </form>
    }
}