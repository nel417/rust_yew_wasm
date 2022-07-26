use stylist::yew::styled_component;
use yew::prelude::*;

mod components;
use components::main_title::{MainTitle, Color};
use components::todo_form::TodoForm;
use crate::components::todo_form::Data;

#[styled_component(App)]
pub fn app() -> Html {
    let form_submit = Callback::from(|data: Data| {

    });
    html! {
        <div>
        <MainTitle title="Todo List" color={Color::Dark} />
        <TodoForm onsubmit={form_submit}/>
        </div >
    }
}