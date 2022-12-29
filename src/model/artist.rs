use crate::model::song::Song;

use {
    serde::{Deserialize, Serialize},
};

// -------------JSON Payloads

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub id: String,
    pub artist_name: String,
    pub total_songs : i32,
    pub songs: Vec<Song>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AddNewArtist {
  pub artist_req: String,
}


// -------------- DAO 

pub struct ArtistDAO {
    pub id: String,
    pub artist_name: String,
}