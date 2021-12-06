use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, fmt};

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Point {
    fn add(&self, offset: &Delta) -> Point {
        Point {
            x: (self.x as isize + offset.dx) as usize,
            y: (self.y as isize + offset.dy) as usize,
        }
    }
}

struct Delta {
    dx: isize,
    dy: isize,
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.dx, self.dy)
    }
}

impl Delta {
    fn sign(&self) -> Delta {
        Delta{dx: self.dx.cmp(&0) as isize, dy: self.dy.cmp(&0) as isize}
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Line {
    fn delta(&self) -> Delta {
        Delta {
            dx: self.end.x as isize - self.start.x as isize,
            dy: self.end.y as isize - self.start.y as isize,
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
}

fn parse_line(line: &str) -> Line {
    let captures = LINE_REGEX.captures(line).unwrap();
    let points = (1..=4).into_iter()
        .map(|i| captures.get(i).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    Line{
        start: Point{x: points[0], y: points[1]},
        end: Point{x: points[2], y: points[3]}
    }
    
}

struct Canvas {
    pixels: Vec<Vec<i64>>,
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pixels.iter().try_for_each(|row| {
            row.iter().try_for_each(|&col|
                if col == 0 {
                    write!(f, " .")
                } else {
                    write!(f, "{:2x}", col)
                }
            )?;
            write!(f, "\n")
        })
    }
}

impl Canvas {
    fn new() -> Canvas {
        Canvas{ pixels: Vec::new() }
    }

    fn num_rows(&self) -> usize {
        self.pixels.len()
    }

    fn ensure_rows(&mut self, num_rows: usize) {
        if num_rows > self.num_rows() {
            let zeros = vec![0 as i64; self.num_cols()];
            self.pixels.resize(num_rows, zeros)
        }
    }

    fn num_cols(&self) -> usize {
        self.pixels.first()
            .map(|row| row.len())
            .unwrap_or(0)
    }

    fn ensure_cols(&mut self, num_cols: usize) {
        if num_cols > self.num_cols() {
            self.pixels.iter_mut()
                .for_each(|row| row.resize(num_cols, 0))
        }
    }

    fn render(&mut self, line: &Line) {
        self.ensure_rows(cmp::max(line.start.y, line.end.y) + 1);
        self.ensure_cols(cmp::max(line.start.x, line.end.x) + 1);
        let delta = line.delta();
        let length = cmp::max(delta.dx.abs(), delta.dy.abs()) as usize;
        let offset = delta.sign();
        std::iter::successors(Some(line.start), |pos| Some(pos.add(&offset)))
            .take(length + 1)
            .for_each(|Point{x, y}| self.pixels[y][x] += 1)
    }

    fn render_simple(&mut self, line: &Line) {
        if line.start.x == line.end.x || line.start.y == line.end.y {
            self.render(line)
        }
    }

    fn count_line_overlaps(&self, min_overlaps: i64) -> i64 {
        self.pixels.iter()
            .flat_map(|row| row.iter())
            .filter(|&&n| n >= min_overlaps)
            .count() as i64
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let mut canvas = Canvas::new();
    input.iter()
        .map(|line| parse_line(line))
        .for_each(|line| canvas.render_simple(&line));
    canvas.count_line_overlaps(2)
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let mut canvas = Canvas::new();
    input.iter()
        .map(|line| parse_line(line))
        .for_each(|line| canvas.render(&line));
    canvas.count_line_overlaps(2)
}
