use yew::prelude::*;

mod data_types;
mod event_bus;
mod components;
use crate::components::*;


#[function_component(App)]
fn app() -> Html {
    
    html! {
        <>
            <div class="title"><h1>{ "Rustonic" }</h1></div>
            <div class="main_body">
                <AlbumList/>
                <MainList/>
                <Queue/>
            </div>
            <ArtistTab/>
        </>
    }  
}

fn main() {
    yew::start_app::<App>();
}
