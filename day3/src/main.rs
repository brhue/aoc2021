use std::collections::HashMap;
fn main() {
    let input: Vec<_> = include_str!("../input.txt").lines().collect();
    println!("part1: {}", part1(&input));
}

fn part1(input: &[&str]) -> usize {
    let cols = input[0].len();
    let counts = input.iter().fold(HashMap::new(), |mut h, bits| {
        for (index, bit) in bits.chars().enumerate() {
            let item = h.entry(index).or_insert_with(String::new);
            item.push(bit);
        }
        h
    });
    let mut g_rate = String::new();
    let mut e_rate = String::new();
    for i in 0..cols {
        let (zeroes, ones) =
            counts
                .get(&i)
                .unwrap()
                .chars()
                .fold((0, 0), |(mut zeroes, mut ones), c| {
                    match c {
                        '1' => ones += 1,
                        '0' => zeroes += 1,
                        _ => unreachable!(),
                    }
                    (zeroes, ones)
                });
        if zeroes > ones {
            g_rate.push('0');
            e_rate.push('1');
        } else {
            g_rate.push('1');
            e_rate.push('0');
        }
    }
    usize::from_str_radix(&g_rate, 2).unwrap() * usize::from_str_radix(&e_rate, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let input: Vec<_> = EXAMPLE
            .lines()
            // .map(|b| usize::from_str_radix(b, 2).unwrap())
            .collect();
        assert_eq!(part1(&input), 198)
    }
}
