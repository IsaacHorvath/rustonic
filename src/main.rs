use yew::prelude::*;

mod data_types;
mod event_bus;
mod components;
use crate::components::*;


#[function_component(App)]
fn app() -> Html {
    
    html! {
        <>
            <header><h1>{ "Rustonic" }</h1></header>
            <main>
                <AlbumList/>
                <MainList/>
                <Queue/>
            </main>
            <ArtistTab/>
        </>
    }  
}

fn main() {
    yew::start_app::<App>();
}
