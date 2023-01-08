CREATE TABLE IF NOT EXISTS artists
(
    artist_id     UUID    PRIMARY KEY     NOT NULL,
    name   TEXT                    NOT NULL

);


CREATE TABLE IF NOT EXISTS songs 
(
    song_id      UUID    PRIMARY KEY     NOT NULL,
    name         TEXT                    NOT NULL,
    artist_id    UUID                    NOT NULL REFERENCES artists(artist_id),
    data         BYTEA                    NOT NULL
);