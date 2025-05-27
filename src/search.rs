use crate::song::Song;

pub fn find_song_by_title<'a>(songs: &'a [Song], title: &str) -> Option<&'a Song> {
    songs.iter().find(|song| song.title.to_lowercase() == title.to_lowercase())
}