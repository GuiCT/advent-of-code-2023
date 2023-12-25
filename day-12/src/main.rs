// I based my implementation on this using this video
// https://www.youtube.com/watch?v=g3Ms5e7Jdqo
// I knew that i needed to use Dynamic Programming, but couldn't figure it out

use std::{collections::HashMap, fs};

use itertools::Itertools;

fn count(
    parse: &str,
    patterns: &Vec<usize>,
    mut memo: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    let key = (parse.to_owned(), patterns.clone());
    if let Some(&cached_value) = memo.get(&key) {
        return cached_value;
    }

    // Empty pattern and string -> successfully parsed.
    // Empty one but not the another -> Impossible configuration
    if parse.is_empty() {
        if patterns.is_empty() {
            memo.insert(key, 1);
            return 1;
        } else {
            memo.insert(key, 0);
            return 0;
        }
    }
    if patterns.is_empty() {
        let parse_contains_hashtag = parse.contains('#');
        if parse_contains_hashtag {
            memo.insert(key, 0);
            return 0;
        } else {
            memo.insert(key, 1);
            return 1;
        }
    }

    // Otherwise, continue parsing from next character
    let mut sum = 0_usize;

    // If next char of the parse is a dot or a joker, consume it
    // (We have checked for empty parse, so we can coerce the unwrap here)
    let next_char = parse.chars().nth(0).unwrap();
    if next_char == '.' || next_char == '?' {
        sum += count(&parse[1..], patterns, &mut memo);
    }
    // If char is a hashtag or a joker
    if next_char == '#' || next_char == '?' {
        let next_group_size = patterns[0];
        let parse_size = parse.len();
        let length_next_group_feasible = parse_size >= next_group_size;
        if length_next_group_feasible {
            let no_dots_within = !parse[..next_group_size].contains('.');
            if length_next_group_feasible && no_dots_within {
                let group_and_parse_same_size = next_group_size == parse_size;
                if group_and_parse_same_size {
                    sum += count(&"", &patterns[1..].to_owned(), &mut memo);
                } else {
                    let char_after_group_not_hashtag =
                        parse.chars().nth(next_group_size).unwrap() != '#';
                    if char_after_group_not_hashtag {
                        sum += count(
                            &parse[next_group_size + 1..],
                            &patterns[1..].to_owned(),
                            &mut memo,
                        );
                    }
                }
            }
        }
    }

    memo.insert(key, sum);
    return sum;
}

fn part_1() {
    let lines = fs::read_to_string("input.txt").expect("Error reading file");
    let mut memoized_map = HashMap::new();
    let possibilities_for_each: Vec<_> = lines
        .split('\n')
        .map(|l| {
            let (parse, pattern) = l.split_once(' ').unwrap();
            let pattern = pattern
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            return count(parse, &pattern, &mut memoized_map);
        })
        .collect();
    let sum_of_possibilities: usize = possibilities_for_each.iter().sum();
    println!("Part 1 result: {:?}", sum_of_possibilities);
}

fn part_2() {
    let lines = fs::read_to_string("input.txt").expect("Error reading file");
    let mut memoized_map = HashMap::new();
    let possibilities_for_each: Vec<_> = lines
        .split('\n')
        .map(|l| {
            let (parse, pattern) = l.split_once(' ').unwrap();
            let parse = std::iter::once(parse).cycle().take(5).join("?");
            let pattern = pattern
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            let pattern_original_length = pattern.len();
            let pattern = pattern
                .into_iter()
                .cycle()
                .take(5 * pattern_original_length)
                .collect_vec();
            return count(&parse, &pattern, &mut memoized_map);
        })
        .collect();
    let sum_of_possibilities: usize = possibilities_for_each.iter().sum();
    println!("Part 2 result: {:?}", sum_of_possibilities);
}

fn main() {
    part_1();
    part_2();
}
