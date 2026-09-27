use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
    thread,
    time::Duration,
};

use anyhow::Context;
use serde::Deserialize;

const ART_FILENAME: &str = "artist_art.jpg";

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let library_dir = args.get(1).context("Missing argument: library directory")?;
    let force = if let Some(flag) = args.get(2) {
        flag == "-f" || flag == "--force"
    } else {
        false
    };

    for entry in fs::read_dir(library_dir)? {
        let entry = entry?;
        let artist_dir = entry.path();
        if !artist_dir.is_dir() {
            continue;
        }

        let artist_name = entry.file_name();
        let artist_name = artist_name.to_str().with_context(|| {
            format!(
                "Failed to convert directory entry name to str: {:?}",
                artist_name
            )
        })?;

        let poster_path = artist_dir.join(ART_FILENAME);
        if poster_path.exists() {
            if force {
                fs::remove_file(poster_path)?;
            } else {
                println!("Art for {artist_name} already exists, skipping");
                continue;
            }
        }

        match process_artist(artist_name, &artist_dir) {
            Ok(artist) => println!(
                "Downloaded art for {artist_name} (MusicBrainz ID: {})",
                artist.musicbrainz_id
            ),
            Err(err) => eprintln!("Failed to download art for {artist_name} ({err:#})"),
        }
    }

    Ok(())
}

fn process_artist(artist_name: &str, artist_dir: &Path) -> anyhow::Result<AudioDBArtist> {
    let artist = AudioDBArtistSearch::search(artist_name)?;
    artist.download_art(artist_dir)?;
    Ok(artist)
}

#[derive(Deserialize)]
struct AudioDBArtistSearch {
    artists: Option<Vec<AudioDBArtist>>,
}

#[derive(Deserialize, Clone)]
struct AudioDBArtist {
    #[serde(rename = "strArtistThumb")]
    pub artist_thumb_url: String,

    #[serde(rename = "strMusicBrainzID")]
    pub musicbrainz_id: String,
}

impl AudioDBArtistSearch {
    pub fn search(artist: &str) -> anyhow::Result<AudioDBArtist> {
        let url = Self::search_url(artist);

        thread::sleep(Duration::from_millis(500)); // avoid rate-limits
        let response: Self = reqwest::blocking::get(&url)?.error_for_status()?.json()?;

        let artists = response
            .artists
            .with_context(|| format!("no artist found with url: {}", url))?;
        let artist = artists
            .first()
            .with_context(|| format!("no artist found with url: {}", url))?;
        Ok(artist.clone())
    }

    fn search_url(query: &str) -> String {
        let query_sanitized: String = query
            .chars()
            .filter_map(|c| {
                if c.is_ascii_alphanumeric() {
                    Some(c)
                } else if c == ' ' {
                    Some('+')
                } else {
                    None
                }
            })
            .collect();

        format!(
            "https://www.theaudiodb.com/api/v1/json/123/search.php?s={}",
            query_sanitized
        )
    }
}

impl AudioDBArtist {
    pub fn download_art(&self, destination_dir: &Path) -> anyhow::Result<()> {
        thread::sleep(Duration::from_millis(500)); // avoid rate-limits
        let mut response = reqwest::blocking::get(&self.artist_thumb_url)?.error_for_status()?;

        let mut file = File::create(destination_dir.join(ART_FILENAME))?;
        io::copy(&mut response, &mut file)?;
        Ok(())
    }
}
