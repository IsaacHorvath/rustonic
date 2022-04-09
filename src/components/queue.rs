use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::data_types::Song;
use crate::components::{Button, Player};
use crate::event_bus::{EventBus, Response, Request};

pub struct Queue {
    //TODO: wrap current songs in option
    current: Vec<(usize, Song)>,
    selected: Option<usize>,
    event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for Queue {
    type Message = Response;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current: vec![],
            selected: None,
            event_bus: EventBus::bridge(ctx.link().callback(|r| r)),
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.current.len() < 1 {
            return html! {
                <div class="queue">
                    <i>{"No songs in queue."}</i>
                </div>
            }
        }
        let songs: Html = self.current.iter().skip(1).enumerate().map(|(i, (_, song))| {
            let delete = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                Response::QueueDelete(i.clone()+1)
            });
            
            html! {
            
                <span>
                    {format!("{}: {}", i+2, song.title)}
                    <Button kind="song_button" text="❌" onclick={delete}/>
                </span>
            }
        }).collect();
        
        let songtitle = self.current[0].1.title.clone();
        let path = self.current[0].1.path.clone();
        let albumtitle = self.current[0].1.album_title.clone();
        let artistname = self.current[0].1.artist_name.clone();
        
        let deleteplaying = ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            Response::QueueDelete(0)
        });
        
        html! {
            <span class="queue">
                <Player {songtitle} {path} {albumtitle} {artistname} {deleteplaying}/>
                {songs}
            </span>
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
            },
            Response::QueueDelete(song) => {
                self.current.remove(song);
                
                if song == 0 {
                    self.event_bus.send(Request::PlayPause);
                    if self.current.len() > 0 {
                        self.event_bus.send(Request::PlayerLoad(self.current[0].1.path.clone()));
                    }
                }
                
                true
            },
            _ => false
        }
    }
}
