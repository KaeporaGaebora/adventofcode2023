use regex::Regex;

const MAX_R: i32 = 12;
const MAX_G: i32 = 13;
const MAX_B: i32 = 14;

pub fn process(input: String) -> String {
    let reg_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();
    let regs = (reg_re, green_re, blue_re);
    let mut sum = 0;

    for (index, line) in input.lines().enumerate() {
        let gameno = index + 1;
        if validate_game(line, &regs) {
            println!("game {} is good", gameno);
            sum += gameno;
        }
    }
    sum.to_string()
}

fn validate_game(line: &str, regs: &(Regex, Regex, Regex)) -> bool {
    let attempts: Vec<&str> = line.split(";").collect();

    for attempt in attempts {
        if !validate_attempt(attempt, regs) {
            return false;
        }
    }
    true
}

fn validate_attempt(attempt: &str, (red_re, green_re, blue_re): &(Regex, Regex, Regex)) -> bool {
    let reds = red_re
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<i32>().unwrap());
    let greens = green_re
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<i32>().unwrap());
    let blues = blue_re
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<i32>().unwrap());

    // print!("r:{:?} g:{:?} b:{:?}", reds, greens, blues);
    if reds.is_some_and(|n| n > MAX_R)
        || greens.is_some_and(|n| n > MAX_G)
        || blues.is_some_and(|n| n > MAX_B)
    {
        //not possible
        // print!("notpossible");

        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::day2::{process, validate_game};
    use regex::Regex;

    #[test]
    fn test1() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .to_string(),
        );
        assert_eq!(result, "8");
    }
    #[test]
    fn test2() {
        let reg_re: Regex = Regex::new(r"(\d+) red").unwrap();
        let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
        let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();
        let regs = (reg_re, green_re, blue_re);
        let result = validate_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &regs);
        assert_eq!(result, true);

        assert_eq!(
            validate_game(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                &regs
            ),
            false
        );
    }
}
