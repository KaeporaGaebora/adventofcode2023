use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIGIT_RE: Regex = Regex::new(r"(\d+)").unwrap();
}
pub fn process(input: String) -> String {
    let mut score = 1;
    let mut lineiter = input.lines();

    let times = DIGIT_RE
        .captures_iter(lineiter.next().unwrap())
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut distances = DIGIT_RE
        .captures_iter(lineiter.next().unwrap())
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("times {:?}, distances {:?}", times, distances);

    for (t, d) in times.iter().zip(distances.iter_mut()) {
        let ways = race_wins(*t, *d);
        println!("for t {}, d {}, ways: {}", t, d, ways);
        score = score * ways;
        println!("score is {}", score);
    }
    score.to_string()
}

pub fn process2(input: String) -> String {
    let mut score = 1;
    let mut lineiter = input.lines();

    let times = DIGIT_RE
        .captures_iter(remove_whitespace(lineiter.next().unwrap()).as_str())
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut distances = DIGIT_RE
        .captures_iter(remove_whitespace(lineiter.next().unwrap()).as_str())
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    //combine times
    // let timestr: String = times.iter().fold(String::new(), |d, f| {});

    println!("times {:?}, distances {:?}", times, distances);

    for (t, d) in times.iter().zip(distances.iter_mut()) {
        let ways = race_wins(*t, *d);
        println!("for t {}, d {}, ways: {}", t, d, ways);
        score = score * ways;
        println!("score is {}", score);
    }
    score.to_string()
}
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

//returns number of possible ways to win race
fn race_wins(time: u64, distance_record: u64) -> u64 {
    let mut sum: u64 = 0;
    let maxcheck = (time as f64 / 2.0).floor() as u64;

    let is_even_game = time % 2 == 0;

    for holdtime in 1..=maxcheck {
        let coastime = (time - holdtime);
        if holdtime * coastime > distance_record {
            // println!(
            //     "hold {}, coast for {}, beats {} ",
            //     holdtime, coastime, distance_record
            // );

            //double for the side we didn't calc, unless we're the middle value
            if is_even_game && holdtime == maxcheck {
                // println!("single val!");

                sum += 1;
            } else {
                // println!("double val!");
                sum += 2;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200"
                .to_string(),
        );
        assert_eq!(result, "288");
    }

    #[test]
    fn test_part2() {
        let result = process2(
            "Time:      7  15   30
Distance:  9  40  200"
                .to_string(),
        );
        assert_eq!(result, "71503");
    }
}
