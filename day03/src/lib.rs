fn bit_stats(input: &[&str], bit: usize) -> i64 {
    input.iter()
        .fold(0, |count, &line| count + match line.as_bytes()[bit] {
            b'1' => 1,
            b'0' => -1,
            _ => panic!("unmatched")
        })
}

fn most_common(input: &[&str], bit: usize) -> u8 {
    if bit_stats(input, bit) >= 0 {b'1'} else {b'0'}
}

fn least_common(input: &[&str], bit: usize) -> u8 {
    if bit_stats(input, bit) >= 0 {b'0'} else {b'1'}
}

fn bits_to_int(bits: &[u8]) -> i64 {
    bits.iter().fold(0, |r, &b| (r << 1) | (if b == b'1' {1} else {0}))
}

pub fn part1(input: &Vec<&str>) -> i64 {
    let gamma = gamma(input);
    let epsilon = epsilon_from_gamma(gamma, input[0].len() as i64);
    gamma * epsilon
}

fn gamma(input: &[&str]) -> i64 {
    let num_bits = input[0].len();
    let bits = (0..num_bits)
        .map(|i| most_common(input, i))
        .collect::<Vec<u8>>();
    bits_to_int(&bits)
}

fn epsilon_from_gamma(gamma: i64, num_bits: i64) -> i64 {
    ((1 << num_bits) - 1) ^ gamma
}

fn reduce_possibilities(input: &Vec<&str>, keep_fn: fn(&[&str], usize) -> u8) -> i64 {
    let num_bits = input[0].len();
    let remaining = (0..num_bits)
        .fold(input.clone(), |possibilities, i| {
            if possibilities.len() <= 1 {
                possibilities
            } else {
                possibilities.iter().for_each(|p| println!("{}", p));
                let keep = keep_fn(&possibilities[..], i);
                possibilities.into_iter()
                    .filter(|&p| p.as_bytes()[i] == keep)
                    .collect::<Vec<&str>>()
            }
        });
    bits_to_int(remaining[0].as_bytes())
}

pub fn part2(input: &Vec<&str>) -> i64 {
    let oxygen_generator_rating = reduce_possibilities(input, most_common);
    let co2_scrubber_rating = reduce_possibilities(input, least_common);
    oxygen_generator_rating * co2_scrubber_rating
}
