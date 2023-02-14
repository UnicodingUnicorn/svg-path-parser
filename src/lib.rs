use std::iter::Peekable;
use std::str::Chars;

mod elements;
mod utils;

use elements::{ PathElementCommand, PathElementLabel };

pub fn parse<'a>(path:&'a str) -> PathParser<'a> {
    PathParser::new(path)
}

pub struct PathParser<'a> {
    data: Peekable<Chars<'a>>,
    current_command: Option<PathElementCommand>,
    cursor: (f64, f64),
    paths: Vec<Vec<(f64, f64)>>,
    hard_ended: bool,
}
impl<'a> PathParser<'a> {
    fn new(data:&'a str) -> Self {
        Self {
            data: data.chars().peekable(),
            current_command: None,
            cursor: (0.0, 0.0),
            paths: Vec::new(),
            hard_ended: false,
        }
    }

    // Keep advancing until the next character is not a separator
    fn discard_separators(&mut self) {
        while let Some(&ch) = self.data.peek() {
            if !utils::is_separator(ch) { // I can't put it up there :(
                break;
            }

            let _ = self.data.next();
        }
    }

    fn get_float(&mut self) -> Option<f64> {
        self.discard_separators();

        let mut s = String::new();
        while let Some(&ch) = self.data.peek() {
            if !utils::is_number_part(ch) {
                break;
            }

            // Should technically never be None, but best to be safe
            s.push(self.data.next()?);            
        }
        
        s.parse::<f64>().ok()
    }

    fn get_point(&mut self, relative:bool) -> Option<(f64, f64)> {
        let x = self.get_float()?;
        let y = self.get_float()?;

        Some(match relative {
            true => utils::add_point(self.cursor, (x, y)),
            false => (x, y),
        })
    }

    // Returns: (ended, result)
    fn get_path(&mut self) -> Option<(bool, Vec<(f64, f64)>)> {
        let mut ended = false;
        let mut hard_ended = false;
        while !ended && !hard_ended {
            (ended, hard_ended) = match self.advance() {
                Some(ended) => (ended, false),
                None => (false, true),
            };
        }

        self.hard_ended = hard_ended;
        self.paths.pop().map(|paths| (ended, paths))
    }

    fn advance(&mut self) -> Option<bool> {
        self.discard_separators();
        let elem = match utils::is_number_part(*self.data.peek()?) {
            true => self.current_command?.updated(),
            false => PathElementCommand::from_ch(self.data.next()?)?,
        };

        // Update memory
        self.current_command = Some(elem);

        // Parse command
        match elem.label() {
            PathElementLabel::Move => self.handle_move(elem.relative()),
            PathElementLabel::Line => self.handle_line(elem.relative()),
            PathElementLabel::Horizontal => self.handle_horizontal(elem.relative()),
            PathElementLabel::Vertical => self.handle_vertical(elem.relative()),
            PathElementLabel::End => self.handle_end(),
        }
    }

    fn handle_move(&mut self, relative:bool) -> Option<bool> {
        self.cursor = self.get_point(relative)?;
        self.paths.push(vec![self.cursor]);

        Some(false)
    }

    fn insert_line(&mut self, end:(f64, f64)) -> Option<bool> {
        if self.paths.len() == 0 {
            self.paths.push(vec![self.cursor]);
        }

        let n = self.paths.len() - 1;
        let n2 = self.paths[n].len();
        if n2 > 0 && self.paths[n][n2 - 1] != self.cursor {
            self.paths[n].push(self.cursor);
        }

        self.paths[n].push(end);
        self.cursor = end;

        Some(false)
    }

    fn handle_line(&mut self, relative:bool) -> Option<bool> {
        let end = self.get_point(relative)?;
        self.insert_line(end)
    }

    fn handle_horizontal(&mut self, relative:bool) -> Option<bool> {
        let y = self.get_float()?;
        let x = match relative {
            true => self.cursor.0 + self.get_float()?,
            false => self.get_float()?,
        };

        self.insert_line((x, y))
    }

    fn handle_vertical(&mut self, relative:bool) -> Option<bool> {
        let x = self.get_float()?;
        let y = match relative {
            true => self.cursor.1 + self.get_float()?,
            false => self.get_float()?,
        };

        self.insert_line((x, y))
    }

    fn handle_end(&mut self) -> Option<bool> {
        if self.paths.len() > 0 && self.paths[0].len() > 0 {
            self.cursor = self.paths[0][0];
        }

        Some(true)
    }
}

impl Iterator for PathParser<'_> {
    type Item = (bool, Vec<(f64, f64)>);
    // The default SVG behaviour is to ignore everything after an error is encountered
    fn next(&mut self) -> Option<Self::Item> {
        if self.hard_ended {
            self.paths.pop().map(|paths| (false, paths))
        } else {
            self.get_path()
        }
    }
}