use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use std::collections::HashMap;
use crate::data_types::Song;
use crate::components::Button;
use crate::event_bus::{EventBus, Request, Response};

pub struct MainList {
    //TODO: wrap current songs in option
    //TODO: make list of songs a vec, in case of multiple entries in a playlist
    //test: String,
    current: HashMap<usize, Song>,
    selected: Option<usize>,
    event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for MainList {
    type Message = Response;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: HashMap::new(),
            selected: None,
            event_bus: EventBus::bridge(ctx.link().callback(|r| r)),
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.current.len() < 1 {
            return html! {
                <div class="main_list">
                    <i>{"No album or playlist selected."}</i>
                </div>
            }
        }
        let songs: Html = self.current.iter().enumerate().map(|(i, (id, song))| {
            let id = id.clone();
            
            //TODO: Send a direct event_bus callback
            let onclick = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                Response::MainListAdd(id)
            });
            html! {
                <span>
                    {format!("{}: {}", i+1, song.title)}
                    <Button kind="song_button" text="+" {onclick}/>
                    //<Button kind="song_button" text=">" {onclick}/>
                </span>
            }
        }).collect();
        
        html! {
            <div class="main_list">{songs}</div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Response::Clicked(selected) => {
                self.selected = selected;
                true
            },
            Response::ShowSongs(songs) => {
                match songs {
                    Some(s) if self.current != s => {
                        self.current = s;
                        self.selected = None;
                        true
                    },
                    None => {
                        self.current = HashMap::new();
                        self.selected = None;
                        true
                    },
                    _ => false
                }
            },
            Response::MainListAdd(song) => {
                self.event_bus.send(Request::QueueSongs(vec![song]));
                false
            },
            _ => false
        }
    }
}
