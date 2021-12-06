use std::collections::HashSet;

fn parse_call_order(input: &str) -> Vec<u8> {
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


struct State<'a> {
    active: Vec<&'a Board>,
    scores: Vec<i64>
}

fn play(input: &Vec<&str>) -> Vec<i64> {
    let call_order = parse_call_order(input[0]);
    // skip blank line between moves and boards
    let boards = parse_boards(&input[2..]);
    let initial_state = State{active: boards.iter().collect(), scores: Vec::new()};
    let completed = (1..=call_order.len())
        .map(|len| &call_order[..len])
        .fold(initial_state, |state, called| {
            let marked: HashSet<u8> = called.iter().copied().collect();
            let last_called = *called.last().unwrap();
            let (winners, active) = state.active.iter()
                .partition(|b| b.is_winner(&marked, last_called));
            State {
                active,
                scores: state.scores.into_iter().chain(
                    winners.iter().map(|b| b.score(&marked) * last_called as i64)
                ).collect()
            }
        });
    completed.scores
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let scores = play(input);
    *scores.first().unwrap()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let scores = play(input);
    *scores.last().unwrap()
}
