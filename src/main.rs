use crossterm::{cursor, event::{self, Event, KeyCode}, queue, style::{self, Color, Colors}, terminal::{
        self,
        ClearType,
    }};

use std::{fmt, io::{Write, stdout}};

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
    selection: usize,
    running: bool,
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self.note {
          Some(note) => write!(f, "{}", note),
          None => write!(f, ".......")
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
    fn new() -> crossterm::Result<Tracker> {
        terminal::enable_raw_mode()?;
        queue!(
            stdout(),
            terminal::EnterAlternateScreen,
            cursor::Hide
        )?;
        stdout().flush()?;
        Ok(Tracker {
            tracks: vec![Cell{ note: None }; 32],
            selection: 0,
            running: true,
        })
    }

    fn process_event(&mut self) -> crossterm::Result<()> {
        match event::read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') => self.running = false,
                    KeyCode::Up if self.selection != 0 => self.selection -= 1,
                    KeyCode::Down if self.selection != self.tracks.len() - 1 => self.selection += 1,
                    _ => {},
                }
            },
            _ => {},
        }
        Ok(())
    }

    fn draw(&self) -> crossterm::Result<()> {
        queue!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0,0)
        )?;
        for (i, cell) in self.tracks.iter().enumerate() {
            if i == self.selection {
                queue!(stdout(), style::SetColors(Colors::new(Color::White, Color::Black)))?;
            }
            write!(stdout(), "{}", cell)?;
            queue!(stdout(), style::ResetColor, cursor::MoveToNextLine(1))?;
        }
        stdout().flush()?;
        Ok(())
    }
}

impl Drop for Tracker {
    fn drop(&mut self) {
        queue!(
            stdout(),
            terminal::LeaveAlternateScreen,
            cursor::Show
        ).unwrap();
        stdout().flush().unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn main() -> crossterm::Result<()>{
    let mut tracker = Tracker::new()?;
    let mut i = 12;
    for cell in &mut tracker.tracks {
        if i % 2 == 0 {
            cell.note = Some(Note {
                key: i,
                inst: 1,
            });
        }

        i += 1;
    }
    while tracker.running {
        tracker.draw()?;
        tracker.process_event()?;
    }
    Ok(())
}
