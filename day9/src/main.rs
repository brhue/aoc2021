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
                low_points.push((x, y))
            }
        }
    }
    let mut basin_sizes = low_points
        .iter()
        .map(|p| calculate_basin(&map[..], p.0, p.1))
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
    let neighbors = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    let mut size = 0;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x as i32, y as i32));
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if map[node.1 as usize][node.0 as usize] != 9 && !visited.contains(&node) {
            visited.insert(node);
            size += 1;

            for [dx, dy] in neighbors {
                let xx = node.0 as i32 + dx;
                let yy = node.1 as i32 + dy;

                if 0 <= xx && xx < map[0].len() as i32 && 0 <= yy && yy < map.len() as i32 {
                    queue.push_back((xx, yy))
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
