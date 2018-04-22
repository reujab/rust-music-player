extern crate id3;
extern crate mp3_duration;
extern crate mp3_metadata;
extern crate rand;
extern crate walkdir;

use self::rand::Rng;
use self::walkdir::WalkDir;

use draw;

use std::env;
use std::time;

pub struct Playlist {
    pub index: usize,
    pub songs: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Playlist {
        // add every .mp3 in ~/Music to `mp3s`
        let mut mp3s = vec![];
        for path in WalkDir::new(env::home_dir().unwrap().to_str().unwrap().to_string() + "/Music") {
            let path = path.unwrap();
            if path.file_type().is_dir() {
                continue;
            }
            let path = path.path();
            let ext = path.extension();
            if ext.is_none() || ext.unwrap() != "mp3" {
                continue;
            }

            mp3s.push(path.to_str().unwrap().to_string())
        }
        // shuffle `mp3s`
        rand::thread_rng().shuffle(&mut mp3s);

        let mut playlist = Playlist{
            index: 0,
            songs: vec![],
        };
        for (i, path) in mp3s.iter().enumerate() {
            // parse artist and title from mp3
            let tag = id3::Tag::read_from_path(path).unwrap();

            // attempt to parse duration (faster than alternative below)
            let mut duration = mp3_duration::from_path(path);

            // if the above fails, parse the duration with another library
            if duration.is_err() {
                duration = Ok(mp3_metadata::read_from_file(path).unwrap().duration);
            }

            let song = Song{
                path: path.to_string(),
                artist: tag.artist().unwrap().to_string(),
                title: tag.title().unwrap().to_string(),
                duration: duration.unwrap(),
            };

            // update progress
            song.draw(i == playlist.index);
            draw::load_progress((i + 1) as f32 / mp3s.len() as f32);
            playlist.songs.push(song);
        }

        playlist
    }
}

pub struct Song {
    pub path: String,
    pub artist: String,
    pub title: String,
    pub duration: time::Duration,
}
