use std::{
    fmt::format,
    fs::File,
    io::{BufRead, BufReader},
};

fn can_consume(parse: &str, count: i32) -> bool {
    let has_joker = parse.find('?');
    if has_joker.is_some() {
        return false;
    }
    return parse.len() == count.try_into().unwrap();
}

fn consume_parse(parse: &str, pattern: &Vec<i32>) -> (String, Vec<i32>) {
    let mut reduced_pattern = pattern.clone();
    let mut splits: Vec<&str> = parse.split('.').filter(|s| !s.is_empty()).collect();
    if splits.len() <= 1 {
        return (parse.to_string(), reduced_pattern);
    }
    // Verifying end
    while !splits.is_empty() && !reduced_pattern.is_empty() {
        let last_split = splits.last().unwrap();
        let last_pattern = reduced_pattern.last().unwrap().clone();
        if can_consume(&last_split, last_pattern) {
            splits.pop();
            reduced_pattern.pop();
        } else {
            break;
        }
    }
    // Verifying start
    while !splits.is_empty() && !reduced_pattern.is_empty() {
        let first_split = splits.first().unwrap();
        let first_pattern = reduced_pattern.first().unwrap().clone();
        if can_consume(&first_split, first_pattern) {
            splits.remove(0);
            reduced_pattern.remove(0);
        } else {
            break;
        }
    }
    return (splits.join(""), reduced_pattern);
}

fn greedy_search(parse: String, pattern: Vec<i32>) -> i32 {
    let joker_indices: Vec<usize> = parse
        .char_indices()
        .filter_map(|(idx, c)| if c == '?' { Some(idx) } else { None })
        .collect();
    let num_jokers = joker_indices.len();
    let mut sum = 0;
    let total_combinations = 1 << num_jokers;
    (0..total_combinations)
        .map(|n| {
            let mut result = parse.clone();
            for (j, &index) in joker_indices.iter().enumerate() {
                // Determine replacement character for each '?'
                let replacement = if n & (1 << j) == 0 { '.' } else { '#' };
                result.replace_range(index..index + 1, &replacement.to_string());
            }
            result
        })
        .into_iter()
        .for_each(|possibility| {
            let (_, new_pattern) = consume_parse(&possibility, &pattern);
            if new_pattern.is_empty() {
                sum += 1;
            }
        });
    return sum;
}

fn main() {
    let file = File::open("input.txt").expect("Error reading file");
    let file_reader = BufReader::new(file);
    let lines: Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();
    let lines_splitted = lines.iter().map(|l| l.split(' ').collect::<Vec<_>>());
    let parses_and_patterns = lines_splitted.map(|t| {
        let numbers: Vec<i32> = t[1]
            .split(',')
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        return (t[0], numbers);
    });
    let mut total_sum = 0;
    parses_and_patterns.for_each(|(og_parse, og_pattern)| {
        let (preprocessed_parse, preprocessed_pattern) = consume_parse(og_parse, &og_pattern);
        total_sum += greedy_search(preprocessed_parse, preprocessed_pattern);
    });
    println!("{:?}", total_sum);
}

#[cfg(test)]
mod tests {
    use crate::{consume_parse, greedy_search};

    #[test]
    fn test_consuming() {
        let parse = "###.???.###";
        let pattern = vec![3, 1, 1, 3];
        let (reduced_parse, reduced_pattern) = consume_parse(parse, &pattern);
        assert_eq!(reduced_parse, "???");
        assert_eq!(reduced_pattern, vec![1, 1]);
        let parse = "?#?#?#?#?#?#?#?";
        let pattern = vec![1, 3, 1, 6];
        let (reduced_parse, reduced_pattern) = consume_parse(parse, &pattern);
        assert_eq!(reduced_parse, "?#?#?#?#?#?#?#?");
        assert_eq!(reduced_pattern, vec![1, 3, 1, 6]);
    }

    #[test]
    fn test_greedy_evaluation() {
        let num_combinations = greedy_search("?#?#?#?#?#?#?#?".to_string(), vec![1, 3, 1, 6]);
        assert_eq!(num_combinations, 1);
    }
}
