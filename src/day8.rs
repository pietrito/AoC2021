/// In the output values, how many times do digits 1, 4, 7, or 8 appear?
pub fn part1(input: &str) -> String {
    input
        .trim()
        .lines()
        .into_iter()
        .map(|l| {
            // Get the second part of the line
            l.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .into_iter()
                // Filter the unique sequence lengths
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        // Sum the number of output digits that had unique sequence lengths
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_example1() {
        assert_eq!(part1(INPUT), "26");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../inputs/8")), "303");
    }

    #[test]
    fn test_example2() {
        assert_eq!(part2(INPUT), "61229");
    }
}
