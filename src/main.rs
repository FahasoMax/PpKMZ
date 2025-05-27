mod song;
mod search;

use std::fs::File;
use std::io::BufReader;

fn main() {
    // Load songs from JSON file
    let file = File::open("data/songs.json").expect("Could not open songs.json");
    let reader = BufReader::new(file);
    let songs: Vec<song::Song> = serde_json::from_reader(reader).expect("Error parsing JSON");

    // Example: Search for a song by title
    let query = "Imagine";
    if let Some(found) = search::find_song_by_title(&songs, query) {
        println!("Found song: {:?}", found);
    } else {
        println!("Song not found");
    }
}