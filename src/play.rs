extern crate rfmod;

use playlist::Playlist;
use self::rfmod::Sys;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;
use draw;
use Command;

pub fn play(playlist: Arc<Mutex<Playlist>>, ctrl_rx: mpsc::Receiver<Command>) {
    let fmod = Sys::new().unwrap();
    if fmod.init() != rfmod::Status::Ok {
        panic!("failed to initialize fmod");
    }

    loop {
        let mp3 = fmod.create_sound(&playlist.lock().unwrap().get_song().path, None, None).unwrap();
        // `chan` CANNOT be used in any other thread than the one it is created in
        // the awkward code below is designed around that
        let chan = mp3.play().unwrap();
        draw::all(&playlist.lock().unwrap(), &chan);
        loop {
            let val = ctrl_rx.recv_timeout(Duration::from_millis(50));
            match val {
                // unfortunately, polling is the only way to determine if the song has
                // finished playing
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let mut playlist = playlist.lock().unwrap();
                    let playing = chan.is_playing();
                    if playing.is_err() || !playing.unwrap() {
                        playlist.index += 1;
                        if playlist.index == playlist.songs.len() {
                            playlist.index = 0;
                        }
                        break;
                    }
                    draw::controls(&playlist, &chan);
                }
                Err(_) => {}

                Ok(Command::Draw) => {
                    draw::all(&playlist.lock().unwrap(), &chan);
                }
                Ok(Command::Play(index)) => {
                    playlist.lock().unwrap().index = index;
                    break;
                }
                Ok(Command::Pause) => {
                    chan.set_paused(!chan.get_paused().unwrap());
                }
                Ok(Command::Prev) => {
                    let mut playlist = playlist.lock().unwrap();
                    if playlist.index != 0 {
                        playlist.index -= 1;
                    }
                    break;
                }
                Ok(Command::Skip) => {
                    chan.stop();
                }
            }
        }
    }
}
