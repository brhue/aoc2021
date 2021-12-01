fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input.txt");
    let mut previous = None;
    let mut count = 0;
    for line in input.lines() {
        let current = line.parse::<u32>().unwrap();
        if previous == None {
            previous = Some(current);
            continue;
        }
        if current > previous.unwrap() {
            count += 1;
        }
        previous = Some(current)
    }
    println!("part1: {}", count);
}

fn part2() {
    let input = include_str!("../input.txt");
    let lines: Vec<_> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    let mut previous = None;
    let mut count = 0;
    for window in lines.windows(3) {
        let current_sum: u32 = window.iter().sum();
        if previous == None {
            previous = Some(current_sum);
            continue;
        }
        if current_sum > previous.unwrap() {
            count += 1;
        }
        previous = Some(current_sum);
    }
    println!("part2: {}", count);
}
