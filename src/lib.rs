use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

mod components;
use components::main_title::{MainTitle, Color};
use components::todo_form::TodoForm;

#[styled_component(App)]
pub fn app() -> Html {

    let main_title_load = Callback::from(|message: String| log!(message));
    html! {
        <div>
        <MainTitle title="Todo List" color={Color::Dark} on_load={main_title_load}/>
        <TodoForm />
        </div >
    }
}
