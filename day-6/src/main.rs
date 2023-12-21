use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("Error reading file");
    let file_reader = BufReader::new(file);
    let lines: Vec<String> = file_reader.lines().map(|l| l.unwrap()).collect();
    // =================================================== First part ==================================================
    let times: Vec<i64> = lines[0][11..]
        .split(' ')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let records: Vec<i64> = lines[1][11..]
        .split(' ')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    assert_eq!(times.len(), records.len());
    let mut product: i64 = 1;

    for i in 0..times.len() {
        /*
           You are gonna use the quadratic equation!
           v = h
           (velocity is equal to the hold-up time)
           d = (t - h)*v = (t - h)*h
           (out attempt distance will be given by the product of the remaining
            duration times our speed)
           d = th - h² (looks familiar, right?)
           d > r => th - h² > r => th - h² - r => 0 There we go. For our attempt
           to be better than the record, we need to solve this quadratic inequality.
           The lower and upper bounds will be given by the roots of the following equation:
           (-1)h² + (t)h + (-r) = 0
           So...
           a = -1
           b = t
           c = -r
           gives:
           bounds = (-b +- sqrt(b² - (4)*(a)*(c))) / 2(a)
           bounds = (-t +- sqrt(t² - 4r)) / -2
           bound_1 = (-t + sqrt(t² - 4r)) / -2
           bound_2 = (-t - sqrt(t² - 4r)) / -2
           To assert that t² - 4r >= 0:
           t² >= 4r, we have both of these values, so we just check everytime.
        */
        let time: f64 = times[i] as f64;
        let record: f64 = records[i] as f64;
        let delta: f64 = (time.powf(2.) - 4. * record) as f64;
        if delta >= 0. {
            let bound_1 = (-time + delta.sqrt()) / -2.;
            let bound_2 = (-time - delta.sqrt()) / -2.;
            println!(
                "Race {:?} bounds as floats: {:?}, {:?}",
                i, bound_1, bound_2
            );
            let min_bound: i64 = (bound_1.min(bound_2) + 1.).floor() as i64;
            let max_bound: i64 = (bound_1.max(bound_2) - 1.).ceil() as i64;
            println!("Race {:?} bounds: {:?} - {:?}", i, min_bound, max_bound);
            let amount_of_possible_values = max_bound - min_bound + 1;
            product *= amount_of_possible_values;
        }
    }
    println!("First part solution: {}", product);
    // ================================================== Second part ==================================================
    let big_race_time: f64 = lines[0][11..]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let big_race_record: f64 = lines[1][11..]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let big_race_delta: f64 = (big_race_time.powf(2.) - 4. * big_race_record) as f64;
    if big_race_delta >= 0. {
        let big_race_bound_1 = (-big_race_time + big_race_delta.sqrt()) / -2.;
        let big_race_bound_2 = (-big_race_time - big_race_delta.sqrt()) / -2.;
        println!(
            "Second part race bound as floats: {:?}, {:?}",
            big_race_bound_1, big_race_bound_2
        );
        let big_race_min_bound: i64 = (big_race_bound_1.min(big_race_bound_2) + 1.).floor() as i64;
        let big_race_max_bound: i64 = (big_race_bound_1.max(big_race_bound_2) - 1.).ceil() as i64;
        println!(
            "Second part race bounds: {:?} - {:?}",
            big_race_min_bound, big_race_max_bound
        );
        let amount_of_possible_values = big_race_max_bound - big_race_min_bound + 1;
        println!("Second part solution: {}", amount_of_possible_values);
    }
}
