extern crate rfmod;

use playlist::Song;
use self::rfmod::{Sys, Sound, Channel};

pub fn play(song: &Song) -> (Sys, Sound, Channel) {
    let fmod = Sys::new().unwrap();
    if fmod.init() != rfmod::Status::Ok {
        panic!("failed to initialize fmod");
    }

    let mp3 = fmod.create_sound(&song.path, None, None).unwrap();
    let chan = mp3.play().unwrap();
    (fmod, mp3, chan)
}
