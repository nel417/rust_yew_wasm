use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use stylist::{yew::styled_component, style};



#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,

}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {

    let stylesheet = style!(
        r#"
            .main-input {
             width: 100%;
              padding: 12px 20px;
              margin: 8px 0;
              box-sizing: border-box;
            }
        "#

    ).unwrap();

    let handle_onchange = props.handle_onchange.clone();

    let on_change = Callback::from(move |event: Event| {
        let target = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(target);
    });


    html! {
        <div class={stylesheet}>
        <input class={"main-input"} type="text" name={props.name.clone()} onchange={on_change} placeholder={"Todo"} />
        </div>
    }
}
