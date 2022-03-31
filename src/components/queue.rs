use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use web_sys::{Element, Node};
use gloo_utils::document;
use crate::data_types::Song;
//use crate::components::Button;
use crate::event_bus::{EventBus, Response};

pub struct Queue {
    //TODO: wrap current songs in option
    current: Vec<(usize, Song)>,
    selected: Option<usize>,
    _event: Box<dyn Bridge<EventBus>>,
}

impl Component for Queue {
    type Message = Response;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: vec![],
            selected: None,
            _event: EventBus::bridge(ctx.link().callback(|r| r)),
        }
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.current.len() < 1 {
            return html! {
                <div class="queue">
                    <i>{"No songs in queue."}</i>
                </div>
            }
        }
        let songs: Html = self.current.iter().enumerate().map(|(i, (_, song))| {
            html! {
                <span>
                    {format!("{}: {}", i+1, song.title)}
                    //<Button kind="song_button" text="+"/>
                    //<Button kind="song_button" text=">"/>
                </span>
            }
        }).collect();
        
        html! {
            <div class="queue">
                <audio controls=true>
                    <source src="demoaudio/01 Futile Devices.mp3" type="audio/mpeg"/>
                    {"Your browser does not support html audio :("}
                </audio>
                {songs}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Response::Clicked(selected) => {
                self.selected = selected;
                true
            },
            Response::QueueSongs(songs) => {
                let mut s = songs.clone();
                self.current.append(&mut s);
                true
            }
            Response::QueuePlay => {
                
                //let audio: HtmlMediaElement = document().create_element("audio").unwrap().dyn_into::<HtmlMediaElement>().unwrap();
                true
            },
            _ => false
        }
    }
}
