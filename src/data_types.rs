// TODO: Wrap ids in newtypes?

#[derive(Clone, PartialEq)]
pub struct Song {
    pub id: usize,
    pub title: String
}

#[derive(Clone, PartialEq)]
pub struct SongList {
    songs: Vec<Song>
}

impl SongList {
    pub fn new(songs: Vec<Song>) -> Self {
        Self { songs }
    }
    
    pub fn len(&self) -> usize {
        self.songs.len()
    }

    pub fn get_songs(&self) -> Vec<Song> {
        self.songs.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: usize,
    pub title: String,
    pub artist_id: usize,
    pub songlist: SongList
}

impl Album {
    pub fn song_list(&self) -> SongList {
        self.songlist.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct Artist {
    pub id: usize,
    pub name: String,
    pub albums: Vec<usize>
}

impl Artist {
    pub fn get_albums(&self) -> Vec<usize> {
        self.albums.clone()
    }
}
