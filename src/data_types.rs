// TODO: Wrap ids in newtypes?

#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub title: String,
    pub path: String,
    pub album_id: usize,
    pub album_title: String,
    pub artist_id: usize,
    pub artist_name: String,
}

#[derive(Clone, PartialEq)]
pub struct Album {
    pub title: String,
    pub artist_id: usize,
    pub songs: Vec<usize>
}

impl Album {
    pub fn get_songs(&self) -> Vec<usize> {
        self.songs.clone()
    }
}

#[derive(Clone, PartialEq)]
pub struct Artist {
    pub name: String,
    pub albums: Vec<usize>
}

impl Artist {
    pub fn get_albums(&self) -> Vec<usize> {
        self.albums.clone()
    }
}
