pub fn process(input: String) -> String {
    let newinput = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "thr3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "ni9ne");

    let lines = newinput.lines();
    let mut sum = 0;
    for line in lines {
        let mut nums = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                nums.push(c.to_digit(10).unwrap());
            }
        }

        // let mut iter = caps.iter();
        // let s1 = iter.next().unwrap().unwrap().as_str();
        // let s2 = iter.last().unwrap().unwrap().as_str();
        //
        // println!("first is {} last is {}", s1, s2);
        //
        let n1 = nums.first().unwrap();
        let n2 = nums.last().unwrap();

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
        println!("{} -> {} ", line, computednum);
        sum += computednum;
    }
    sum.to_string()
}

// fn get_num(input: &str) -> Option<u32> {
//     match input {
//         "one" | "1" => Some(1),
//         "two" | "2" => Some(2),
//         "three" | "3" => Some(3),
//         "four" | "4" => Some(4),
//         "five" | "5" => Some(5),
//         "six" | "6" => Some(6),
//         "seven" | "7" => Some(7),
//         "eight" | "8" => Some(8),
//         "nine" | "9" => Some(9),
//         _ => None,
//     }
// }

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
