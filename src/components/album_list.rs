use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::components::Button;
use crate::event_bus::{EventBus, Request, Response};
use std::collections::HashMap;

pub struct AlbumList {
    //TODO: wrap current albums in option
    current: HashMap<usize, String>,
    selected: Option<usize>,
    event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for AlbumList {
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
                <div class="album_list">
                    <i class="album">{"No artist selected."}</i>
                </div>
            }
        }
        
        let albums: Html = self.current.iter().map(|(id, title)| {
            let id = id.clone();
            
            let c = match self.selected {
                Some(i) if i==id => "album_selected",
                _ => "album"
            };
            
            //TODO: Can we make these callbacks go directly to the event bus?
            let onclick = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                Response::Clicked(Some(id))
            });
            
            let onclick_add = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                Response::AlbumListAdd(id)
            });
            
            html! {
                <span class={c} {onclick}>
                    <img src="img/albumcoversmall.png" alt="This is an album"/>
                    <div class="album_title">{format!("{}", title)}
                        <span class="gap"/>
                        <Button kind="album_button" text="+" onclick={onclick_add}/>
                        //<Button kind="album_button" text=">"/>
                    </div>
                </span>
            }
        }).collect();
        
        let onclick = ctx.link().callback(move |_| Response::Clicked(None));
        
        html! {
            <div class="album_list" {onclick}>
                {albums}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Response::Clicked(selected) => {
                self.selected = selected;
                self.event_bus.send(Request::FetchSongs(selected));
                true
            },
            Response::ShowAlbums(albums) => {
                match albums {
                    Some(a) if self.current != a => {
                        self.current = a;
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
            Response::AlbumListAdd(album) => {
                self.event_bus.send(Request::QueueAlbum(album));
                false
            }
            _ => false
        }
    }
}
