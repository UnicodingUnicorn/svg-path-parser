# SVG Path Parser

A really un-opinionated library for reading SVG paths. So un-opinionated, in fact, that you're just returned lists of points and whether the path is closed or not. If the path is closed, just assume there's a line between the last point and the first point.

## Usage

First of all, extract the path string from the `d` tag. I dunno, use regex or something, it's a free world. Next, feed it into the parser:

```rust
let paths = svg_path_parser::parse(&path).collect::<Vec<(bool, Vec<(f64, f64)>)>>();
```

The `bool` indicates whether the path is closed and the `Vec` is a vector of all the points. Treat it as a continuous connect the dots thing.

### Creating lines from a list of points

I get that a list of points is not very helpful.

```rust
struct Line {
    start: (f64, f64),
    end: (f64, f64),
}
impl Line {
    pub fn new(start:(f64, f64), end:(f64, f64)) -> Self {
        Self { start, end, }
    }
}

fn to_lines((close, path):(bool, Vec<(f64, f64)>)) -> Vec<Line> {
    let mut lines = path.iter()
        .zip(path.iter().skip(1))
        .map(|(start, end)| Line::new(*start, *end))
        .collect::<Vec<Line>>();
    
    if close && lines.len() > 0 {
        let &end = lines[lines.len() - 1];
        let &start = lines[0]

        if start.start != end.end {
            lines.push(Line::new(end.end, start.start));
        }
    }

    lines
}
```