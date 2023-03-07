use std::iter::Peekable;
use std::str::Chars;

mod curves;
mod elements;
mod utils;

use elements::{ PathElementCommand, PathElementLabel, PreviousElementCommand };

pub fn parse<'a>(path:&'a str) -> PathParser<'a> {
    PathParser::new(path, 64)
}

pub fn parse_with_resolution<'a>(path:&'a str, resolution:u64) -> PathParser<'a> {
    PathParser::new(path, resolution)
}

pub struct PathParser<'a> {
    data: Peekable<Chars<'a>>,
    current_command: Option<PathElementCommand>,
    previous_command: Option<PreviousElementCommand>,
    cursor: (f64, f64),
    paths: Vec<Vec<(f64, f64)>>,
    hard_ended: bool,
    resolution: u64,
}
impl<'a> PathParser<'a> {
    fn new(data:&'a str, resolution:u64) -> Self {
        Self {
            data: data.chars().peekable(),
            current_command: None,
            previous_command: None,
            cursor: (0.0, 0.0),
            paths: Vec::new(),
            hard_ended: false,
            resolution,
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
        let mut decimal_count = 0;
        while let Some(&ch) = self.data.peek() {
            if ch == '+' || ch == '-' && s.is_empty() {
                s.push(self.data.next()?);
            } else if ch == '.' && decimal_count == 0 {
                s.push(self.data.next()?);
                decimal_count = 1;  
            } else if ch.is_digit(10) {
                s.push(self.data.next()?);
            } else {
                break;
            }
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

    fn get_bool(&mut self) -> Option<bool> {
        let n = self.get_float()?; // I can't use floats in match arms
        if n == 1.0 {
            Some(true)
        } else if n == 0.0 {
            Some(false)
        } else {
            None
        }
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
        let command = match elem.label() {
            PathElementLabel::Move => self.handle_move(elem.relative()),
            PathElementLabel::Line => self.handle_line(elem.relative()),
            PathElementLabel::Horizontal => self.handle_horizontal(elem.relative()),
            PathElementLabel::Vertical => self.handle_vertical(elem.relative()),
            PathElementLabel::CubicBezier => self.handle_cubic_bezier(elem.relative()),
            PathElementLabel::SmoothCubicBezier => self.handle_smooth_cubic_bezier(elem.relative()),
            PathElementLabel::QuadraticBezier => self.handle_quadratic_bezier(elem.relative()),
            PathElementLabel::SmoothQuadraticBezier => self.handle_smooth_quadratic_bezier(elem.relative()),
            PathElementLabel::Arc => self.handle_arc(elem.relative()),
            PathElementLabel::End => self.handle_end(),
        }?;

        self.previous_command = Some(command);
        Some(self.previous_command == Some(PreviousElementCommand::End)) // Avoid more allocations
    }

    fn handle_move(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        self.cursor = self.get_point(relative)?;
        self.paths.push(vec![self.cursor]);

        Some(PreviousElementCommand::NotCurve)
    }

    fn update_paths(&mut self, end:(f64, f64)) {
        // Make sure there's an active path
        if self.paths.len() == 0 {
            self.paths.push(vec![self.cursor]);
        }

        // Make sure that the last point is pointing to the cursor
        let n = self.paths.len() - 1;
        let n2 = self.paths[n].len();
        if n2 > 0 && self.paths[n][n2 - 1] != self.cursor {
            self.paths[n].push(self.cursor);
        }

        // Update cursor
        self.cursor = end;
    }

    fn insert_points(&mut self, mut points:Vec<(f64, f64)>) {
        let end = match points.len() {
            0 => self.cursor,
            _ => points[points.len() - 1],
        };

        self.update_paths(end);
        let n = self.paths.len() - 1;

        self.paths[n].append(&mut points);        
        // Don't return anything as these are used with curves
    }

    fn insert_line(&mut self, end:(f64, f64)) -> Option<PreviousElementCommand> {
        self.update_paths(end);

        let n = self.paths.len() - 1;
        self.paths[n].push(end);

        Some(PreviousElementCommand::NotCurve)
    }

    fn handle_line(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let end = self.get_point(relative)?;
        self.insert_line(end)
    }

    fn handle_horizontal(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let y = self.get_float()?;
        let x = match relative {
            true => self.cursor.0 + self.get_float()?,
            false => self.get_float()?,
        };

        self.insert_line((x, y))
    }

    fn handle_vertical(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let x = self.get_float()?;
        let y = match relative {
            true => self.cursor.1 + self.get_float()?,
            false => self.get_float()?,
        };

        self.insert_line((x, y))
    }

    fn handle_end(&mut self) -> Option<PreviousElementCommand> {
        if self.paths.len() > 0 && self.paths[0].len() > 0 {
            self.cursor = self.paths[0][0];
        }

        Some(PreviousElementCommand::End)
    }

    fn handle_cubic_bezier(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let p1 = self.get_point(relative)?;
        let p2 = self.get_point(relative)?;
        let end = self.get_point(relative)?;

        let points = curves::compute_cubic_bezier(self.cursor, p1, p2, end, self.resolution);
        self.insert_points(points);

        Some(PreviousElementCommand::CubicBezier(p2))
    }

    fn handle_smooth_cubic_bezier(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let p2 = self.get_point(relative)?;
        let end = self.get_point(relative)?;

        let p1 = match self.previous_command {
            Some(PreviousElementCommand::CubicBezier(p1)) => utils::reflect_point(p1, self.cursor),
            _ => self.cursor,
        };

        let points = curves::compute_cubic_bezier(self.cursor, p1, p2, end, self.resolution);
        self.insert_points(points);

        Some(PreviousElementCommand::CubicBezier(p2))
    }

    fn handle_quadratic_bezier(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let p1 = self.get_point(relative)?;
        let end = self.get_point(relative)?;

        let points = curves::compute_quadratic_bezier(self.cursor, p1, end, self.resolution);
        self.insert_points(points);

        Some(PreviousElementCommand::QuadraticBezier(p1))
    }

    fn handle_smooth_quadratic_bezier(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let end = self.get_point(relative)?;
        let p1 = match self.previous_command {
            Some(PreviousElementCommand::QuadraticBezier(p1)) => utils::reflect_point(p1, self.cursor),
            _ => self.cursor,
        };

        let points = curves::compute_quadratic_bezier(self.cursor, p1, end, self.resolution);
        self.insert_points(points);

        Some(PreviousElementCommand::QuadraticBezier(p1))
    }

    fn handle_arc(&mut self, relative:bool) -> Option<PreviousElementCommand> {
        let r = self.get_point(relative)?;
        let rotation = self.get_float()?;
        let large = self.get_bool()?;
        let sweep = self.get_bool()?;
        let end = self.get_point(relative)?;

        let points = curves::compute_arc(self.cursor, r, rotation, large, sweep, end, self.resolution);
        self.insert_points(points);

        Some(PreviousElementCommand::NotCurve)
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