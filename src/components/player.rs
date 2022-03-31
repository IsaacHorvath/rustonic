use yew::prelude::*;
use crate::components::Button;

#[derive(Clone, Properties, PartialEq)]
pub struct PlayerProps {
    pub songtitle: String,
    pub albumtitle: String,
    pub artistname: String,
    pub playpause: Callback<MouseEvent>,
    pub deleteplaying: Callback<MouseEvent>,
}

#[function_component(Player)]
pub fn player(PlayerProps {songtitle, albumtitle, artistname, playpause, deleteplaying}: &PlayerProps) -> Html {
    html! {
        <span class="player">
            <audio id="player">
                <source src="demoaudio/01 Futile Devices.mp3" type="audio/mpeg"/>
                {"Your browser does not support html audio :("}
            </audio>
            <Button id="playpause" kind="song_button" text="▶️" onclick={playpause}/>
            <span>{songtitle}<i>{" by "}</i>{artistname}<i>{" on "}</i>{albumtitle}</span>
            <Button id="deleteplaying" kind="song_button" text="❌" onclick={deleteplaying}/>
        </span>
    }
}
