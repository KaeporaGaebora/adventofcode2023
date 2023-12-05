use ndarray::prelude::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

fn fill_array(input: String) -> Array<char, Ix2> {
    let ylen = input.find('\n').unwrap();
    let xlen = input.lines().count();

    let mut a = Array::<char, Ix2>::from_elem((xlen, ylen), '.');

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            // println!("{},{} is {}", x, y, c);
            a[[x, y]] = c;
        }
    }
    a
}

pub fn process(input: String) -> String {
    let ylen = input.find('\n').unwrap();
    let xlen = input.lines().count();

    let a = fill_array(input);

    //also create an array used for marking
    let mut found_numbers: HashSet<Coord> = HashSet::new();

    //get x,y coordinate of symbols
    let mut points = Vec::new();

    for (x, line) in a.outer_iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            // println!("{},{} is {}", x, y, c);
            if !c.is_alphanumeric() && !c.eq(&'.') {
                // let symbols = "@#$%&*-=+/";
                let coord = Coord { x, y };
                println!("found a symbol {} at {}", c, coord);
                points.push(coord)
            }
        }
    }

    println!("{:?}", a);

    //search around symbols for numbers.

    let cardinal_directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    //8 cardinal directions
    //search left and right to get all the number

    for point in points {
        for (u, v) in cardinal_directions {
            if let Some(digit_search_coord) = point.add_vec(u, v) {
                if let Some(grabbedchar) = get_char_valid(&a, digit_search_coord) {
                    if grabbedchar.is_ascii_digit() {
                        println!(
                            "found a dig {} at {} near {}",
                            grabbedchar, digit_search_coord, point
                        );
                        //search left to find start of number
                        let first = get_first_digit(&a, digit_search_coord);

                        found_numbers.insert(first);
                    }
                }
            }
        }
    }

    //since its a hashset, and we grabbed first digit of each number we won't have duplicate coordinates/numbers.
    //iterate through each one, grab whole number, parse

    let sum = found_numbers
        .iter()
        .map(|&c| get_full_number(&a, c))
        .reduce(|acc, c| acc + c)
        .unwrap();

    sum.to_string()
}

fn get_full_number(a: &Array<char, Ix2>, first_digit: Coord) -> u32 {
    //continue to the right to get the full number
    //seach for the next invalid digit
    let mut current_coord = first_digit.clone();
    while get_char_is_digit(&a, current_coord) {
        //we are on a valid digit, try the next one?
        // println!("trying {}", current_coord);
        current_coord = current_coord.add_vec(0, 1).unwrap();
    }
    //if while loop broke, then we went too far, go back one

    let ystart = first_digit.y;
    let yfinish = current_coord.y; //techincally this is one past, but the numbers work out

    // println!(" from {} to {}", ystart, yfinish);
    //get number between first_digit and current coord:
    let s = a
        .slice(s![first_digit.x, ystart..yfinish])
        .iter()
        .collect::<String>();

    println!("found number: {}, at {}", s, first_digit);

    s.parse::<u32>().unwrap()
}

fn get_first_digit(a: &Array<char, Ix2>, start_digit: Coord) -> Coord {
    let mut current_coord = start_digit.clone();
    loop {
        if let Some(try_coord) = current_coord.add_vec(0, -1) {
            if get_char_is_digit(&a, try_coord) {
                current_coord = try_coord;
                continue;
            } else {
                //not a digit? stop
                return current_coord;
            }
        } else {
            //can't go left more. stop.
            return current_coord;
        }
    }
}

fn get_char_is_digit(a: &Array<char, Ix2>, coord: Coord) -> bool {
    if let Some(grabbedchar) = get_char_valid(&a, coord) {
        grabbedchar.is_ascii_digit()
    } else {
        false
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord {
    fn add_vec(&self, x: i32, y: i32) -> Option<Coord> {
        //todo, downcast, no checks performed
        let tryx = self.x as i32 + x;
        let tryy = self.y as i32 + y;
        if tryx < 0 || tryy < 0 {
            //request O.B? Don't
            return None;
        }
        Some(Coord {
            x: tryx as usize,
            y: tryy as usize,
        })
    }
}

fn get_char_valid(a: &Array<char, Ix2>, coord: Coord) -> Option<char> {
    let &[xmax, ymax] = a.shape() else { todo!() };
    if coord.x >= xmax || coord.y >= ymax {
        None
    } else {
        Some(a[[coord.x, coord.y]])
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{fill_array, get_first_digit, get_full_number, process, Coord};

    #[test]
    fn test1() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .to_string(),
        );
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_parsing() {
        let input = "1234
5678
.2..
.33.
34..
..65"
            .to_string();

        let ylen = input.find('\n').unwrap();
        let xlen = input.lines().count();

        let a = fill_array(input);

        assert_eq!(get_full_number(&a, Coord { x: 0, y: 0 }), 1234);
        assert_eq!(get_full_number(&a, Coord { x: 1, y: 0 }), 5678);
        assert_eq!(get_full_number(&a, Coord { x: 2, y: 1 }), 2);
        assert_eq!(get_full_number(&a, Coord { x: 3, y: 1 }), 33);
        assert_eq!(get_full_number(&a, Coord { x: 4, y: 0 }), 34);
        assert_eq!(get_full_number(&a, Coord { x: 5, y: 2 }), 65);

        assert_eq!(
            get_first_digit(&a, Coord { x: 0, y: 2 }),
            Coord { x: 0, y: 0 }
        );
    }
}
