use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,

}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {

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
        <input type="text" name={props.name.clone()} onchange={on_change} placeholder={"Todo"} />
    }
}
