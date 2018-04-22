mod playlist;
mod draw;
mod play;

extern crate chan_signal;
extern crate termion;

use chan_signal::Signal;

use play::play;

use playlist::Playlist;

use std::io::Write;
use std::io::{stdin, stdout};

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}{}{}", screen::ToAlternateScreen, cursor::Hide, cursor::Goto(1, 1));
    stdout.flush().unwrap();

    // generate shuffled playlist
    let playlist = Playlist::new();
    draw::all(&playlist);

    // this variable must be crated BEFORE fmod for some reason
    let signal = chan_signal::notify(&[Signal::WINCH]);

    let (_fmod, _mp3, chan) = play(&playlist.songs[playlist.index]);

    // redraw when the terminal window is resized
    std::thread::spawn(move || {
        loop {
            signal.recv().unwrap();
            draw::all(&playlist);
        }
    });

    // wait for q or ^C
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            Key::Char(' ') => {
                chan.set_paused(!chan.get_paused().unwrap());
            }
            _ => {},
        }
    }

    // reset terminal
    print!("{}{}", screen::ToMainScreen, cursor::Show);
}
