use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("part1: {}", part1(input));
    let elaspsed = now.elapsed();
    println!("part1 in {}µs", elaspsed.as_micros());

    let now = Instant::now();
    println!("part2: {}", part2(input));
    let elaspsed = now.elapsed();
    println!("part2 in {}µs", elaspsed.as_micros());
}

fn part1(input: &str) -> usize {
    let sum_of_bad = input
        .lines()
        .filter_map(|l| match parse_line(l) {
            LineStatus::Corrupted(c) => Some(c),
            _ => None,
        })
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => 0,
        })
        .sum();
    sum_of_bad
}

fn part2(input: &str) -> usize {
    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|l| match parse_line(l) {
            LineStatus::Incomplete(s) => Some(s.iter().rev().fold(0, |score, c| match c {
                b'(' => (score * 5) + 1,
                b'[' => (score * 5) + 2,
                b'{' => (score * 5) + 3,
                b'<' => (score * 5) + 4,
                _ => unreachable!(),
            })),
            _ => None,
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2_usize] as usize
}

#[derive(Debug, PartialEq)]
enum LineStatus {
    Corrupted(u8),
    Incomplete(Vec<u8>),
}

fn parse_line(line: &str) -> LineStatus {
    let mut stack = Vec::with_capacity(110);
    let mut chars = line.bytes();
    stack.push(chars.next().unwrap());

    for char in chars {
        match char {
            b')' => {
                if stack.pop().unwrap() != b'(' {
                    return LineStatus::Corrupted(char);
                }
            }
            b']' => {
                if stack.pop().unwrap() != b'[' {
                    return LineStatus::Corrupted(char);
                }
            }
            b'}' => {
                if stack.pop().unwrap() != b'{' {
                    return LineStatus::Corrupted(char);
                }
            }
            b'>' => {
                if stack.pop().unwrap() != b'<' {
                    return LineStatus::Corrupted(char);
                }
            }
            _ => stack.push(char),
        }
    }
    LineStatus::Incomplete(stack)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example_part1() {
        assert_eq!(26397, part1(EXAMPLE))
    }

    #[test]
    fn test_is_corrupted() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{";

        let out: Vec<_> = input.lines().map(parse_line).collect();
        assert_eq!(
            &out[..],
            [
                LineStatus::Corrupted(b'}'),
                LineStatus::Corrupted(b')'),
                LineStatus::Corrupted(b']'),
                LineStatus::Corrupted(b')'),
                LineStatus::Corrupted(b'>')
            ]
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(288957, part2(EXAMPLE))
    }
}
