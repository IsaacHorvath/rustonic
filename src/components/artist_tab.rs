use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use crate::data_types::Artist;
use crate::event_bus::{EventBus, Request};
use std::collections::HashMap;

pub enum ArtistMsg {
    Clicked(Option<usize>),
}

pub struct ArtistTab {
    artists: HashMap<usize, Artist>,
    selected: Option<usize>,
    event_bus: Dispatcher<EventBus>,
}

impl Component for ArtistTab {
    type Message = ArtistMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            artists: demo_artists(),
            selected: None,
            event_bus: EventBus::dispatcher(),
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let artist_html: Html = self.artists.iter().map(|(id, artist)|{
            let id = id.clone();
            
            let (c, nc) = match self.selected {
                Some(i) if i==id => ("artist_selected", "aname_selected"),
                _ => ("artist", "aname")
            };
            
            let onclick = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                ArtistMsg::Clicked(Some(id))
            });
            
            html! {
                <div class={c} {onclick}>
                    <div class={nc}>{format!("{}", artist.name.clone())}</div>
                </div>
            }
        }).collect();
        
        // If the tab is clicked, but not an artist
        let onclick = ctx.link().callback(|_| {ArtistMsg::Clicked(None)});
        
        html! {
            <div class="artist_tab" {onclick}>
                {artist_html}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ArtistMsg::Clicked(selected) => {
                self.selected = selected;
                
                // Send message to album list containing a vec of ids
                let albums = match selected {
                    Some(i) => self.artists.get(&i).unwrap().get_albums(),
                    None => vec![]
                };
                self.event_bus.send(Request::EventBusMsg(albums));
                
                true
            }
        }
    }
}













fn demo_artists() -> HashMap<usize, Artist> {
    let mut artists = HashMap::new();
    artists.insert(0, Artist {
        id: 0,
        name: "Sufjan Stevens".to_string(),
        albums: vec![0,1]
    });
    artists.insert(1, Artist {
        id: 1,
        name: "Merzbow".to_string(),
        albums: vec![2]
    });
    artists.insert(2, Artist {
        id: 2,
        name: "Holly Herndon".to_string(),
        albums: vec![3]
    });
    artists.insert(3, Artist {
        id: 3,
        name: "Feist".to_string(),
        albums: vec![4]
    });
    
    artists
}
