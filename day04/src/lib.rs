use std::collections::HashSet;

fn parse_drawing(input: &str) -> Vec<u8> {
    input
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

struct BoardOffset {
    r: usize,
    c: usize,
}

static BOARD_SIZE: usize = 5;

struct Board {
    spaces: Vec<Vec<u8>>,
}

impl Board {
    fn is_winner(&self, marked: &HashSet<u8>, latest: u8) -> bool {
        self
            .find(latest)
            .map(|BoardOffset{r, c}| {
                self.is_row_marked(r, marked) || self.is_column_marked(c, marked)
            })
            .unwrap_or(false)
    }

    fn find(&self, value: u8) -> Option<BoardOffset> {
        self.spaces.iter()
            .map(|row| row.iter().position(|&v| v == value))
            .enumerate()
            .find_map(|(r, maybe_c)| maybe_c.map(|c| BoardOffset{r, c}))
    }

    fn is_row_marked(&self, row: usize, marked: &HashSet<u8>) -> bool {
        self.spaces[row].iter().all(|v| marked.contains(v))
    }

    fn is_column_marked(&self, col: usize, marked: &HashSet<u8>) -> bool {
        self.spaces.iter().map(|row| row[col]).all(|v| marked.contains(&v))
    }

    fn score(&self, marked: &HashSet<u8>) -> i64 {
        self.spaces.iter()
            .flat_map(|row| row.iter())
            .filter(|&v| !marked.contains(v))
            .map(|&v| v as i64)
            .sum()
    }
}

fn parse_board(lines: &[&str]) -> Board {
    let spaces = lines.iter()
        .map(|line| line
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect()
        )
        .collect::<Vec<_>>();
    Board{spaces}
}

fn parse_boards(input: &[&str]) -> Vec<Board> {
    input
        .chunks(BOARD_SIZE + 1) // skip blank line between boards
        .map(|lines| parse_board(&lines[0..BOARD_SIZE]))
        .collect()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let drawing = parse_drawing(input[0]);
    // skip blank line between moves and boards
    let boards = parse_boards(&input[2..]);
    let winner = (1..=drawing.len())
        .map(|len| &drawing[..len])
        .find_map(|drawn| {
            let marked: HashSet<u8> = drawn.iter().copied().collect();
            let last_drawn = *drawn.last().unwrap();
            boards.iter()
                .find(|b| b.is_winner(&marked, last_drawn))
                .map(|b| b.score(&marked) * last_drawn as i64)
        });
    winner.unwrap_or(0)
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
