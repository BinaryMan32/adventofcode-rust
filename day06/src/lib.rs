fn parse_initial_state(input: &str) -> Vec<u8> {
    input
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

static LIFETIME_RESET: u8 = 6;
static LIFETIME_NEW: u8 = 8;
fn advance(input: &Vec<u8>) -> Vec<u8> {
    let num_new = input.iter().filter(|&&x| x == 0u8).count();
    input.into_iter()
        .map(|&x| match x {
            0u8 => LIFETIME_RESET,
            _ => x - 1,
        })
        .chain(std::iter::repeat(LIFETIME_NEW).take(num_new))
        .collect()
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let initial_state = parse_initial_state(&input[0]);
    let final_state = std::iter::successors(Some(initial_state), |state| Some(advance(state)))
        .skip(80)
        .next()
        .unwrap();
    final_state.len() as i64
}

pub fn part2(input: &Vec<&str>) -> i64 {
    0
}
