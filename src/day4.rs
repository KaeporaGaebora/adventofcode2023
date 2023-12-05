use regex::Regex;
use std::collections::HashMap;

pub fn process(input: String) -> String {
    let digit_reg = Regex::new(r"(\d{1,2})").unwrap();

    let mut sum = 0;
    for (_game, line) in input.lines().enumerate() {
        let precolon: Vec<&str> = line.splitn(2, ':').collect();
        let gamesplit: Vec<&str> = precolon.get(1).unwrap().splitn(2, '|').collect();

        let rolled_nums_slice = *gamesplit.get(0).unwrap();
        let my_nums_slice = *gamesplit.get(1).unwrap();

        let rolled_nums = get_nums_in_str(&digit_reg, rolled_nums_slice);
        let my_nums = get_nums_in_str(&digit_reg, my_nums_slice);

        let mut matches = 0;
        // print!("game {} matches: ", game + 1);
        for rolled in rolled_nums {
            if my_nums.contains(&rolled) {
                // print!(" {}", rolled);
                matches += 1;
            }
        }
        if matches > 0 {
            let points = 1 << (matches - 1);
            // println!(" that's {} points", points);
            sum += points;
        }
        // println!(" that's 0 points");
    }

    sum.to_string()
}

pub fn process2(input: String) -> String {
    let digit_reg = Regex::new(r"(\d{1,2})").unwrap();

    let mut copymap: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;
    //for a given game number, how many copies have been generated

    for (g, line) in input.lines().enumerate() {
        let game = g + 1;
        let matches = game_matches(&digit_reg, line);

        //how many copies of this card do we have?
        let multiplicity = *copymap.get(&game).unwrap_or(&0) + 1;
        println!("{} copies of game {}", multiplicity, game);

        //generate copies for as many as there are matches
        for i in 1..=matches {
            let newcopy = game + (i as usize);
            println!("add {} copy of game {}", multiplicity, newcopy);
            copymap
                .entry(newcopy)
                .and_modify(|count| *count += multiplicity)
                .or_insert(multiplicity);
        }

        //count copies of this card
        sum += multiplicity;
    }

    //need to know how many total cards we have

    sum.to_string()
}

fn game_matches(regex: &Regex, gameline: &str) -> u32 {
    let precolon: Vec<&str> = gameline.splitn(2, ':').collect();
    let gamesplit: Vec<&str> = precolon.get(1).unwrap().splitn(2, '|').collect();

    let rolled_nums_slice = *gamesplit.get(0).unwrap();
    let my_nums_slice = *gamesplit.get(1).unwrap();

    let rolled_nums = get_nums_in_str(regex, rolled_nums_slice);
    let my_nums = get_nums_in_str(regex, my_nums_slice);

    let mut matches = 0;
    // print!("game {} matches: ", game + 1);
    for rolled in rolled_nums {
        if my_nums.contains(&rolled) {
            // print!(" {}", rolled);
            matches += 1;
        }
    }
    matches
}

fn get_nums_in_str(regex: &Regex, line: &str) -> Vec<u32> {
    regex
        .captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use crate::day4::{process, process2};

    #[test]
    fn test1() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .to_string(),
        );
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = process2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .to_string(),
        );
        assert_eq!(result, "30");
    }
}
