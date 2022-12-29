use crate::{model::artist::{AddNewArtist, Artist}, utils::util::*};
use{
actix_web::HttpResponse,
actix_web::web::Json,
};


//--------listing artists
#[get("/artists")]
pub async fn list_artists() -> HttpResponse {

    // ------------Required to get all artists from DAO and have them inserted in Vec<Artist>

    let artists: Vec<Artist> = vec![];
Resp::Ok(artists).get_response()

}


//--------getting artist by ID
#[get("/artists/{id}")]
pub async fn get_artist() -> HttpResponse {

    // ------------Required to get an artist by id from DAO and have it inserted in Option<Artist>

    let artist: Option<Artist> = None;

    match artist {
        Some(artist) =>  Resp::Ok(artist).get_response(),
        None => Resp::Notfound(NTF::new("No artist with such description found".to_string())
    ).get_response(),
    }

}



//--adding artist in
#[post("/artists")]
pub async fn add_artist(artist_req: Json<AddNewArtist>) -> HttpResponse {
    
    // ------------Required to add an artist from DAO and have that inserted in Vec<Artist>

let artist: Vec<Artist> = vec![];
Resp::Created(artist).get_response()
}