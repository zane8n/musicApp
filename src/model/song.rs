use {
    serde::{Deserialize, Serialize},
    diesel::{ExpressionMethods, Insertable,Queryable,RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    uuid::Uuid,
   crate::schema::songs,
   crate::DBPooledConnection,
    crate::utils::util::*,
    crate::model::artist::*,
};


// -------------JSON Payloads 

#[derive(Debug,Deserialize,Serialize)]
pub struct Song {
    pub song_id: String,
    pub name: String,
    pub artist_id: String,
    pub artist_name: String,
    pub data : Vec<u8>,
}

impl Song {
    pub fn to_songdao(&self) -> SongDAO{
    SongDAO   {
        song_id : Uuid::parse_str(self.song_id.as_str()).unwrap(),
        name: self.name.to_string(),
        artist_id: Uuid::parse_str(self.song_id.as_str()).unwrap(),
        data: self.data.to_vec(),
}
}
}
#[derive(Debug,Deserialize,Serialize)]
pub struct AddNewSong {
   pub song_req: String,
}


// -------------- DAO 
#[derive(Queryable, Insertable)]
#[diesel(table_name = songs)]
pub struct SongDAO {
    pub song_id: Uuid,
    pub name: String,
    pub artist_id: Uuid,
    pub data: Vec<u8>,
}


impl SongDAO {
pub fn to_song(&self, artist_name: String) -> Song {
    Song {
        song_id : self.song_id.to_string(),
        name: self.name.to_string(),
        artist_id: self.artist_id.to_string(),
        artist_name: artist_name,
        data: self.data.to_vec()
    }

}
}

pub fn get_artist_name(_artist_id: Uuid,conn: &DBPooledConnection) -> String {
    match get_artist(_artist_id, &conn){
        Some(result) => result.artist_name,
        None => "Artist not found.".to_string(),
    }
}

pub fn get_all_songs(conn: &DBPooledConnection) -> Vec<Song>{
    use crate::schema::songs::dsl::*;
    match songs.load::<SongDAO>(conn) {
        Ok(result) => {
            result.into_iter().map(|s| {
                let artist_name = get_artist_name(s.artist_id,conn);
                s.to_song(artist_name)
            }).collect::<Vec<Song>>()
        },
        Err(_) => vec![],
    }
}


pub fn get_song(_song_id: Uuid, conn: &DBPooledConnection) -> Option<Song>{
use crate::schema::songs::dsl::*;
match songs.filter(song_id.eq(_song_id)).load::<SongDAO>(conn){
    Ok(result) => {
        match result.first(){
        Some(res) => {
            let artist_name = get_artist_name(res.artist_id, conn);
            Some(res.to_song(artist_name))
            },
        _ => None,
        }
    },
    Err(_) => None,

}
}


pub fn add_new_song(song_req: AddNewSong, _artist_id: Uuid, filename: &str, conn: &DBPooledConnection) -> Result<Song, Error>{
use crate::schema::songs::dsl::*;

let artist_name = get_artist_name(_artist_id, &conn);
let datas = rd(filename);
let new_song = Song {
    song_id: Uuid::new_v4().to_string(),
    name: song_req.song_req,
    artist_id: _artist_id.to_string(),
    artist_name: artist_name.to_string(),
    data : datas,
};
let new_song_dao = new_song.to_songdao();
match diesel::insert_into(songs).values(&new_song_dao).execute(conn){
    Ok(_) => Ok(new_song_dao.to_song(artist_name.to_string())),
    Err(e) => Err(e),
}
}