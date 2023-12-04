use regex::Regex;

pub fn process(input: String) -> String {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9").unwrap();

    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        //ugh, just use a regex!!

        // let mut results = vec![];
        // for (_, [path, lineno, line]) in re.captures_iter(line).map(|c| c.extract()) {
        //     results.push((path, lineno.parse::<u64>().unwrap(), line));
        // }
        // let caps = re.captures_iter(line);
        let caps: Vec<_> = re.captures_iter(line).map(|c| c.get(0).unwrap()).collect();

        let s1 = caps.first().unwrap().as_str();
        let s2 = caps.last().unwrap().as_str();

        let n1 = get_num(s1).unwrap();
        let n2 = get_num(s2).unwrap();

        // let mut tokens: Vec<u32> = Vec::new();
        //
        // //search forwards
        // for (_i, c) in line.chars().enumerate() {
        //     if c.is_ascii_digit() {
        //         tokens.push(c.to_digit(10).unwrap());
        //     }
        // }
        // println!("{:?}", results)
        // let n1 = results.first().unwrap()

        // println!("found: {:?}", tokens);
        //
        // let n1 = tokens.first().unwrap();
        // let n2 = tokens.last().unwrap();

        let computednum = n1 * 10 + n2;

        println!("{} -> found [{}{}] -> {}", line, n1, n2, computednum);
        sum += computednum;
    }
    sum.to_string()
}

fn get_num(input: &str) -> Option<u32> {
    match input {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn short_test() {
        let result = process("1aaaaaa2".to_string());
        assert_eq!(result, "12");
    }

    #[test]
    fn part_2() {
        assert_eq!(process("abcone2threexyz".to_string()), "13");
        assert_eq!(process("4nineeightseven2".to_string()), "42");
        assert_eq!(process("aaaatwoaaaa".to_string()), "22");
        assert_eq!(process("kkkkkk1llll".to_string()), "11");
    }

    #[test]
    fn part_2_long() {
        assert_eq!(
            process(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                    .to_string()
            ),
            "281"
        );
    }
}
