use gloo::console::log;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
   pub name: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let on_change = Callback::from(|event: Event| {
        let target = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>();
        log!(target.value())
    });
    html! {
        <input type="text" name={props.name.clone()} onchange={on_change} />
    }
}
