use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::data_types::{Album, Song, SongList};
use crate::components::Button;
use crate::event_bus::{EventBus, Request};
use std::collections::HashMap;

pub enum AlbumMsg {
    Clicked(Option<usize>),
    Artist(Vec<usize>),
}

pub struct AlbumList {
    album_cache: HashMap<usize, Album>,
    current: Vec<usize>,
    selected: Option<usize>,
    _event: Box<dyn Bridge<EventBus>>,
}

impl Component for AlbumList {
    type Message = AlbumMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            album_cache: demo_albums(),
            current: vec![],
            selected: None,
            _event: EventBus::bridge(ctx.link().callback(AlbumMsg::Artist)),
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
        
        let albums: Html = self.current.iter().map(|id| {
            let id = id.clone();
            
            let c = match self.selected {
                Some(i) if i==id => "album_selected",
                _ => "album"
            };
            
            let onclick = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                AlbumMsg::Clicked(Some(id))
            });
            
            let title = &self.album_cache.get(&id).unwrap().title;
            
            html! {
                <span class={c} {onclick}>
                    <img src="img/albumcover.png" alt="This is an album"/>
                    <div class="album_title">{format!("{}", title)}
                    <span class="gap"/>
                    <Button kind="album_button" text="+"/>
                    <Button kind="album_button" text=">"/></div>
                </span>
            }
        }).collect();
        
        let onclick = ctx.link().callback(move |_| AlbumMsg::Clicked(None));
        
        html! {
            <div class="album_list" {onclick}>
                {albums}
            </div>
        }
    }
    
 

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AlbumMsg::Clicked(selected) => {
                self.selected = selected;
                //self.event_bus             .send(Request::EventBusMsg(selected));
                true
            }
            AlbumMsg::Artist(albums) => {
                if albums != self.current {
                    self.current = albums;
                    self.selected = None;
                    true
                } else {
                    false
                }
            }
        }
    }
}













fn demo_albums() -> HashMap<usize, Album> {
    let mut albums = HashMap::new();
    albums.insert(0, Album {
        id: 0,
        title: "Illinois".to_string(),
        artist_id: 0,
        songlist: SongList::new(
            vec![
                Song {id: 0, title: "Concerning the UFO Sighting Near Highland, Illinois".to_string()},
                Song {id: 0, title: "The Black Hawk War".to_string()},
                Song {id: 0, title: "Come On, Feel the Illinoise".to_string()}
            ])
    });
    albums.insert(1, Album {
        id: 1,
        title: "The Age of Adz".to_string(),
        artist_id: 0,
        songlist: SongList::new(
            vec![
                Song {id: 0, title: "Futile Devices".to_string()},
                Song {id: 0, title: "Too Much".to_string()},
                Song {id: 0, title: "The Age of Adz".to_string()}
            ])
    });
    albums.insert(2, Album {
        id: 2,
        title: "1930".to_string(),
        artist_id: 1,
        songlist: SongList::new(
            vec![
                Song {id: 0, title: "Intro".to_string()},
                Song {id: 0, title: "1930".to_string()},
                Song {id: 0, title: "Munchen".to_string()}
            ])
    });
    albums.insert(3, Album {
        id: 3,
        title: "Platform".to_string(),
        artist_id: 2,
        songlist: SongList::new(
            vec![
                Song {id: 0, title: "Interference".to_string()},
                Song {id: 0, title: "Chorus".to_string()},
                Song {id: 0, title: "Unequal".to_string()}
            ])
    });
    albums.insert(4, Album {
        id: 4,
        title: "Let It Die".to_string(),
        artist_id: 3,
        songlist: SongList::new(
            vec![
                Song {id: 0, title: "Gatekeeper".to_string()},
                Song {id: 0, title: "Mushaboom".to_string()},
                Song {id: 0, title: "Let It Die".to_string()}
            ])
    });
    
    albums
}

