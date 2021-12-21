use std::collections::VecDeque;
use std::cmp::Ordering;

trait Visitor {
    fn literal(version: u8, number: u32) -> u32;
    fn operator<I>(version: u8, packet_type: u8, results: I) -> u32 where I: Iterator<Item=u32>;
}

pub struct BitReader {
    bits: VecDeque<u8>,
    current_data: u32,
    current_size: usize,
    bits_read: usize
}

impl BitReader {
    pub fn from_hex_string(input: &str) -> Self {
        Self {
            bits: input.chars()
                .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
                .collect(),
            current_data: 0,
            current_size: 0,
            bits_read: 0
        }
    }

    pub fn read_bits(&mut self, num_bits: usize) -> Option<u32> {
        while self.current_size < num_bits {
            self.current_data = (self.current_data << 4) | (self.bits.pop_front()? as u32);
            self.current_size += 4;
        }

        let remaining_bits = self.current_size - num_bits;
        let result = self.current_data >> remaining_bits;
        self.current_data = self.current_data & (((1 << remaining_bits) - 1) as u32);
        self.current_size = remaining_bits;
        self.bits_read += num_bits;

        Some(result)
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        self.read_bits(1).map(|x| x != 0)
    }

    pub fn bits_read(&self) -> usize {
        self.bits_read
    }
}

fn parse_packet<V>(reader: &mut BitReader) -> u32 where V: Visitor {
    let version = reader.read_bits(3).unwrap() as u8;
    let packet_type = reader.read_bits(3).unwrap() as u8;
    match packet_type {
        4 => V::literal(version, parse_literal(reader)),
        _ => parse_operator::<V>(reader, version, packet_type)
    }
}

fn parse_literal(reader: &mut BitReader) -> u32 {
    let mut value = 0;
    loop {
        let more = reader.read_bit().unwrap();
        let group = reader.read_bits(4).unwrap();
        value = (value << 4) | group;
        if !more { break; }
    }
    value
}

fn parse_operator<V>(reader: &mut BitReader, version: u8, packet_type: u8) -> u32 where V: Visitor {
    let length_type = reader.read_bit().unwrap();
    if length_type {
        let num_packets = reader.read_bits(11).unwrap() as usize;
        V::operator(version, packet_type,
            std::iter::from_fn(|| Some(parse_packet::<V>(reader)))
                .take(num_packets)
        )
    } else {
        let num_bits = reader.read_bits(15).unwrap() as usize;
        let end_offset = reader.bits_read() + num_bits;
        V::operator(version, packet_type,
            std::iter::from_fn(||
                match reader.bits_read().cmp(&end_offset) {
                    Ordering::Less => Some(parse_packet::<V>(reader)),
                    Ordering::Equal => None,
                    Ordering::Greater => panic!("read packet overrun")
                }
            )
        )
    }
}

struct VersionSum {}
impl Visitor for VersionSum {
    fn literal(version: u8, _number: u32) -> u32 {
        version as u32
    }
    fn operator<I>(version: u8, _packet_type: u8, results: I) -> u32 where I: Iterator<Item=u32> {
        results.fold(version as u32, |total, result| total + result)
    }
}

pub fn part1(input: &str) -> u32 {
    let mut reader = BitReader::from_hex_string(input);
    parse_packet::<VersionSum>(&mut reader)
}

pub fn part2(input: &str) -> u32 {
    0
}
