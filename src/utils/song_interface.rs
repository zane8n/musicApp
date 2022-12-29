use crate::{model::song::{AddNewSong, Song}, utils::util::*};

use{
actix_web::HttpResponse,
actix_web::web::Json,
};


//-------listing songs

#[get("/songs")]
pub async fn list_songs() -> HttpResponse {

    // ------------Required to get all songs from DAO and have them inserted in Vec<Song>

    let songs: Vec<Song> = vec![];
Resp::Ok(songs).get_response()

}

//---------geting a song by ID
#[get("/songs/{id}")]
pub async fn get_song() -> HttpResponse {

    // ------------Required to get a song by id from DAO and have it inserted in Option<Song>


    let song: Option<Song> = None;

    match song {
        Some(song) =>  Resp::Ok(song).get_response(),
        None => Resp::Notfound(NTF::new("Song not found.".to_string())
    ).get_response(),
    }

}


//-------Adding song
#[post("/artists/{id}/songs")]
pub async fn add_song(song_req: Json<AddNewSong>) -> HttpResponse {
    
    // ------------Required to add a song from DAO and have that inserted in Vec<Song>


let song: Vec<Song> = vec![];
Resp::Created(song).get_response()
}