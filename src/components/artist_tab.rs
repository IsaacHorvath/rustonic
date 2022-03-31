use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::event_bus::{EventBus, Request, Response};
use std::collections::HashMap;

/*pub enum ArtistTabMsg {
    Clicked(Option<usize>),
}*/

pub struct ArtistTab {
    artists: HashMap<usize, String>,
    selected: Option<usize>,
    event_bus: Box<dyn Bridge<EventBus>>,
}

impl Component for ArtistTab {
    type Message = Response;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            artists: HashMap::new(),
            selected: None,
            event_bus: EventBus::bridge(ctx.link().callback(|r| r)),
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Collect html for each artist in the tab
        let artist_html: Html = self.artists.iter().map(|(id, name)| {
            let id = id.clone();
            
            let (c, nc) = match self.selected {
                Some(i) if i==id => ("artist_selected", "aname_selected"),
                _ => ("artist", "aname")
            };
            
            let onclick = ctx.link().callback(move |e: MouseEvent| {
                e.stop_propagation();
                Response::Clicked(Some(id))
            });
            
            html! {
                <div class={c} {onclick}>
                    <div class={nc}>{format!("{}", name.clone())}</div>
                </div>
            }
        }).collect();
        
        // If the tab is clicked, but it didn't hit an artist
        let onclick = ctx.link().callback(|_| {Response::Clicked(None)});
        
        html! {
            <div class="artist_tab" {onclick}>
                {artist_html}
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Response::Clicked(selected) => {
                self.selected = selected;
                self.event_bus.send(Request::FetchAlbums(selected));
                true
            }
            Response::ShowArtists(artists) => {
                self.artists = match artists {
                    Some(a) => a,
                    None => HashMap::new()
                };
                true
            }
            _ => false
        }
    }
    
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // If it's our first render, fetch the artist list
        if first_render {
            self.event_bus.send(Request::FetchArtists);
        }
    }
}













