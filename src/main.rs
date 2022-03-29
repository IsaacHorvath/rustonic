use yew::prelude::*;
use std::ops::Deref;
use std::collections::HashMap;

mod components;
use crate::components::*;

mod data_types;
use crate::data_types::*;

mod event_bus;
use crate::event_bus::*;

#[function_component(App)]
fn app() -> Html {
    
    html! {
    <>
        <div class="title"><h1>{ "Rustonic" }</h1></div>
        <div class="main_body">
            <AlbumList/>
            <MainList/>
        </div>
        <ArtistTab/>
    </>
    }  
}

fn main() {
    yew::start_app::<App>();
}
