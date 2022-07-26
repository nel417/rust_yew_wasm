use stylist::{yew::styled_component, style};
use yew::prelude::*;

mod components;
use components::main_title::{MainTitle};
use components::todo_form::TodoForm;
use crate::components::todo_form::Data;


#[styled_component(App)]

pub fn app() -> Html {
    let stylesheet = style!(
        r#"

            * {
            font-family: Arial, Helvetica, sans-serif;
             }
            .main-container{
                height: 100%;
                width: 100%;
            }
            .center {
            position: absolute;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
            padding: 10px;
            }

        "#

    ).unwrap();




    let form_submit = Callback::from(|data: Data| {
    });


    html! {
        <div class={stylesheet}>
        <div class={"main-container"}>
            <MainTitle title="Todo List"/>
            <div class={"center"}>
            <TodoForm onsubmit={form_submit}/>
           </div>
         </div>
        </div>
    }
}