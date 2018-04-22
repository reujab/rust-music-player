extern crate termion;

use playlist::Song;

use self::termion::color;
use self::termion::style;

impl Song {
    pub fn draw(&self, selected: bool) {
        let (width, _) = termion::terminal_size().unwrap();
        let print = format!("{} - {}", self.artist, self.title);
        let bg = color::Bg(if selected {
            color::Rgb(0, 188, 212)
        } else {
            color::Rgb(158, 158, 158)
        });

        println!("{}{}{}{}{}\r", bg, color::Fg(color::Rgb(0, 0, 0)), print, " ".repeat(width as usize - print.chars().count()), style::Reset);
    }
}
