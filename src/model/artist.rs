use crate::model::song::Song;

use {
    serde::{Deserialize, Serialize},
    diesel::{ExpressionMethods, Insertable,Queryable,RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
   crate::DBPooledConnection,
    uuid::Uuid,
   crate::schema::artists,
   crate::model::song::*,
    diesel::dsl::sql,


    
};



// -------------JSON Payloads

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub artist_id: String,
    pub artist_name: String,
    pub total_songs : i32,
    pub songs: Vec<Song>,
}

impl Artist {
    pub fn to_artistdao(&self) -> ArtistDAO {
        ArtistDAO{
            artist_id: Uuid::parse_str(self.artist_id.as_str()).unwrap(),
            name: self.artist_name.to_string(),
        }

    }
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AddNewArtist {
  pub artist_name: String,
}


// -------------- DAO 
#[derive(Queryable, Insertable)]
#[diesel(table_name = artists)]
pub struct ArtistDAO {
    pub artist_id: Uuid,
    pub name: String,
}

impl ArtistDAO{
    pub fn to_artist(&self, songs: Vec<Song> ) -> Artist {
        Artist{
            artist_id: self.artist_id.to_string(),
            artist_name: self.name.to_string(),
            total_songs: songs.len() as i32,
            songs,
        }
    }
}
pub fn get_songs(_artistdao: &ArtistDAO, conn: &DBPooledConnection) -> Vec<Song> {

    use crate::schema::songs::dsl::*;
    match songs.filter(artist_id.eq(_artistdao.artist_id)).load::<SongDAO>(conn) {
        Ok(result) => result.into_iter()
                                .map(|s| s.to_song(_artistdao.name.clone()))
                                .collect::<Vec<Song>>(),
        Err(_) => vec![],

    }
}

pub fn get_artist(_artist_id: Uuid, conn: &DBPooledConnection) -> Option<Artist> {
    use crate::schema::artists::dsl::*;
    match artists.filter(artist_id.eq(_artist_id)).load::<ArtistDAO>(conn) {
        Ok(result) => {
            match result.first() {
                Some(res) => {
                    let songs = get_songs(&res, conn);
                    Some(res.to_artist(songs))
                },
                _ => None,
            }
        },
        Err(_) => None,
    }
}

pub fn get_all_artists(conn: &DBPooledConnection) -> Vec<Artist> {

    use crate::schema::artists::dsl::*;
    match artists.load::<ArtistDAO>(conn){
        Ok(result) => {
            result.into_iter().map(|a| {
                let songs = get_songs(&a, conn);
                a.to_artist(songs)

            }).collect::<Vec<Artist>>()
        },
        Err(_) => vec![],
    }
}


pub fn add_new_artist(artist_req: AddNewArtist, conn: &DBPooledConnection) -> Result<Artist, Error>{
    use crate::schema::artists::dsl::*;
    let new_artist = Artist {
    artist_id: Uuid::new_v4().to_string(),
    artist_name: artist_req.artist_name.to_string(),
    total_songs : 0,
    songs: vec![],
    };
    let new_artist_dao = new_artist.to_artistdao();
    match diesel::insert_into(artists).values(&new_artist_dao).execute(conn){
        Ok(_) => Ok(new_artist_dao.to_artist(vec![])),
        Err(e) => Err(e),
    }
}
