pub fn process(input: String) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        //dumb approach first:

        //search forwards
        let mut foundfirst: Option<u32> = None;
        for (_i, c) in line.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                foundfirst = c.to_digit(10);
                break;
            }
        }
        //search backwards
        let mut foundlast: Option<u32> = None;
        for (_i, c) in line.chars().enumerate() {
            // print!("{}", c);
            if c.is_ascii_digit() {
                foundlast = c.to_digit(10);
                break;
            }
        }
        let (n1, n2) = (foundfirst.unwrap(), foundlast.unwrap());
        let computednum = n1 * 10 + n2;
        sum += computednum;
        // println!("{}", computednum);

        // line.find(char::is_ascii_digit())
        //get first and last number in string
        //if only one number, double it
    }
    sum.to_string()
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
    }
}
