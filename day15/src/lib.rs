use min_max_heap::MinMaxHeap;
use std::iter;

struct Board {
    risk: Vec<Vec<u32>>
}

fn new_matrix(num_rows: usize, num_cols: usize, value: u32) -> Vec<Vec<u32>> {
    iter::repeat(
        iter::repeat(value).take(num_cols).collect()
    ).take(num_rows).collect()
}

fn parse_board(input: &Vec<&str>) -> Vec<Vec<u32>> {
    input.iter().map(|line|
        line.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect()
    ).collect()
}

fn increment_vec(input: &Vec<u32>) -> Vec<u32> {
    input.iter().map(|&x| if x < 9 {x + 1} else {1}).collect()
}

fn increment_tile(input: &Vec<Vec<u32>>) ->Vec<Vec<u32>> {
    input.iter().map(increment_vec).collect()
}

fn duplicate_board(base: &Vec<Vec<u32>>, num_tiles: usize) -> Vec<Vec<u32>> {
    let base_row = base.iter().map(|row|
        std::iter::successors(Some(row.clone()), |v| Some(increment_vec(v)))
            .take(num_tiles)
            .flatten()
            .collect::<Vec<u32>>()
    )
    .collect::<Vec<Vec<u32>>>();
    std::iter::successors(Some(base_row), |r| Some(increment_tile(&r)))
        .take(num_tiles)
        .flatten()
        .collect()
}

impl Board {
    fn parse(input: &Vec<&str>, num_tiles: usize) -> Board {
        Board { risk: duplicate_board(&parse_board(input), num_tiles) }
    }

    fn num_rows(&self) -> usize {
        self.risk.len()
    }

    fn num_cols(&self) -> usize {
        self.risk[0].len()
    }

    fn neighbors(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if pos.0 > 0 {Some((pos.0 - 1, pos.1))} else {None},
            if pos.0 < self.num_rows() - 1 {Some((pos.0 + 1, pos.1))} else {None},
            if pos.1 > 0 {Some((pos.0, pos.1 - 1))} else {None},
            if pos.1 < self.num_cols() - 1 {Some((pos.0, pos.1 + 1))} else {None},
        ]
    }

    fn get_lowest_risk(&mut self, init_pos: (usize, usize), init_risk: u32) -> u32 {
        let mut total = new_matrix(self.risk.len(), self.risk[0].len(), u32::max_value());
        let mut unvisited = MinMaxHeap::new();
        unvisited.push((init_risk, init_pos));
        while let Some((risk, pos)) = unvisited.pop_min() {
            if risk < total[pos.0][pos.1] {
                total[pos.0][pos.1] = risk;
                self.neighbors(pos)
                    .iter()
                    .flatten()
                    .for_each(|&n| {
                        let new_risk = risk + self.risk[n.0][n.1];
                        if new_risk < total[n.0][n.1] {
                            unvisited.push((new_risk, n))
                        }
                    })
            }
        }
        *total.last().unwrap().last().unwrap()
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let mut board = Board::parse(input, 1);
    board.get_lowest_risk((0, 0), 0) as i64
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let mut board = Board::parse(input, 5);
    board.get_lowest_risk((0, 0), 0) as i64
}
