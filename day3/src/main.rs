fn main() {
    let input: Vec<_> = include_str!("../input.txt").lines().collect();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &[&str]) -> usize {
    let cols = input[0].len();
    let mask = 2_u32.pow(cols as u32) - 1;
    let mut g_rate = 0;
    for i in 0..cols {
        let mut count = 0;
        for &num in input {
            count += num[i..=i].parse::<i32>().unwrap();
        }
        g_rate = g_rate << 1
            | (match count > (input.len() / 2) as i32 {
                true => 1,
                false => 0,
            })
    }
    let e_rate = g_rate ^ mask;
    (g_rate * e_rate) as usize
}

fn part2(input: &[&str]) -> usize {
    // Most common or a 1
    let oxy_cmp = |(zeroes, ones)| ones >= zeroes;
    // least common or a 0
    let c02_cmp = |(zeroes, ones)| zeroes > ones;
    // oxygen_rating(input) * scrubber_rating(input)
    get_rating(input, oxy_cmp) * get_rating(input, c02_cmp)
}

fn get_rating<F>(input: &[&str], pred: F) -> usize
where
    F: Fn((usize, usize)) -> bool,
{
    let positions = input[0].len();
    let mut items = input.to_owned();
    for i in 0..positions {
        let (zeroes, ones) = count_bits(&items, i);
        let next_bit = if pred((zeroes, ones)) { "1" } else { "0" };
        items = items
            .into_iter()
            .filter(|item| &item[i..=i] == next_bit)
            .collect();
        if items.len() == 1 {
            break;
        }
    }
    usize::from_str_radix(items[0], 2).unwrap()
}

fn count_bits(input: &[&str], column: usize) -> (usize, usize) {
    input.iter().fold((0, 0), |(mut zeroes, mut ones), item| {
        match &item[column..column + 1] {
            "0" => zeroes += 1,
            "1" => ones += 1,
            _ => unreachable!(),
        }
        (zeroes, ones)
    })
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

    #[test]
    fn test_part2() {
        let input: Vec<_> = EXAMPLE.lines().collect();
        assert_eq!(part2(&input), 230)
    }
}
