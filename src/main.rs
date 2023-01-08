#[macro_use]
extern crate actix_web;
extern crate diesel;

use{
    actix_web::{middleware,App, HttpServer},
    actix_web::web::Data,
    diesel::r2d2::ConnectionManager,
    diesel::PgConnection,
    r2d2::{Pool, PooledConnection},
    std::{env,io},
};
use crate::{utils::artist_interface, utils::song_interface};
pub(crate) mod schema;
mod model {
    pub(crate) mod song;
    pub(crate) mod artist;
}
mod utils {
   pub(crate) mod artist_interface;
   pub(crate) mod song_interface;
    pub(crate) mod util;
}

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_to_pool(pool: Data<DBPool>) -> DBPooledConnection {
    pool.get().expect("Failled to reach DB connection pool.")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(song_interface::list_songs)
        .service(song_interface::get_song)
        .service(song_interface::add_song)
        .service(artist_interface::list_artists)
        .service(artist_interface::get_artist)
        .service(artist_interface::add_artist)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}