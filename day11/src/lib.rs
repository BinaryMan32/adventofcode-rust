struct Cavern {
    cells: Vec<Vec<u8>>,
    num_flashes: i64,
}

impl Cavern {
    fn parse(lines: &Vec<&str>) -> Cavern {
        Cavern {
            cells: lines.iter().map(|line|
                line.chars().map(|c|
                    c.to_string().parse::<u8>().unwrap()
                ).collect::<Vec<_>>()
            ).collect::<Vec<_>>(),
            num_flashes: 0,
        }
    }

    fn num_rows(&self) -> usize { self.cells.len() }
    fn num_cols(&self) -> usize { self.cells[0].len() }
    fn num_cells(&self) -> usize { self.num_rows() * self.num_cols() }

    fn step(&mut self) -> usize {
        (0..self.num_rows()).for_each(|r|
            (0..self.num_cols()).for_each(|c| {
                self.inc_flash_reset(r, c)
            })
        );
        self.cells.iter_mut()
            .flat_map(|row| row.iter_mut())
            .filter(|&&mut cell| cell > 9)
            .map(|cell| {
                *cell = 0;
                self.num_flashes += 1
            })
            .count()
    }

    fn inc_flash_reset(&mut self, r: usize, c: usize) {
        let cell = &mut self.cells[r][c];
        *cell += 1;
        if *cell == 10 {
            if r > 0 {
                if c > 0 { self.inc_flash_reset(r - 1, c - 1) }
                self.inc_flash_reset(r - 1, c);
                if c < (self.num_cols() - 1) { self.inc_flash_reset(r - 1, c + 1) }
            }

            if c > 0 { self.inc_flash_reset(r, c - 1) }
            if c < (self.num_cols() - 1) { self.inc_flash_reset(r, c + 1) }

            if r < (self.num_rows() - 1) {
                if c > 0 { self.inc_flash_reset(r + 1, c - 1) }
                self.inc_flash_reset(r + 1, c);
                if c < self.num_cols() - 1 { self.inc_flash_reset(r + 1, c + 1) }
            }
        }
    }
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let mut cavern = Cavern::parse(input);
    (0..100).map(|_i| cavern.step() as i64).sum()
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let mut cavern = Cavern::parse(input);
    (1..)
        .find_map(|step|
            if cavern.step() == cavern.num_cells() {
                Some(step)
            } else {
                None
            }
        )
        .unwrap()
}
