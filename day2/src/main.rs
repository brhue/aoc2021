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
    let (h, d) =
        input
            .iter()
            .map(|l| l.parse().unwrap())
            .fold((0, 0), |(mut h, mut d), command| {
                match command {
                    SubCommand::Down(amount) => d += amount,
                    SubCommand::Up(amount) => d -= amount,
                    SubCommand::Forward(amount) => h += amount,
                    _ => (),
                };
                (h, d)
            });
    h * d
}

fn part2(input: &[&str]) -> i32 {
    let (h, d, _) = input.iter().map(|l| l.parse().unwrap()).fold(
        (0, 0, 0),
        |(mut h, mut d, mut aim), command| {
            match command {
                SubCommand::Down(amount) => aim += amount,
                SubCommand::Up(amount) => aim -= amount,
                SubCommand::Forward(amount) => {
                    h += amount;
                    d += aim * amount;
                }
                _ => (),
            };
            (h, d, aim)
        },
    );
    h * d
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
