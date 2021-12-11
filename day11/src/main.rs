use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("part1: {}", part1(input));
    let elapsed = now.elapsed();
    println!("part1 took {}µs", elapsed.as_micros());

    let now = Instant::now();
    println!("part2: {}", part2(input));
    let elapsed = now.elapsed();
    println!("part2 took {}µs", elapsed.as_micros());
}

#[derive(Debug)]
struct Octo {
    energy: u32,
    flashed: bool,
}

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<Octo>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let energy = c.to_digit(10).unwrap();
                    Octo {
                        energy,
                        flashed: false,
                    }
                })
                .collect()
        })
        .collect();
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += step(&mut map[..]);
    }
    total_flashes
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<Octo>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let energy = c.to_digit(10).unwrap();
                    Octo {
                        energy,
                        flashed: false,
                    }
                })
                .collect()
        })
        .collect();
    let mut steps_taken = 0;
    loop {
        let flashes = step(&mut map[..]);
        steps_taken += 1;

        // 100 is input size
        if flashes == 100 {
            break;
        }
    }
    steps_taken
}

fn step(octos: &mut [Vec<Octo>]) -> usize {
    let neighbors = [
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
    ];

    let mut need_to_flash = false;
    for y in 0..octos.len() {
        for x in 0..octos[0].len() {
            let o = &mut octos[y][x];
            o.energy += 1;
            if o.energy > 9 {
                need_to_flash = true;
            }
        }
    }

    let mut flashes = 0;
    while need_to_flash {
        need_to_flash = false;
        for y in 0..octos.len() {
            for x in 0..octos[0].len() {
                let mut o = &mut octos[y as usize][x as usize];
                if o.energy > 9 && !o.flashed {
                    // we flashing
                    flashes += 1;
                    o.flashed = true;
                    for [dx, dy] in neighbors {
                        let xx = x as i32 + dx;
                        let yy = y as i32 + dy;
                        if (0..10).contains(&xx) && 0 <= yy && yy < 10 {
                            let xx = xx as usize;
                            let yy = yy as usize;
                            let n = &mut octos[yy][xx];
                            n.energy += 1;
                            if n.energy > 9 && !n.flashed {
                                need_to_flash = true
                            }
                        }
                    }
                }
            }
        }
        if !need_to_flash {
            break;
        }
    }

    for y in 0..octos.len() {
        for x in 0..octos[0].len() {
            let mut o = &mut octos[y as usize][x as usize];
            if o.flashed {
                o.energy = 0;
                o.flashed = false;
            }
        }
    }

    flashes
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn example_part1() {
        assert_eq!(1656, part1(EXAMPLE))
    }

    #[test]
    fn example_part2() {
        assert_eq!(195, part2(EXAMPLE))
    }
}
