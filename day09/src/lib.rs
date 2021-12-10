struct HeightMap {
    cells: Vec<Vec<u8>>
}

impl HeightMap {
    fn parse(input: &Vec<&str>) -> HeightMap {
        HeightMap {
            cells: input.iter()
                .map(|row| row.chars()
                    .map(|col| col.to_string().parse::<u8>().unwrap())
                    .collect()
                )
                .collect()
        }
    }

    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let num_rows = self.cells.len();
        let num_cols = self.cells[0].len();
        let max_row = num_rows - 2;
        let max_col = num_cols - 2;
        (0..num_rows).flat_map(|r| {
            (0..num_cols).filter_map(move |c| {
                let center: u8 = self.cells[r][c];
                if (r < 1 || center < self.cells[r-1][c])
                 && (r > max_row || center < self.cells[r+1][c])
                 && (c < 1 || center < self.cells[r][c-1])
                 && (c > max_col || center < self.cells[r][c+1]) {
                    Some((r, c))
                } else {
                    None
                }
            })
        }).collect()
    }

    fn sum_lowest(&self) -> i64 {
        self.find_low_points()
            .into_iter()
            .map(|(r, c)| (self.cells[r][c] + 1) as i64)
            .sum()
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let height_map = HeightMap::parse(input);
    height_map.sum_lowest()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
