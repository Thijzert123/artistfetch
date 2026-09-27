# Artistfetch

Simple tool to fetch artist art form TheAudioDB.

## Installation
Clone this repository and install with Cargo:
```
git clone https://github.com/Thijzert123/artistfetch.git
cd artistfetch
cargo install
```

## Usage
This is the expected library structure:
```
root
-- artist 1
   -- artist_art.jpg
   -- artist 1 songs
-- artist 2
   -- artist_art.jpg
   -- artist 2 songs
```
All directories inside `root` must be an artist. For every artist, `artistfetch` will try to donwload art.
The artist name is based on the directory name. A `artist_art.jpg` file will be created if an image is found.

To use the CLI, just pass the path to `root` as the first argument. If you want to replace all existing art files,
add `-f` or `--force` as second argument:
```
artistfetch /path/to/music/lib --force
```
