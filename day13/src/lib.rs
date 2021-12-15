use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

struct Fold {
    axis: char,
    pos: u16
}

impl Fold {
    fn run(&self, p: &Point) -> Point {
        match self.axis {
            'x' if p.x > self.pos => Point {x: 2 * self.pos - p.x, y: p.y},
            'y' if p.y > self.pos => Point {x: p.x, y: 2 * self.pos - p.y},
            _ => *p
        }
    }
}

lazy_static! {
    static ref FOLD_REGEX: Regex = Regex::new(r"fold along ([xy])=(\d+)").unwrap();
}

fn parse_input(input: &Vec<&str>) -> (HashSet<Point>, Vec<Fold>) {
    let mut remainder = input.iter();
    let points = remainder
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split(",")
                .map(|p| p.parse::<u16>().unwrap())
                .collect_tuple::<(u16, u16)>()
                .unwrap();
            Point{x, y}
        })
        .collect();
    let folds = remainder
        .map(|line| {
            let captures = FOLD_REGEX.captures(line).unwrap();
            Fold{
                axis: captures.get(1).unwrap().as_str().chars().next().unwrap(),
                pos: captures.get(2).unwrap().as_str().parse::<u16>().unwrap()
            }
        })
        .collect();
    (points, folds)
}

fn fold_points(fold: &Fold, points: &HashSet<Point>) -> HashSet<Point> {
    points.iter()
        .map(|p| fold.run(p))
        .collect()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let (points, folds) = parse_input(input);
    fold_points(&folds[0], &points).len() as i64
}

fn render(points: &HashSet<Point>) -> String {
    let num_rows = points.iter().map(|p| p.y).max().unwrap() + 1;
    let num_cols = points.iter().map(|p| p.x).max().unwrap() + 1;
    (0..num_rows).map(|y|
        (0..num_cols).map(|x|
            if points.contains(&Point{x, y}) {'#'} else {'.'}
        ).collect::<String>()
    ).join("\n")
}

pub fn part2(input: &Vec<&str>) -> String {
    let (points, folds) = parse_input(input);
    let final_points = folds.iter()
        .fold(points, |points, fold| fold_points(fold, &points));
    let answer = render(&final_points);
    println!("{}", answer);
    answer
}
