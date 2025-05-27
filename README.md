# Information sur une chanson : tonalité, ...

Codé avec RUST et les données sous JSON file.

## How to Run

1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Run:
   ```sh
   cargo run
   ```
4. Edit `data/songs.json` to add your own songs.

## Project Structure
- `src/main.rs` – The main app logic and entry point.
- `src/song.rs` – Song data model.
- `src/search.rs` – Search functions for songs.
- `data/songs.json` – Your song data.
