/*
    To be pretty honest, i kinda copied this answer from https://www.reddit.com/r/adventofcode/comments/1883ibu/comment/kbir387/
    But i ended up learning a lot about dot-chaining in Rust, so worth it.
*/
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut number = ['0', '1'];
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line_result = line.expect("Error reading line from stdin!");
        if line_result.is_empty() {
            break;
        }
        // If we change "one" -> "1", some cases will be messed up with
        // because e can be part of another string, like "eight". So it can
        // appear as "oneight". To avoid this shit, use the word before and
        // after the digit.
        let new_line = line_result
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        let digits: Vec<_> = new_line.chars().filter(|char| char.is_digit(10)).collect();

        number[0] = *(digits.first().unwrap());
        number[1] = *(digits.last().unwrap());
        let value = String::from_iter(number).parse::<i32>().unwrap();
        sum += value;
    }
    print!("{}\n", sum);
}
