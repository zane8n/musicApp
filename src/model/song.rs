use {
    serde::{Deserialize, Serialize},
};

// -------------JSON Payloads 

#[derive(Debug,Deserialize,Serialize)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub artist_name: String,
    pub data : Vec<u8>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AddNewSong {
   pub song_req: String,
}


// -------------- DAO 

pub struct SongDAO {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub data: Vec<u8>,
}