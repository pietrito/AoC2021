use std::collections::HashMap;

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

/**
 *   0:      1:      2:      3:      4:
 *  AAAA    ....    AAAA    AAAA    ....
 * B    C  .    C  .    C  .    C  B    C
 * B    C  .    C  .    C  .    C  B    C
 *  ....    ....    DDDD    DDDD    DDDD
 * E    F  .    F  E    .  .    F  .    F
 * E    F  .    F  E    .  .    F  .    F
 *  GGGG    ....    GGGG    GGGG    ....
 *
 *   5:      6:      7:      8:      9:
 *  AAAA    AAAA    AAAA    AAAA    AAAA
 * B    .  B    .  .    C  B    C  B    C
 * B    .  B    .  .    C  B    C  B    C
 *  DDDD    DDDD    ....    DDDD    DDDD
 * .    F  E    F  .    F  E    F  .    F
 * .    F  E    F  .    F  E    F  .    F
 *  GGGG    GGGG    ....    GGGG    GGGG
 *
 *  Frequencies:
 *  A: 8    <- Done
 *  B: 6*   <- Done
 *  C: 8    <- Done
 *  D: 7
 *  E: 4*   <- Done
 *  F: 9*   <- Done
 *  G: 7
 *
 */

fn pattern_to_wire(pattern: u8) -> u32 {
    match pattern {
        0b1110111 => 0,
        0b0010010 => 1,
        0b1011101 => 2,
        0b1011011 => 3,
        0b0111010 => 4,
        0b1101011 => 5,
        0b1101111 => 6,
        0b1010010 => 7,
        0b1111111 => 8,
        0b1111011 => 9,
        _ => panic!("Unknown pattern"),
    }
}

pub fn part2(input: &str) -> String {
    let mut final_sum: u32 = 0;

    for line in input.trim().lines() {
        let mut parts = line.split(" | ");
        let signals: Vec<&str> = parts.next().unwrap().split(' ').collect();
        let digits: Vec<&str> = parts.next().unwrap().split(' ').collect();

        let mut wire_map: HashMap<char, u8> = HashMap::new();

        // Find the 3 unique frequencies
        let real_b = "abcdefg"
            .chars()
            .map(|c| (c, signals.join("").matches(c).count()))
            .find_map(|(c, count)| if count == 6 { Some(c) } else { None })
            .unwrap();
        let real_e = "abcdefg"
            .chars()
            .map(|c| (c, signals.join("").matches(c).count()))
            .find_map(|(c, count)| if count == 4 { Some(c) } else { None })
            .unwrap();
        let real_f = "abcdefg"
            .chars()
            .map(|c| (c, signals.join("").matches(c).count()))
            .find_map(|(c, count)| if count == 9 { Some(c) } else { None })
            .unwrap();

        wire_map.insert(real_b, 1 << 5);
        wire_map.insert(real_e, 1 << 2);
        wire_map.insert(real_f, 1 << 1);

        // Find the A segment using the exclusive difference of signals 1 and 7.
        let one = signals
            .iter()
            .find(|s| s.len() == 2)
            .expect("Should be a single signal pattern with length 2.");
        let seven = signals
            .iter()
            .find(|s| s.len() == 3)
            .expect("Should be a single signal pattern with length 3.");

        let real_a = seven.chars().find(|c| !one.contains(*c)).unwrap();
        wire_map.insert(real_a, 1 << 6);

        // Find the C segment by taking the last unknown of signal 7.
        let real_c = seven
            .chars()
            .find(|c| ![real_a, real_f].contains(c))
            .unwrap();
        wire_map.insert(real_c, 1 << 4);

        // Find the D segment by taking the last unknown of signal 4.
        let four = signals
            .iter()
            .find(|s| s.len() == 4)
            .expect("Should be a single signal pattern with length 4.");
        let real_d = four
            .chars()
            .find(|c| ![real_b, real_c, real_f].contains(c))
            .unwrap();
        wire_map.insert(real_d, 1 << 3);

        // The signal G is the last unknown
        let real_g = "abcdefg"
            .chars()
            .find(|c| !wire_map.keys().collect::<String>().contains(&c.to_string()))
            .unwrap();
        wire_map.insert(real_g, 1);

        // Then, for each 4 digits, concatenate their mask and multiply by the correct power of 10
        let mut number: u32 = 0;
        for (i, digit) in digits.iter().enumerate().take(4) {
            number += pattern_to_wire(digit.chars().map(|c| wire_map[&c]).sum::<u8>())
                * 10u32.pow((3 - i) as u32);
        }
        final_sum += number;
    }

    final_sum.to_string()
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
