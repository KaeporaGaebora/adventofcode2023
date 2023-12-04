use regex::Regex;
use std::cmp::max;

const MAX_R: i32 = 12;
const MAX_G: i32 = 13;
const MAX_B: i32 = 14;

struct Regexholder {
    r: Regex,
    g: Regex,
    b: Regex,
}

pub fn process(input: String) -> String {
    let regs = Regexholder {
        r: Regex::new(r"(\d+) red").unwrap(),
        g: Regex::new(r"(\d+) green").unwrap(),
        b: Regex::new(r"(\d+) blue").unwrap(),
    };
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

fn validate_game(line: &str, regs: &Regexholder) -> bool {
    let attempts: Vec<&str> = line.split(";").collect();

    for attempt in attempts {
        if !validate_attempt(attempt, regs) {
            return false;
        }
    }
    true
}

fn validate_attempt(attempt: &str, regs: &Regexholder) -> bool {
    let reds = regs
        .r
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<i32>().unwrap());
    let greens = regs
        .g
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<i32>().unwrap());
    let blues = regs
        .b
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

pub fn process2(input: String) -> String {
    let regs = Regexholder {
        r: Regex::new(r"(\d+) red").unwrap(),
        g: Regex::new(r"(\d+) green").unwrap(),
        b: Regex::new(r"(\d+) blue").unwrap(),
    };
    let mut sum: u64 = 0;

    for (_index, line) in input.lines().enumerate() {
        let c = possible_game(line, &regs);
        let powers: u64 = (c.r * c.b * c.g) as u64;
        sum += powers;
    }
    sum.to_string()
}

struct Colors {
    r: u32,
    g: u32,
    b: u32,
}
impl Colors {
    ///modifies self to be max of self or other
    fn max(&mut self, other: &Colors) {
        self.r = max(self.r, other.r);
        self.g = max(self.g, other.g);
        self.b = max(self.b, other.b);
    }
}

fn possible_game(line: &str, regs: &Regexholder) -> Colors {
    let attempts: Vec<&str> = line.split(";").collect();

    let mut min = Colors { r: 0, g: 0, b: 0 };
    for attempt in attempts {
        let att = possible_attempt(attempt, regs);
        min.max(&att);
    }
    min
}

fn possible_attempt(attempt: &str, regs: &Regexholder) -> Colors {
    let reds = regs
        .r
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<u32>().unwrap());
    let greens = regs
        .g
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<u32>().unwrap());
    let blues = regs
        .b
        .captures(attempt)
        .and_then(|f| f.get(1))
        .map(|f| f.as_str())
        .map(|f| f.to_string().parse::<u32>().unwrap());

    Colors {
        r: reds.unwrap_or(0),
        g: greens.unwrap_or(0),
        b: blues.unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::{process, process2, validate_game, Regexholder};
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
        let regs = Regexholder {
            r: Regex::new(r"(\d+) red").unwrap(),
            g: Regex::new(r"(\d+) green").unwrap(),
            b: Regex::new(r"(\d+) blue").unwrap(),
        };
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

    #[test]
    fn test_part2() {
        let result = process2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .to_string(),
        );
        assert_eq!(result, "2286");
    }
}
