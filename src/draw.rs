extern crate rfmod;
extern crate termion;

use playlist::Playlist;
use playlist::Song;
use self::termion::clear;
use self::termion::color;
use self::termion::cursor;
use self::termion::style;
use std::io::Write;
use std::io::stdout;

// gets terminal size
fn size() -> (usize, usize) {
    let (width, height) = termion::terminal_size().unwrap();
    (width as usize, height as usize)
}

impl Playlist {
    pub fn get_displayed_songs(&self) -> &[Song] {
        let (_, height) = size();
        let begin = if self.index < 3 || self.songs.len() <= height-1 {
            0
        } else if self.songs[self.index..].len() < height - 3 {
            self.songs.len() - height + 1
        } else {
            self.index - 3
        };
        if begin + height - 1 > self.songs.len() {
            &self.songs[begin..self.songs.len()]
        } else {
            &self.songs[begin..begin+height-1]
        }
    }
}

fn bar(width: usize, percentage: f32) -> String {
    let bar_width = width as f32 * percentage;
    let full_blocks = bar_width as usize;
    // round down to nearest eight
    let eighth = ((bar_width - bar_width.floor()) * 8.0).floor() / 8.0;

    const BLOCKS: &[&str] = &["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
    let bar = "█".repeat(full_blocks) + BLOCKS[(eighth * 8.0) as usize];
    let empty = " ".repeat(width - bar.chars().count());

    format!("{}{}{}{}{}", color::Bg(color::White), color::Fg(color::Rgb(244, 67, 54)), bar, empty, style::Reset)
}

// clears the screen and draws everything
pub fn all(playlist: &Playlist, chan: &rfmod::Channel) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush().unwrap();
    music(playlist);
    controls(playlist, chan);
}

// draws every displayable song
pub fn music(playlist: &Playlist) {
    for song in playlist.get_displayed_songs() {
        song.draw(song.index == playlist.get_song().index);
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
    let percent = format!(" {:3}%", (progress * 100.0) as usize);
    let bar = bar(width - percent.len(), progress);

    print!("{save}{goto}{bar}{percent}{restore}", save=cursor::Save, goto=cursor::Goto(1, height as u16), bar=bar, percent=percent, restore=cursor::Restore);
    stdout().flush().unwrap();
}

pub fn controls(playlist: &Playlist, chan: &rfmod::Channel) {
    let (width, height) = size();
    let ms_played = chan.get_position(rfmod::TIMEUNIT_MS).unwrap();
    let secs_total = playlist.get_song().duration.as_secs();
    let mut progress = ms_played as f32 / 1000.0 / secs_total as f32;
    if progress > 1.0 {
        progress = 1.0;
    }

    let controls = if chan.get_paused().unwrap() {
        "▶"
    } else {
        "⏸"
    }.to_owned() + " ";
    let timestamp = format!(" [{:02}:{:02}/{:02}:{:02}]", ms_played / 1000 / 60, ms_played / 1000 % 60, secs_total / 60, secs_total % 60);
    let bar = bar(width - controls.chars().count() - timestamp.len(), progress);

    print!("{goto}{controls}{bar}{timestamp}", goto=cursor::Goto(1, height as u16), controls=controls, bar=bar, timestamp=timestamp);
    stdout().flush().unwrap();
}
