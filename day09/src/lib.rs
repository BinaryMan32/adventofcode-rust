use std::iter;

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

    fn create_mask(&self) -> Vec<Vec<bool>> {
        let num_cols = self.cells[0].len();
        (0..self.cells.len()).map(|_r| iter::repeat(false).take(num_cols).collect()).collect()
    }

    fn find_basin_size(&self, r: usize, c: usize, mask: &mut Vec<Vec<bool>>) -> usize {
        if mask[r][c] || self.cells[r][c] >= 9 {
            0
        } else {
            mask[r][c] = true;
            1
                + (if r > 0 {self.find_basin_size(r - 1, c, mask)} else {0})
                + (if r < self.cells.len() - 1 {self.find_basin_size(r + 1, c, mask)} else {0})
                + (if c > 0 {self.find_basin_size(r, c - 1, mask)} else {0})
                + (if c < self.cells[0].len() - 1 {self.find_basin_size(r, c + 1, mask)} else {0})
        }
    }

    fn find_basin_sizes(&self) -> Vec<i64> {
        let mut mask = self.create_mask();
        self.find_low_points()
            .into_iter()
            .map(|(r, c)| self.find_basin_size(r, c, &mut mask) as i64)
            .collect()
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let height_map = HeightMap::parse(input);
    height_map.sum_lowest()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let height_map = HeightMap::parse(input);
    let mut basin_sizes = height_map.find_basin_sizes();
    basin_sizes.sort();
    println!("basin_sizes={:?}", basin_sizes);
    basin_sizes.into_iter()
        .rev()
        .take(3)
        .product()
}
