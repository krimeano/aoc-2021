use crate::aoc_lib::{bin_str_to_number, bin_str_to_number_128};

pub fn solve_16_1(raw_input: &[String]) -> u32 {
    let input = to_binary(&raw_input[0]);
    let packet = Packet::from(&input);

    packet.collect_versions()
}


pub fn solve_16_2(raw_input: &[String]) -> u64 {
    let input = to_binary(&raw_input[0]);
    let packet = Packet::from(&input);
    packet.calculate()
}

const TYPE_ID_LITERAL: u8 = 4;
const META_BITS: usize = 15;
const META_COUNT: usize = 11;
const LITERAL_ITEM_SIZE: usize = 5;
// const HEX_SIZE: usize = 4;


#[derive(Debug)]
struct Packet {
    raw: String,
    header: Header,
    content: String,
    remainder: Option<String>,
    value: Option<u64>,
    subpackets: Option<Vec<Packet>>,
}

impl Packet {
    pub fn from(raw: &str) -> Self {
        let header = Header::from(raw);
        let content = raw[header.start_at..].to_string();

        let mut out = Self {
            raw: raw.to_string(),
            header,
            content,
            remainder: None,
            value: None,
            subpackets: None,
        };

        if out.header.is_literal {
            out.parse_literal();
        } else {
            if let Some(is_count) = out.header.is_size_count {
                if is_count { out.parse_subpackets_by_count() } else { out.parse_subpackets_by_length() }
            }
        }
        out
    }

    fn parse_literal(&mut self) {
        let mut str_value = String::new();
        let mut cur_pos = 0;
        let mut next_pos = cur_pos + LITERAL_ITEM_SIZE;
        let mut is_last_chunk = false;

        while !is_last_chunk && next_pos <= self.content.len() {
            let chunk = &self.content[cur_pos..next_pos];
            if let Some(c) = chunk.chars().nth(0) {
                is_last_chunk = c == '0';
            }
            str_value.push_str(&chunk[1..]);
            cur_pos += LITERAL_ITEM_SIZE;
            next_pos += LITERAL_ITEM_SIZE;
        }
        self.value = Some(bin_str_to_number_128(&str_value, 64) as u64);

        if cur_pos < self.content.len() {
            self.remainder = Some(self.content[cur_pos..].to_string())
        };
    }

    fn parse_subpackets_by_count(&mut self) {
        let mut subpackets = Vec::new();
        let mut total_read = 0;
        let mut remainder = self.content.clone();

        while total_read < self.header.size.unwrap() {
            let packet = Packet::from(&remainder);
            if let Some(r) = &packet.remainder {
                remainder = r.clone();
            }
            total_read += 1;
            subpackets.push(packet);
        }
        self.subpackets = Some(subpackets);
        self.remainder = Some(remainder);
    }

    fn parse_subpackets_by_length(&mut self) {
        let mut subpackets = Vec::new();
        let mut total_read_length = 0;
        let mut remainder = self.content.clone();
        while total_read_length < self.header.size.unwrap() {
            let packet = Packet::from(&remainder);
            if let Some(r) = &packet.remainder {
                total_read_length += remainder.len() - r.len();
                remainder = r.clone();
            }
            subpackets.push(packet);
        }
        self.subpackets = Some(subpackets);
        self.remainder = Some(remainder);
    }


    pub fn collect_versions(&self) -> u32 {
        self.header.version + if let Some(subs) = &self.subpackets {
            subs.iter().map(|x| x.collect_versions()).sum()
        } else {
            0
        }
    }

    pub fn calculate(&self) -> u64 {
        if let Some(subs) = &self.subpackets {
            match self.header.type_id {
                0 => subs.iter().map(|x| x.calculate()).sum(),
                1 => subs.iter().map(|x| x.calculate()).product(),
                2 => if let Some(out) = subs.iter().map(|x| x.calculate()).min() { out } else { u64::MAX }
                3 => if let Some(out) = subs.iter().map(|x| x.calculate()).max() { out } else { u64::MIN }
                5 => if subs[0].calculate() > subs[1].calculate() { 1 } else { 0 }
                6 => if subs[0].calculate() < subs[1].calculate() { 1 } else { 0 }
                7 => if subs[0].calculate() == subs[1].calculate() { 1 } else { 0 }
                _ => {
                    panic!("not implemented yet for type_id = {}", self.header.type_id)
                }
            }
        } else {
            self.value.unwrap()
        }
    }
}

#[derive(Debug)]
struct Header {
    version: u32,
    type_id: u8,
    is_literal: bool,
    is_size_count: Option<bool>,
    size: Option<usize>,
    start_at: usize,
}

impl Header {
    pub fn from(raw: &str) -> Self {
        let version = bin_str_to_number(&raw[0..3]);
        let type_id = bin_str_to_number(&raw[3..6]) as u8;
        let is_literal = type_id == TYPE_ID_LITERAL;
        let mut start_at = 6;
        let mut is_size_count = None;
        let mut size = None;

        if !is_literal {
            let is_count = &raw[6..7] == "1";
            is_size_count = Some(is_count);
            start_at = 7;
            let meta_size = if is_count { META_COUNT } else { META_BITS };
            size = Some(bin_str_to_number(&raw[start_at..start_at + meta_size]) as usize);
            start_at += meta_size;
        }

        return Self {
            version,
            type_id,
            is_literal,
            is_size_count,
            size,
            start_at,
        };
    }
}

fn to_binary(line: &str) -> String {
    let mut out = String::new();
    for x in line.chars() {
        let mut a = String::new();
        a.push(x);
        let d: u8 = u8::from_str_radix(&a, 16).unwrap();
        out.push_str(&format!("{:04b}", d));
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::{make_file_name, read_input};

    use super::*;

    #[test]
    fn literal() {
        assert_eq!(to_binary("D2FE28"), "110100101111111000101000");

        let packet = Packet::from(&to_binary("D2FE28"));
        assert_eq!(packet.header.version, 6);
        assert_eq!(packet.header.type_id, 4);
        assert_eq!(packet.header.is_literal, true);
        assert_eq!(packet.value, Some(2021));
        assert_eq!(packet.remainder.unwrap(), "000");
    }

    #[test]
    fn operators() {
        {
            assert_eq!(to_binary("38006F45291200"), "00111000000000000110111101000101001010010001001000000000");

            let a = Packet::from("11010001010");
            assert_eq!(a.value, Some(10));

            let b = Packet::from("0101001000100100");
            assert_eq!(b.value, Some(20));

            let packet = Packet::from(&to_binary("38006F45291200"));
            assert_eq!(packet.header.version, 1);
            assert_eq!(packet.header.type_id, 6);
            assert_eq!(packet.header.is_literal, false);
            assert_eq!(packet.header.is_size_count.unwrap(), false);
            assert_eq!(packet.header.size.unwrap(), 27);
            assert_eq!(packet.subpackets.unwrap().len(), 2);
            assert_eq!(packet.value, None);
            assert_eq!(packet.remainder.unwrap(), "0000000");
        }
        {
            let a = Packet::from("01010000001");
            assert_eq!(a.value, Some(1));
            let a = Packet::from("10010000010");
            assert_eq!(a.value, Some(2));
            let a = Packet::from("00110000011");
            assert_eq!(a.value, Some(3));

            assert_eq!(to_binary("EE00D40C823060"), "11101110000000001101010000001100100000100011000001100000");

            let packet = Packet::from(&to_binary("EE00D40C823060"));
            assert_eq!(packet.header.version, 7);
            assert_eq!(packet.header.type_id, 3);
            assert_eq!(packet.header.is_literal, false);
            assert_eq!(packet.header.is_size_count.unwrap(), true);
            assert_eq!(packet.value, None);
            assert_eq!(packet.header.size.unwrap(), 3);
            assert_eq!(packet.subpackets.unwrap().len(), 3);
            assert_eq!(packet.remainder.unwrap(), "00000");
        }
    }

    #[test]
    fn day_16() {
        let test_data = read_input(make_file_name(true, 16, 1));

        assert_eq!(solve_16_1(&test_data[0..1].to_vec()), 16);
        assert_eq!(solve_16_1(&test_data[1..2].to_vec()), 12);
        assert_eq!(solve_16_1(&test_data[2..3].to_vec()), 23);
        assert_eq!(solve_16_1(&test_data[3..4].to_vec()), 31);


        let test_data = read_input(make_file_name(true, 16, 2));
        assert_eq!(solve_16_2(&test_data[0..1].to_vec()), 3);
        assert_eq!(solve_16_2(&test_data[1..2].to_vec()), 54);
        assert_eq!(solve_16_2(&test_data[2..3].to_vec()), 7);
        assert_eq!(solve_16_2(&test_data[3..4].to_vec()), 9);
        assert_eq!(solve_16_2(&test_data[4..5].to_vec()), 1);
        assert_eq!(solve_16_2(&test_data[5..6].to_vec()), 0);
        assert_eq!(solve_16_2(&test_data[6..7].to_vec()), 0);
        assert_eq!(solve_16_2(&test_data[7..8].to_vec()), 1);
    }
}
