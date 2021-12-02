use std::num::ParseIntError;

fn main() {
    let input: Vec<_> = include_str!("../input.txt").lines().collect();

    let part1_ans = part1(&input[..]);
    println!("part1: {}", part1_ans);

    let part2_ans = part2(&input[..]);
    println!("part2: {}", part2_ans);
}

enum SubCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
    Unknown,
}

impl std::str::FromStr for SubCommand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let value: i32 = parts[1].parse()?;
        match parts[0] {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Ok(Self::Unknown),
        }
    }
}

fn part1(input: &[&str]) -> i32 {
    let mut h_pos = 0;
    let mut depth = 0;
    for line in input {
        let command: SubCommand = line.parse().unwrap();
        match command {
            SubCommand::Down(amount) => depth += amount,
            SubCommand::Up(amount) => depth -= amount,
            SubCommand::Forward(amount) => h_pos += amount,
            _ => (),
        }
    }
    h_pos * depth
}

fn part2(input: &[&str]) -> i32 {
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input {
        let command: SubCommand = line.parse().unwrap();
        match command {
            SubCommand::Down(amount) => aim += amount,
            SubCommand::Up(amount) => aim -= amount,
            SubCommand::Forward(amount) => {
                h_pos += amount;
                depth += aim * amount;
            }
            _ => (),
        }
    }
    h_pos * depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part1(&input[..]), 150);
    }
    #[test]
    fn part2_example() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part2(&input[..]), 900);
    }
}
