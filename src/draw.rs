extern crate termion;

use playlist::Playlist;
use playlist::Song;

use std::io::Write;
use std::io::stdout;

use self::termion::color;
use self::termion::clear;
use self::termion::cursor;
use self::termion::style;

fn size() -> (usize, usize) {
    let (width, height) = termion::terminal_size().unwrap();
    (width as usize, height as usize)
}

pub fn all(playlist: &Playlist) {
    let (_, height) = size();
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush().unwrap();
    for i in 0..height-1 {
        if i < playlist.songs.len() {
            playlist.songs[i].draw(i == playlist.index);
        }
    }
}

impl Song {
    pub fn draw(&self, selected: bool) {
        let (width, _) = size();
        let title = format!("{} — {}", self.artist, self.title);
        let duration = format!("{}:{:02}", self.duration.as_secs() / 60, self.duration.as_secs() % 60);

        if selected {
            print!("{}{}", color::Bg(color::Rgb(0, 188, 212)), style::Bold);
        } else {
            print!("{}", color::Bg(color::Rgb(224, 224, 224)))
        }

        println!("{fg}{title}{spaces}{duration}{reset}\r", fg=color::Fg(color::Rgb(0, 0, 0)), title=title, spaces=" ".repeat(width - title.chars().count() - duration.len()), duration=duration, reset=style::Reset);
    }
}

pub fn load_progress(progress: f32) {
    let (width, height) = size();
    let bar = "#".repeat((progress * (width as f32 - 7.0)) as usize);
    let percent = ((progress * 100.0) as usize).to_string();

    print!("{save}{goto}[{bar}{empty}]{spaces}{percent}%{restore}", save=cursor::Save, goto=cursor::Goto(1, height as u16), bar=bar, empty=" ".repeat(width - bar.len() - 7), spaces=" ".repeat(4 - percent.len()), percent=percent, restore=cursor::Restore);
    stdout().flush().unwrap();
}
