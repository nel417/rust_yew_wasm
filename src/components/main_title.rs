use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
}

#[derive(PartialEq)]
pub enum Color {
    Light,
    Dark
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Light => "light".to_owned(),
            Color::Dark => "dark".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"


            .light {
                color: black;
                background-color: white;
            }

            .dark {
                color: white;
                background-color: black;
            }
        "#

    ).unwrap();

    html!{
        <div class={stylesheet}>
                 <h1 class={&props.color.to_string()}>{&props.title}</h1>
        </div>

   }
}