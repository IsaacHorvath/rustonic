//use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use crate::data_types::{Artist, Album, Song};

//#[derive(Serialize, Deserialize, Debug)]
//#[derive(Clone)]
pub enum Request {
    FetchArtists,
    FetchAlbums(Option<usize>),
    FetchSongs(Option<usize>),
    QueueSongs(Vec<usize>),
    QueueAlbum(usize),
}

#[derive(Debug, Clone)]
pub enum Response {
    Clicked(Option<usize>),
    ShowArtists(Option<HashMap<usize, String>>),
    ShowAlbums(Option<HashMap<usize, String>>),
    ShowSongs(Option<HashMap<usize, Song>>),
    MainListAdd(usize),
    AlbumListAdd(usize),
    QueueSongs(Vec<(usize, Song)>),
    QueuePlay,
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
    
    artists: HashMap<usize, Artist>,
    albums: HashMap<usize, Album>,
    songs: HashMap<usize, Song>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            artists: demo_artists(),
            albums: demo_albums(),
            songs: demo_songs(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        // TODO: Only send to relevant subscribers! I'm not sure how to tell which are which from the HandlerId
        let response = match msg {
            Request::FetchArtists => {
                let artists: HashMap<usize, String> = self.artists
                                                          .iter()
                                                          .map(|(id, a)|
                                                              (*id, a.name.clone()))
                                                          .collect();
                Response::ShowArtists(Some(artists))
            },
            Request::FetchAlbums(Some(artist_id)) => {
                let albums: HashMap<usize, String> = self.artists
                                                         .get(&artist_id)
                                                         .unwrap()
                                                         .get_albums()
                                                         .iter()
                                                         .map(|id|
                                                             (*id, self.albums.get(id).unwrap().title.clone()))
                                                         .collect();
                Response::ShowAlbums(Some(albums))
            },
            Request::FetchSongs(Some(album_id)) => {
                let songs: HashMap<usize, Song> = self.albums
                                                      .get(&album_id)
                                                      .unwrap()
                                                      .get_songs()
                                                      .iter()
                                                      .map(|id| 
                                                          (*id, self.songs.get(id).unwrap().clone()))
                                                      .collect();
                Response::ShowSongs(Some(songs))
            },
            Request::FetchAlbums(None) => Response::ShowAlbums(None),
            Request::FetchSongs(None) => Response::ShowSongs(None),
            Request::QueueSongs(songs) => {
                let songs: Vec<(usize, Song)> = songs.iter().map(|id| (*id, self.songs.get(id).unwrap().clone())).collect();
                Response::QueueSongs(songs)
            }
            Request::QueueAlbum(album_id) => {
                let songs: Vec<(usize, Song)> = self.albums.get(&album_id).unwrap().get_songs().iter().map(|id| (*id, self.songs.get(id).unwrap().clone())).collect();
                Response::QueueSongs(songs)
            }
        };
        
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, response.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}



fn demo_artists() -> HashMap<usize, Artist> {
    let mut artists = HashMap::new();
    artists.insert(0, Artist {
        name: "Sufjan Stevens".to_string(),
        albums: vec![0,1]
    });
    artists.insert(1, Artist {
        name: "Merzbow".to_string(),
        albums: vec![2]
    });
    artists.insert(2, Artist {
        name: "Holly Herndon".to_string(),
        albums: vec![3]
    });
    artists.insert(3, Artist {
        name: "Feist".to_string(),
        albums: vec![4]
    });
    
    artists
}

fn demo_albums() -> HashMap<usize, Album> {
    let mut albums = HashMap::new();
    albums.insert(0, Album {
        title: "Illinois".to_string(),
        artist_id: 0,
        songs: vec![0,1,2]
    });
    albums.insert(1, Album {
        title: "The Age of Adz".to_string(),
        artist_id: 0,
        songs: vec![3,4,5]
    });
    albums.insert(2, Album {
        title: "1930".to_string(),
        artist_id: 1,
        songs: vec![6,7,8]
    });
    albums.insert(3, Album {
        title: "Platform".to_string(),
        artist_id: 2,
        songs: vec![9,10,11]
    });
    albums.insert(4, Album {
        title: "Let It Die".to_string(),
        artist_id: 3,
        songs: vec![12,13,14]
    });
    
    albums
}

fn demo_songs() -> HashMap<usize, Song> {
    let mut songs = HashMap::new();
    songs.insert(0, Song {
                        title: "Concerning the UFO Sighting Near Highland, Illinois".to_string(),
                        album_id: 0,
                        album_title: "Illinois".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(1, Song {
                        title: "The Black Hawk War".to_string(),
                        album_id: 0,
                        album_title: "Illinois".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(2, Song {
                        title: "Come On, Feel the Illinoise".to_string(),
                        album_id: 0,
                        album_title: "Illinois".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(3, Song {
                        title: "Futile Devices".to_string(),
                        album_id: 0,
                        album_title: "The Age of Adz".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(4, Song {
                        title: "Too Much".to_string(),
                        album_id: 0,
                        album_title: "The Age of Adz".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(5, Song {
                        title: "The Age of Adz".to_string(),
                        album_id: 0,
                        album_title: "The Age of Adz".to_string(),
                        artist_id: 0,
                        artist_name: "Sufjan Stevens".to_string()
                    });
    songs.insert(6, Song {
                        title: "Intro".to_string(),
                        album_id: 0,
                        album_title: "1930".to_string(),
                        artist_id: 0,
                        artist_name: "Merzbow".to_string()
                    });
    songs.insert(7, Song {
                        title: "1930".to_string(),
                        album_id: 0,
                        album_title: "1930".to_string(),
                        artist_id: 0,
                        artist_name: "Merzbow".to_string()
                    });
    songs.insert(8, Song {
                        title: "Munchen".to_string(),
                        album_id: 0,
                        album_title: "1930".to_string(),
                        artist_id: 0,
                        artist_name: "Merzbow".to_string()
                    });
    songs.insert(9, Song {
                        title: "Interference".to_string(),
                        album_id: 0,
                        album_title: "Platform".to_string(),
                        artist_id: 0,
                        artist_name: "Holly Herndon".to_string()
                    });
    songs.insert(10, Song {
                        title: "Chorus".to_string(),
                        album_id: 0,
                        album_title: "Platform".to_string(),
                        artist_id: 0,
                        artist_name: "Holly Herndon".to_string()
                    });
    songs.insert(11, Song {
                        title: "Unequal".to_string(),
                        album_id: 0,
                        album_title: "Platform".to_string(),
                        artist_id: 0,
                        artist_name: "Holly Herndon".to_string()
                    });
    songs.insert(12, Song {
                        title: "Gatekeeper".to_string(),
                        album_id: 0,
                        album_title: "Let It Die".to_string(),
                        artist_id: 0,
                        artist_name: "Feist".to_string()
                    });
    songs.insert(13, Song {
                        title: "Mushaboom".to_string(),
                        album_id: 0,
                        album_title: "Let It Die".to_string(),
                        artist_id: 0,
                        artist_name: "Feist".to_string()
                    });
    songs.insert(14, Song {
                        title: "Let It Die".to_string(),
                        album_id: 0,
                        album_title: "Let It Die".to_string(),
                        artist_id: 0,
                        artist_name: "Feist".to_string()
                    });
    songs
}


