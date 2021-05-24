use crossterm::{cursor, queue, terminal::{self, ClearType}};

use std::{
    io::{stdout},
    fmt,
};

#[derive(Debug, Copy, Clone)]
struct Cell {
    note: Option<Note>,
}

#[derive(Debug, Copy, Clone)]
struct Note {
    key: u8,
    inst: u8,
}

struct Tracker {
    tracks: Vec<Cell>,
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self.note {
          Some(note) => write!(f, "{}", note),
          None => write!(f, "       ")
      }
  }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:0>3}", key_to_string(self.key), self.inst)
    }
}

fn key_to_string(key: u8) -> String {
    const NAMES: &'static [&'static str] = &[
        "C ",
        "C#",
        "D ",
        "D#",
        "E ",
        "F ",
        "F#",
        "G ",
        "G#",
        "A ",
        "A#",
        "B ",
    ];
    let octave = key as i32 / 12 - 1;
    format!("{}{}", NAMES[key as usize % 12], octave)
}

impl Tracker {
    fn new() -> Tracker {
        Tracker {
            tracks: vec![Cell{ note: None }; 32],
        }
    }

    fn draw(&self) -> crossterm::Result<()> {
        queue!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0)
        )?;
        for cell in &self.tracks {
            println!("{}", cell);
        }
        Ok(())
    }
}

fn main() -> crossterm::Result<()>{
    let mut tracker = Tracker::new();
    let mut i = 12;
    for cell in &mut tracker.tracks {
        cell.note = Some(Note {
            key: i,
            inst: 1,
        });
        i += 1;
    }
    tracker.draw()?;
    Ok(())
}
