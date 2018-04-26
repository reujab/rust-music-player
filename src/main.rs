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
    Draw,
    Pause,
    Prev,
    Skip,
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!("{}{}{}", screen::ToAlternateScreen, cursor::Hide, cursor::Goto(1, 1));
    stdout.flush().unwrap();

    // generate shuffled playlist
    let playlist = Arc::new(Mutex::new(Playlist::new()));
    draw::music(&playlist.lock().unwrap());

    // this variable must be created BEFORE fmod for some reason
    let winch = chan_signal::notify(&[Signal::WINCH]);
    // music control channel
    let (ctrl_tx, ctrl_rx) = mpsc::channel();

    {
        let playlist = playlist.clone();
        thread::spawn(move || play(playlist, ctrl_rx));
    }

    // redraw when the terminal window is resized
    {
        let ctrl_tx = ctrl_tx.clone();
        thread::spawn(move || {
            loop {
                winch.recv().unwrap();
                ctrl_tx.send(Command::Draw).unwrap();
            }
        });
    }

    // wait for q or ^C
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => { break }
            Key::Ctrl('c') => { break }
            Key::Char(' ') => {
                ctrl_tx.send(Command::Pause).unwrap();
            }
            Key::Up => {
                ctrl_tx.send(Command::Prev).unwrap();
            }
            Key::Down => {
                ctrl_tx.send(Command::Skip).unwrap();
            }
            _ => {}
        }
    }

    // reset terminal
    print!("{}{}", screen::ToMainScreen, cursor::Show);
}
