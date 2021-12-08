fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let count = input
        .lines()
        .map(|line| {
            let (_patterns, output) = line.split_once(" | ").unwrap();
            output
                .trim()
                .split_whitespace()
                .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>();
    count
}

// pabcdefg
// 00000000

fn part2(input: &str) -> usize {
    let sum = input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();
            let (mut easy, mut rest): (Vec<&str>, Vec<&str>) = patterns
                .trim()
                .split_whitespace()
                .partition(|p| matches!(p.len(), 2 | 3 | 4 | 7));
            easy.sort_by_key(|a| a.len());
            let mut key = [""; 10];
            for pattern in easy {
                match pattern.len() {
                    2 => key[1] = pattern,
                    3 => key[7] = pattern,
                    4 => key[4] = pattern,
                    7 => key[8] = pattern,
                    _ => (),
                }
            }
            let three = rest
                .iter()
                .position(|pat| pat.len() == 5 && key[1].chars().all(|c| pat.contains(c)))
                .unwrap();
            key[3] = rest.remove(three);
            let nine = rest
                .iter()
                .position(|pat| key[3].chars().all(|c| pat.contains(c)))
                .unwrap();
            key[9] = rest.remove(nine);
            let five = rest
                .iter()
                .position(|pat| pat.chars().all(|c| key[9].chars().any(|cc| cc == c)))
                .unwrap();
            key[5] = rest.remove(five);
            let two = rest.iter().position(|pat| pat.len() == 5).unwrap();
            key[2] = rest.remove(two);
            let six = rest
                .iter()
                .position(|pat| key[5].chars().all(|c| pat.contains(c)))
                .unwrap();
            key[6] = rest.remove(six);
            key[0] = rest.pop().unwrap();

            let digits = output
                .split_whitespace()
                .map(|digit| {
                    let num = key
                        .iter()
                        .position(|k| {
                            k.chars().all(|c| digit.contains(c)) && digit.len() == k.len()
                        })
                        .unwrap();
                    num
                })
                .collect::<Vec<_>>();
            let result = digits.iter().fold(0, |acc, d| acc * 10 + d);
            result
        })
        .sum::<usize>();
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn example_part1() {
        assert_eq!(26, part1(EXAMPLE))
    }

    #[test]
    fn example_part2() {
        assert_eq!(61229, part2(EXAMPLE))
    }
}
