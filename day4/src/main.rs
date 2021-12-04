fn main() {
    let input: Vec<_> = include_str!("../input.txt").split("\n\n").collect();
    println!("part1: {}", part1(&input[..]))
}

fn part1(input: &[&str]) -> u32 {
    let numbers: Vec<_> = input[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut bingo_boards: Vec<_> = input[1..]
        .iter()
        .map(|&b| BingoBoard::new(b.trim()))
        .collect();
    for number in numbers {
        for board in bingo_boards.iter_mut() {
            board.mark_board(number);
            if board.check_board() {
                let unmarked_sum: u32 = board
                    .board
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| !board.marked[i])
                    .map(|(_, &s)| s)
                    .sum();
                return unmarked_sum * number;
            }
        }
    }
    0
}

#[derive(Debug, Default)]
struct BingoBoard {
    board: [u32; 25],
    marked: [bool; 25],
}

impl BingoBoard {
    fn new(board_spaces: &str) -> Self {
        let board = board_spaces
            .split_whitespace()
            .enumerate()
            .fold([0; 25], |mut b, item| {
                b[item.0] = item.1.parse::<u32>().unwrap();
                b
            });
        Self {
            board,
            ..BingoBoard::default()
        }
    }

    fn mark_board(&mut self, num: u32) {
        let spot = self.board.iter().position(|&i| i == num);
        if let Some(i) = spot {
            self.marked[i] = true
        }
    }

    fn check_board(&self) -> bool {
        for row in 0..5 {
            if (0..5).all(|col| self.marked[row * 5 + col]) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|row| self.marked[row * 5 + col]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
";

    #[test]
    fn example_part1() {
        let input: Vec<_> = EXAMPLE.split("\n\n").collect();
        assert_eq!(4512, part1(&input[..]))
    }
}
