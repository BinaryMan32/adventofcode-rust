fn parse_initial_state(input: &str) -> Vec<u8> {
    input
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

static LIFETIME_RESET: u8 = 6;
static LIFETIME_NEW: u8 = 8;

fn state_buckets(state: &Vec<u8>) -> Vec<u128> {
    (0..(LIFETIME_NEW+1))
        .map(|n| state.iter().filter(|&&x| x == n).count() as u128)
        .collect()
}

fn advance(buckets: &Vec<u128>) -> Vec<u128> {
    let mut result = buckets.clone();
    result.rotate_left(1);
    result[LIFETIME_RESET as usize] += buckets[0];
    result
}

fn simulate(input: &str, days: usize) -> Vec<u128> {
    let initial_state = state_buckets(&parse_initial_state(input));
    std::iter::successors(Some(initial_state), |state| Some(advance(state)))
        .skip(days)
        .next()
        .unwrap()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    simulate(&input[0], 80).iter().sum::<u128>() as i64
}

pub fn part2(input: &Vec<&str>) -> i64 {
    simulate(&input[0], 256).iter().sum::<u128>() as i64
}
