use yew::prelude::*;
use yew::{function_component, Properties, virtual_dom::AttrValue};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub player : AttrValue, 
}

#[function_component]
pub fn CaseComponent(props: &Props) -> Html {
    match props.player.as_str() {
        "_" => html!{<div />},
        "X" => html!{<div class="red"/>},
        "O" => html!{<div class="yellow"/>},
        default => html!{"Err"}
    }
}


