use yew::prelude::*;
use stylist::{yew::styled_component, style};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,

}

#[styled_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {

    let stylesheet = style!(
        r#"
         .main-button {
          background-color: #f4511e;
          border: none;
          color: white;
          padding: 16px 48px;
          text-align: center;
          font-size: 16px;
          margin: 4px 2px;
          opacity: 0.6;
          transition: 0.3s;
          display: inline;
          text-decoration: none;
          cursor: pointer;
}

.main-button:hover {opacity: 1}


        "#

    ).unwrap();
    html! {
        <div class={stylesheet}>
        <button class={"main-button"}>{&props.label}</button>
        </div>
    }
}