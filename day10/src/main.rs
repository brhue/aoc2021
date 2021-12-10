fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let sum_of_bad = input
        .lines()
        .map(is_corrupted)
        .filter(|c| c.0)
        .map(|c| {
            let bad_char = c.1.unwrap();
            match bad_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        })
        .sum();
    sum_of_bad
}

fn part2(input: &str) -> usize {
    let incomplete_lines: Vec<&str> = input.lines().filter(|l| !is_corrupted(l).0).collect();
    let mut scores: Vec<u64> = incomplete_lines
        .iter()
        .map(|l| {
            let mut stack = vec![];
            let mut chars = l.chars();
            stack.push(chars.next().unwrap());

            for char in chars {
                match char {
                    ')' => {
                        stack.pop();
                    }
                    ']' => {
                        stack.pop();
                    }
                    '}' => {
                        stack.pop();
                    }
                    '>' => {
                        stack.pop();
                    }
                    _ => stack.push(char),
                }
            }
            stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => ")",
                    '[' => "]",
                    '{' => "}",
                    '<' => ">",
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .map(|l| {
            let mut score = 0u64;
            for char in l.chars() {
                match char {
                    ')' => score = (score * 5) + 1,
                    ']' => score = (score * 5) + 2,
                    '}' => score = (score * 5) + 3,
                    '>' => score = (score * 5) + 4,
                    _ => unreachable!(),
                }
            }
            score
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2_usize] as usize
}

fn is_corrupted(line: &str) -> (bool, Option<char>) {
    let mut stack = vec![];
    let mut chars = line.chars();
    stack.push(chars.next().unwrap());

    for char in chars {
        match char {
            ')' => {
                if stack.last().unwrap() != &'(' {
                    return (true, Some(char));
                }
                stack.pop();
            }
            ']' => {
                if stack.last().unwrap() != &'[' {
                    return (true, Some(char));
                }
                stack.pop();
            }
            '}' => {
                if stack.last().unwrap() != &'{' {
                    return (true, Some(char));
                }
                stack.pop();
            }
            '>' => {
                if stack.last().unwrap() != &'<' {
                    return (true, Some(char));
                }
                stack.pop();
            }
            _ => stack.push(char),
        }
    }
    (false, None)
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
            [
                (true, Some('}')),
                (true, Some(')')),
                (true, Some(']')),
                (true, Some(')')),
                (true, Some('>'))
            ]
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(288957, part2(EXAMPLE))
    }
}
