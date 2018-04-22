mod playlist;

// extern crate chan_signal;
extern crate termion;

// use chan_signal::Signal;

use playlist::Playlist;

use std::io::Write;
use std::io::{stdin, stdout};

use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen;

fn main() {
    let playlist = Playlist::new();
    for song in playlist.songs {
        println!("{} - {}", song.artist, song.title);
    }
    return;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}{}{}", screen::ToAlternateScreen, cursor::Hide, cursor::Goto(1, 1));
    stdout.flush().unwrap();

    // // redraw when the terminal window is resized
    // let signal = chan_signal::notify(&[Signal::WINCH]);
    // std::thread::spawn(move || {
    //     loop {
    //         signal.recv().unwrap();
    //         draw(playlist);
    //     }
    // });

    // wait for q or ^C
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            _ => {},
        }
    }

    // reset terminal
    print!("{}{}", screen::ToMainScreen, cursor::Show)
}
