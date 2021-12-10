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
        .filter_map(is_corrupted)
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
        .filter(|l| is_corrupted(l).is_none())
        .map(|l| {
            let mut stack = Vec::with_capacity(110);
            let mut chars = l.bytes();
            stack.push(chars.next().unwrap());

            for char in chars {
                match char {
                    b')' => {
                        stack.pop();
                    }
                    b']' => {
                        stack.pop();
                    }
                    b'}' => {
                        stack.pop();
                    }
                    b'>' => {
                        stack.pop();
                    }
                    _ => stack.push(char),
                }
            }
            stack.iter().rev().fold(0, |score, c| match c {
                b'(' => (score * 5) + 1,
                b'[' => (score * 5) + 2,
                b'{' => (score * 5) + 3,
                b'<' => (score * 5) + 4,
                _ => unreachable!(),
            })
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2_usize] as usize
}

fn is_corrupted(line: &str) -> Option<u8> {
    let mut stack = Vec::with_capacity(110);
    let mut chars = line.bytes();
    stack.push(chars.next().unwrap());

    for char in chars {
        match char {
            b')' => {
                if stack.pop().unwrap() != b'(' {
                    return Some(char);
                }
            }
            b']' => {
                if stack.pop().unwrap() != b'[' {
                    return Some(char);
                }
            }
            b'}' => {
                if stack.pop().unwrap() != b'{' {
                    return Some(char);
                }
            }
            b'>' => {
                if stack.pop().unwrap() != b'<' {
                    return Some(char);
                }
            }
            _ => stack.push(char),
        }
    }
    None
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

        let out: Vec<_> = input.lines().map(is_corrupted).collect();
        assert_eq!(
            &out[..],
            [Some(b'}'), Some(b')'), Some(b']'), Some(b')'), Some(b'>')]
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(288957, part2(EXAMPLE))
    }
}
