#[cfg(test)]
mod tests {
    use day16::{BitReader, part1, part2};

    static INPUT: &str = include_str!("input");

    #[test]
    fn test_bit_reader() {
        // BA is 10111010
        let mut reader = BitReader::from_hex_string("BA");
        assert_eq!(reader.bits_read(), 0);

        assert_eq!(reader.read_bit().unwrap(), true);
        assert_eq!(reader.bits_read(), 1);

        assert_eq!(reader.read_bits(2).unwrap(), 0b01);
        assert_eq!(reader.bits_read(), 3);

        assert_eq!(reader.read_bits(4).unwrap(), 0b1101);
        assert_eq!(reader.bits_read(), 7);

        assert_eq!(reader.read_bit().unwrap(), false);
        assert_eq!(reader.bits_read(), 8);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("D2FE28"), 6);
        assert_eq!(part1("38006F45291200"), 9);
        assert_eq!(part1("EE00D40C823060"), 14);
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(INPUT), 929);
    }

    #[test]
    fn part2_example() {
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(INPUT), 0);
    }
}
