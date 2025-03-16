#![doc = include_str!("descriptions/day_13.md")]

use std::str::FromStr;

pub fn solve_star_one(input: &str) -> usize {
    let mut packets_pairs: Vec<Vec<Packet>> = Vec::new();
    packets_pairs.push(Vec::new());
    let mut last_pair: &mut Vec<Packet>;

    for line in input.lines() {
        if line.trim().is_empty() {
            packets_pairs.push(Vec::new());
            continue;
        }
        last_pair = packets_pairs.last_mut().unwrap();
        last_pair.push(Packet::from_str(line.trim()).unwrap());
    }

    let mut index_sum = 0;
    for (index, pair) in packets_pairs.iter().enumerate() {
        if pair.len() < 2 {
            panic!("There have to be a full pair");
        }
        let a = &pair[0];
        let b = &pair[1];

        if a < b {
            index_sum += index + 1;
        }
    }
    index_sum
}

pub fn solve_star_two(input: &str) -> usize {
    let mut packets: Vec<Packet> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        packets.push(Packet::from_str(line).unwrap());
    }
    let div0 = Packet::from_str("[[2]]").unwrap();
    let div1 = Packet::from_str("[[6]]").unwrap();
    packets.push(div0.clone());
    packets.push(div1.clone());

    packets.sort();

    (packets.iter().position(|e| *e == div0).unwrap() + 1)
        * (packets.iter().position(|e| *e == div1).unwrap() + 1)
}

#[derive(Debug, Eq, Clone)]
enum Packet {
    Integer(i32),
    Packet(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // this is the final pack, that we want to return at the end
        let mut packs: Vec<Packet> = Vec::new();
        // holds the chars of one number before we parse it to an interger
        let mut buf_string = String::with_capacity(2);

        for c in s.chars() {
            match c {
                '[' => packs.push(Packet::Packet(Vec::new())),
                ',' => {
                    // extract the number if the buf_string isn't empty
                    process_buffered_number(&mut packs, &mut buf_string);
                }
                ']' => {
                    // there could be a number, check the buf_string for a number
                    process_buffered_number(&mut packs, &mut buf_string);

                    // now lets get the last vec to get it in propper
                    let unpack = packs.pop().unwrap();
                    // if the length is 0 we have finished the work
                    if packs.len() == 0 {
                        return Ok(unpack);
                    }

                    // lets
                    if let Packet::Packet(v) = packs.last_mut().unwrap() {
                        v.push(unpack);
                    }
                }
                _ => {
                    buf_string.push(c);
                }
            }
        }

        Err("Can't convert string")
    }
}

/// Used in `Packet::from_str(input)` for converting the buffered strin in to a number
fn process_buffered_number(packs: &mut Vec<Packet>, buf_string: &mut String) {
    if !buf_string.is_empty() {
        let value = buf_string.parse::<i32>().unwrap();

        if let Packet::Packet(v) = packs.last_mut().unwrap() {
            v.push(Packet::Integer(value));
        }
        buf_string.clear();
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Packet(_), Self::Integer(r0)) => {
                self == &Packet::Packet(vec![Packet::Integer(*r0)])
            }
            (Self::Integer(l0), Self::Packet(_)) => {
                other == &Packet::Packet(vec![Packet::Integer(*l0)])
            }
            (Self::Packet(l0), Self::Packet(r0)) => {
                if l0.len() != r0.len() {
                    return false;
                }
                l0.iter().zip(r0.iter()).all(|(a, b)| a == b)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => Some(l0.cmp(r0)),
            (Self::Packet(_), Self::Integer(r0)) => {
                Some(self.cmp(&Packet::Packet(vec![Packet::Integer(*r0)])))
            }
            (Self::Integer(l0), Self::Packet(_)) => {
                Some(Packet::Packet(vec![Packet::Integer(*l0)]).cmp(other))
            }
            (Self::Packet(l0), Self::Packet(r0)) => {
                for (a, b) in l0.iter().zip(r0.iter()) {
                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return Some(ordering),
                    }
                }
                return Some(l0.len().cmp(&r0.len()));
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.cmp(r0),
            (Self::Packet(_), Self::Integer(r0)) => {
                self.cmp(&Packet::Packet(vec![Packet::Integer(*r0)]))
            }
            (Self::Integer(l0), Self::Packet(_)) => {
                Packet::Packet(vec![Packet::Integer(*l0)]).cmp(other)
            }
            (Self::Packet(l0), Self::Packet(r0)) => {
                for (a, b) in l0.iter().zip(r0.iter()) {
                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                return l0.len().cmp(&r0.len());
            }
        }
    }
}

pub const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[cfg(test)]
mod tests_day_13 {
    use super::*;
    use crate::load_input;

    #[test]
    fn test_packet_from_str() {
        // this test is generated by claude
        // Test simple cases
        let result = Packet::from_str("[9]").unwrap();
        if let Packet::Packet(v) = result {
            assert_eq!(v.len(), 1);
            if let Packet::Integer(val) = v[0] {
                assert_eq!(val, 9);
            } else {
                panic!("Expected Integer, got {:?}", v[0]);
            }
        } else {
            panic!("Expected Packet, got {:?}", result);
        }

        // Test nested packets
        let result = Packet::from_str("[[1],[2,3,4]]").unwrap();
        if let Packet::Packet(outer) = result {
            assert_eq!(outer.len(), 2);

            // Check first inner packet
            if let Packet::Packet(inner1) = &outer[0] {
                assert_eq!(inner1.len(), 1);
                if let Packet::Integer(val) = inner1[0] {
                    assert_eq!(val, 1);
                } else {
                    panic!("Expected Integer, got {:?}", inner1[0]);
                }
            } else {
                panic!("Expected Packet, got {:?}", outer[0]);
            }

            // Check second inner packet
            if let Packet::Packet(inner2) = &outer[1] {
                assert_eq!(inner2.len(), 3);
                for (i, expected) in [2, 3, 4].iter().enumerate() {
                    if let Packet::Integer(val) = inner2[i] {
                        assert_eq!(val, *expected);
                    } else {
                        panic!("Expected Integer, got {:?}", inner2[i]);
                    }
                }
            } else {
                panic!("Expected Packet, got {:?}", outer[1]);
            }
        } else {
            panic!("Expected Packet, got {:?}", result);
        }

        // Check case with inner array and normal
        let result = Packet::from_str("[[4,4],4,4]").unwrap();
        if let Packet::Packet(outer) = result {
            assert_eq!(outer.len(), 3);

            // Check first element (a packet)
            if let Packet::Packet(inner) = &outer[0] {
                assert_eq!(inner.len(), 2);
                for val in inner {
                    if let Packet::Integer(num) = val {
                        assert_eq!(*num, 4);
                    } else {
                        panic!("Expected Integer, got {:?}", val);
                    }
                }
            } else {
                panic!("Expected Packet, got {:?}", outer[0]);
            }

            // Check second and third elements (integers)
            for i in 1..3 {
                if let Packet::Integer(val) = outer[i] {
                    assert_eq!(val, 4);
                } else {
                    panic!("Expected Integer, got {:?}", outer[i]);
                }
            }
        } else {
            panic!("Expected Packet, got {:?}", result);
        }

        // Test the case with empty lists `[[]]`
        let result = Packet::from_str("[[]]").unwrap();
        if let Packet::Packet(outer) = result {
            assert_eq!(outer.len(), 1);

            if let Packet::Packet(inner) = &outer[0] {
                assert_eq!(inner.len(), 0)
            } else {
                panic!("Expected Packet with non content, got {:?}", outer);
            }
        } else {
            panic!("Expected Paket with one Element in it, got {:?}", result);
        }
    }

    #[test]
    fn test_smaller_packets() {
        let packet1 = Packet::from_str("[1,1,3,1,1]").unwrap();
        let packet2 = Packet::from_str("[1,1,5,1,1]").unwrap();

        assert_eq![packet1 < packet2, true];
        assert_eq![packet1 > packet2, false];

        let packet1 = Packet::from_str("[[1],[2,3,4]]").unwrap();
        let packet2 = Packet::from_str("[[1],4]").unwrap();

        assert_eq![packet1 < packet2, true];
        assert_eq![packet1 > packet2, false];

        let packet1 = Packet::from_str("[[[]]]").unwrap();
        let packet2 = Packet::from_str("[[]]").unwrap();

        assert_eq![packet1 < packet2, false];

        // [9] vs [[8,7,6]]
        let packet1 = Packet::from_str("[9]").unwrap();
        let packet2 = Packet::from_str("[[8,7,6]]").unwrap();

        assert_eq![packet1 < packet2, false]; // Hey AI! Here the test fails! Why?

        let packet1 = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let packet2 = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();

        assert_eq![packet1 < packet2, false];
    }

    #[test]
    fn star_one_example() {
        let sum = solve_star_one(EXAMPLE);
        assert_eq!(sum, 13);
    }

    #[test]
    fn star_one_input() {
        let sum = solve_star_one(&load_input(13));
        assert_eq!(sum, 4821);
    }

    #[test]
    fn star_two_example() {
        let sum = solve_star_two(EXAMPLE);
        assert_eq!(sum, 140);
    }

    #[test]
    fn star_two_input() {
        let sum = solve_star_two(&load_input(13));
        assert_eq!(sum, 21890);
    }
}
