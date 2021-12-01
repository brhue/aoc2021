fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input.txt");
    let lines: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let count = lines.windows(2).filter(|pair| pair[1] > pair[0]).count();
    println!("part1: {}", count);
}

fn part2() {
    let input = include_str!("../input.txt");
    let lines: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();

    // [a, b, c, d, e]
    // In a sliding window of size 3 [a, b, c] and [b, c, d] we can take a window of size 4
    // and just compare the 'a' and 'd' to check for an increase since items 'b' and 'c' stay the same.
    let count = lines.windows(4).filter(|quad| quad[3] > quad[0]).count();
    println!("part2: {}", count);
}
