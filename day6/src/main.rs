fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn simulate(mut fish: [usize; 9], days: usize) -> usize {
    for _day in 0..days {
        fish = fish
            .iter()
            .enumerate()
            .fold([0; 9], |mut fish_by_day, (i, n)| {
                if i == 0 {
                    fish_by_day[6] += n;
                    fish_by_day[8] += n;
                } else {
                    fish_by_day[i - 1] += n;
                }
                fish_by_day
            });
    }
    fish.into_iter().sum()
}

fn part1(input: &str) -> usize {
    let fish =
        input
            .split(',')
            .map(|f| f.parse::<u8>().unwrap())
            .fold([0; 9], |mut fish_by_day, fish| {
                fish_by_day[fish as usize] += 1;
                fish_by_day
            });
    simulate(fish, 80)
}

fn part2(input: &str) -> usize {
    let fish =
        input
            .split(',')
            .map(|f| f.parse::<u8>().unwrap())
            .fold([0; 9], |mut fish_by_day, fish| {
                fish_by_day[fish as usize] += 1;
                fish_by_day
            });
    simulate(fish, 256)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn example_part1() {
        assert_eq!(5934, part1(EXAMPLE))
    }

    #[test]
    fn example_part2() {
        assert_eq!(26984457539, part2(EXAMPLE))
    }
}
