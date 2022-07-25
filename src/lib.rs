use stylist::yew::styled_component;
use yew::prelude::*;

mod components;
use components::main_title::{MainTitle, Color};
use components::todo_form::TodoForm;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
        <MainTitle title="Todo List" color={Color::Dark} />
        <TodoForm />
        </div >
    }
}