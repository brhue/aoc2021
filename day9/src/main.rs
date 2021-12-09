use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let mut risk_level = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_low_point(&map[..], x, y) {
                risk_level += (map[y][x] + 1) as u64;
            }
        }
    }
    risk_level as usize
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let mut low_points = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_low_point(&map[..], x, y) {
                low_points.push((y, x))
            }
        }
    }
    let mut basin_sizes = low_points
        .iter()
        .map(|p| calculate_basin(&map[..], p.1, p.0))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn is_low_point(map: &[Vec<u8>], x: usize, y: usize) -> bool {
    let point = map[y][x];
    let top = y == 0 || point < map[y - 1][x];
    let right = x == map[0].len() - 1 || point < map[y][x + 1];
    let bottom = y == map.len() - 1 || point < map[y + 1][x];
    let left = x == 0 || point < map[y][x - 1];
    top && right && bottom && left
}

// Adapted flood fill algorithm from https://en.wikipedia.org/wiki/Flood_fill
fn calculate_basin(map: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut size = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((y, x));
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if let Some(row) = map.get(node.0) {
            if let Some(val) = row.get(node.1) {
                if *val != 9 && !visited.contains(&node) {
                    visited.insert(node);
                    size += 1;
                    queue.push_back((node.0, node.1 - 1));
                    queue.push_back((node.0, node.1 + 1));
                    queue.push_back((node.0 - 1, node.1));
                    queue.push_back((node.0 + 1, node.1));
                }
            }
        }
    }
    size
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
    ";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 15)
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), 1134)
    }
}
