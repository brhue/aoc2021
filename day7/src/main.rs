fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input))
}

fn part1(input: &str) -> i32 {
    let positions: Vec<_> = input
        .split(',')
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect();
    calculate_fuel(&positions)
}

fn part2(input: &str) -> i32 {
    let positions: Vec<_> = input
        .split(',')
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect();
    calculate_fuel2(&positions)
}

fn calculate_fuel(crabs: &[i32]) -> i32 {
    (0..=1000)
        .map(|h_pos| crabs.iter().map(|c| (c - h_pos).abs()).sum())
        .min()
        .unwrap()
}

fn calculate_fuel2(crabs: &[i32]) -> i32 {
    (0..=1000)
        .map(|h_pos| {
            crabs
                .iter()
                .map(|c| {
                    let diff = (c - h_pos).abs();
                    (1..=diff).sum::<i32>()
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_part1() {
        assert_eq!(37, part1(EXAMPLE))
    }

    #[test]
    fn example_part2() {
        assert_eq!(168, part2(EXAMPLE))
    }
}
