use bitvec::prelude::*;

fn hex_to_bytes(input: &str) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| &input[i..i + 2])
        .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        operands: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version as usize,
            Packet::Operator {
                version, operands, ..
            } => *version as usize + operands.iter().map(Packet::version_sum).sum::<usize>(),
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id, operands, ..
            } => match type_id {
                0 => operands.iter().map(Packet::evaluate).sum(),
                1 => operands.iter().map(Packet::evaluate).product(),
                2 => operands.iter().map(Packet::evaluate).min().unwrap(),
                3 => operands.iter().map(Packet::evaluate).max().unwrap(),
                5 => {
                    let mut values_iter = operands.iter().map(Packet::evaluate);
                    if values_iter.next().unwrap() > values_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let mut values_iter = operands.iter().map(Packet::evaluate);
                    if values_iter.next().unwrap() < values_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let mut values_iter = operands.iter().map(Packet::evaluate);
                    if values_iter.next().unwrap() == values_iter.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    fn parse_literal_value(slice: &BitSlice<Msb0, u8>) -> (usize, &BitSlice<Msb0, u8>) {
        let mut slice_rest = slice;

        let mut value = 0_usize;
        loop {
            let (chunk, rest) = slice_rest.split_at(5);
            slice_rest = rest;

            let nibble = (&chunk[1..5]).load_be::<u8>();

            value <<= 4;
            value |= nibble as usize;

            if !chunk[0] {
                break;
            }
        }

        (value, slice_rest)
    }

    fn parse_operator_operands(slice: &BitSlice<Msb0, u8>) -> (Vec<Packet>, &BitSlice<Msb0, u8>) {
        let mut slice_rest = slice;
        let mut operands = Vec::new();

        if slice_rest[0] {
            let (chunk, rest) = slice_rest.split_at(12);
            slice_rest = rest;

            let operands_count = (&chunk[1..]).load_be::<usize>();

            for _ in 0..operands_count {
                let (operand, rest) = Self::parse(slice_rest);
                slice_rest = rest;

                operands.push(operand);
            }
        } else {
            let (chunk, rest) = slice_rest.split_at(16);
            slice_rest = rest;

            let operands_length_bits = (&chunk[1..]).load_be::<usize>();

            let (mut operands_slice, rest) = slice_rest.split_at(operands_length_bits);
            slice_rest = rest;

            loop {
                let (operand, rest) = Self::parse(operands_slice);
                operands_slice = rest;

                operands.push(operand);

                if operands_slice.len() < 6 {
                    break;
                }
            }
        }

        (operands, slice_rest)
    }

    fn parse(slice: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
        let (header, slice_rest) = slice.split_at(6);
        let mut slice_rest = slice_rest;

        let packet_version_slice = &header[..3];
        let packet_version: u8 = packet_version_slice.load_be();

        let packet_type_slice = &header[3..];
        let packet_type: u8 = packet_type_slice.load_be();

        let packet = match packet_type {
            4 => {
                let (value, rest) = Self::parse_literal_value(slice_rest);
                slice_rest = rest;
                Packet::Literal {
                    version: packet_version,
                    value,
                }
            }
            _ => {
                let (operands, rest) = Self::parse_operator_operands(slice_rest);
                slice_rest = rest;
                Packet::Operator {
                    version: packet_version,
                    type_id: packet_type,
                    operands,
                }
            }
        };

        (packet, slice_rest)
    }
}

fn parse_input(input: &str) -> Packet {
    let input_bytes = hex_to_bytes(input);
    let input_bits = input_bytes.view_bits::<Msb0>();

    Packet::parse(input_bits).0
}

pub fn part1(input: &str) -> usize {
    parse_input(input).version_sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input).evaluate()
}
