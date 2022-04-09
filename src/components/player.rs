use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::components::Button;
use crate::event_bus::{EventBus, Response};
use web_sys::{Element, HtmlMediaElement};

#[derive(Clone, Properties, PartialEq)]
pub struct PlayerProps {
    pub songtitle: String,
    pub path: String,
    pub albumtitle: String,
    pub artistname: String,
    pub deleteplaying: Callback<MouseEvent>,
}

pub struct Player {
    audio_node: NodeRef,
    pp_node: NodeRef,
    _event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for Player {
    type Message = Response;
    type Properties = PlayerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            audio_node: NodeRef::default(),
            pp_node: NodeRef::default(),
            _event_bus: EventBus::bridge(ctx.link().callback(|r| r)),
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        let playpause = ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            Response::PlayerPlayPause
        });

        html! {
            <span class="player">
                <audio id="audio" ref={self.audio_node.clone()}>
                    <source src={props.path.clone()} type="audio/mpeg"/>
                    {"Your browser does not support html audio :("}
                </audio>
                <Button id="playpause" ref={self.pp_node.clone()} kind="song_button" text="▶️" onclick={playpause}/>
                <span>{props.songtitle.clone()}<i>{" by "}</i>{props.artistname.clone()}<i>{" on "}</i>{props.albumtitle.clone()}</span>
                <Button id="deleteplaying" kind="song_button" text="❌" onclick={props.deleteplaying.clone()}/>
            </span>
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Response::PlayerPlayPause => {
                let audio = self.audio_node.cast::<HtmlMediaElement>().unwrap();
                let button = self.pp_node.cast::<Element>().unwrap();
                
                if audio.paused() {
                    button.set_inner_html("⏸");
                    //TODO: Handle this promise
                    audio.play().unwrap();
                }
                else {
                    button.set_inner_html("▶");
                    audio.pause().unwrap();
                }
                
                true
            },
            Response::PlayerLoad(song) => {
                let audio = self.audio_node.cast::<HtmlMediaElement>().unwrap();
                audio.set_src(&song);
                
                true
            }
            _ => false
        }
    }
}
