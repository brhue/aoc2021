fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|l| {
            let points = l.split_once(" -> ").unwrap();
            let p1 = points.0.split_once(',').unwrap();
            let p2 = points.1.split_once(',').unwrap();
            let p1 = (p1.0.parse::<u32>().unwrap(), p1.1.parse::<u32>().unwrap());
            let p2 = (p2.0.parse::<u32>().unwrap(), p2.1.parse::<u32>().unwrap());
            (p1, p2)
        })
        // keep only horz or vert lines
        .filter(|points| points.0 .0 == points.1 .0 || points.0 .1 == points.1 .1)
        .collect::<Vec<_>>();
    let mut diagram: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    for pair in pairs {
        let p1 = pair.0;
        let p2 = pair.1;
        if p1.0 == p2.0 {
            let mut y1 = p1.1;
            let mut y2 = p2.1;
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            for i in y1..=y2 {
                diagram[i as usize][p1.0 as usize] += 1;
            }
        }
        if p1.1 == p2.1 {
            let mut x1 = p1.0;
            let mut x2 = p2.0;
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            for i in x1..=x2 {
                diagram[p1.1 as usize][i as usize] += 1;
            }
        }
    }
    diagram
        .iter()
        .map(|r| r.iter().filter(|spot| *spot > &1).count())
        .sum()
}

fn part2(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(|l| {
            let points = l.split_once(" -> ").unwrap();
            let p1 = points.0.split_once(',').unwrap();
            let p2 = points.1.split_once(',').unwrap();
            let p1 = (p1.0.parse::<i32>().unwrap(), p1.1.parse::<i32>().unwrap());
            let p2 = (p2.0.parse::<i32>().unwrap(), p2.1.parse::<i32>().unwrap());
            (p1, p2)
        })
        .collect::<Vec<_>>();
    let mut diagram: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    for pair in pairs {
        let p1 = pair.0;
        let p2 = pair.1;
        let x_step = if p1.0 <= p2.0 { 1 } else { -1 };
        let y_step = if p1.1 <= p2.1 { 1 } else { -1 };
        let mut spot_to_mark = p1;
        loop {
            diagram[spot_to_mark.1 as usize][spot_to_mark.0 as usize] += 1;
            if spot_to_mark.0 != p2.0 {
                spot_to_mark.0 += x_step;
            }
            if spot_to_mark.1 != p2.1 {
                spot_to_mark.1 += y_step;
            }
            if spot_to_mark.0 == p2.0 && spot_to_mark.1 == p2.1 {
                diagram[spot_to_mark.1 as usize][spot_to_mark.0 as usize] += 1;
                break;
            }
        }
    }
    diagram
        .iter()
        .map(|r| r.iter().filter(|spot| *spot > &1).count())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn example_part1() {
        assert_eq!(5, part1(EXAMPLE))
    }

    #[test]
    fn example_part2() {
        assert_eq!(12, part2(EXAMPLE))
    }
}
