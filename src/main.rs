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
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen;

pub enum Command {
    Pause,
    Skip,
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}{}{}", screen::ToAlternateScreen, cursor::Hide, cursor::Goto(1, 1));
    stdout.flush().unwrap();

    // generate shuffled playlist
    let playlist = Arc::new(Mutex::new(Playlist::new()));
    draw::all(&playlist.lock().unwrap());

    // this variable must be crated BEFORE fmod for some reason
    let winch = chan_signal::notify(&[Signal::WINCH]);

    // redraw when the terminal window is resized
    {
        let playlist = playlist.clone();
        thread::spawn(move || {
            loop {
                winch.recv().unwrap();
                draw::all(&playlist.lock().unwrap());
            }
        });
    }

    // music control channel
    let (ctrl_tx, ctrl_rx) = mpsc::channel();
    play(playlist.clone(), ctrl_rx);

    // wait for q or ^C
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            Key::Char(' ') => {
                ctrl_tx.send(Command::Pause).unwrap();
            },
            Key::Down => {
                ctrl_tx.send(Command::Skip).unwrap();
            },
            _ => {},
        }
    }

    // reset terminal
    print!("{}{}", screen::ToMainScreen, cursor::Show);
}
