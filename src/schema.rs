// @generated automatically by Diesel CLI.

diesel::table! {
    artists (artist_id) {
        artist_id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    songs (song_id) {
        song_id -> Uuid,
        name -> Text,
        artist_id -> Uuid,
        data -> Bytea,
    }
}

diesel::joinable!(songs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    songs,
);
