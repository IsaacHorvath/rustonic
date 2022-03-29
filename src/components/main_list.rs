use yew::prelude::*;
use crate::data_types::SongList;
use crate::components::Button;

#[function_component(MainList)]
pub fn main_list() -> Html {
    /*if songlist.len() < 1 {
        return html! {
            <div class="main_list">
                <i>{"No album or playlist selected."}</i>
            </div>
        }
    }*/
    /*let songs: Html = songlist.get_songs().iter().enumerate().map(|(i, song)| {
        html! {
            <span>
                {format!("{}: {}", i+1, song.title)}
                <Button kind="song_button" text="+"/>
                <Button kind="song_button" text=">"/>
            </span>
        }
    }).collect();*/
    let songs = "no songs today!";
    
    html! {
        <div class="main_list">{songs}</div>
    }
}
