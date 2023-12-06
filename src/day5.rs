use itertools::{merge, Itertools};
use kdam::tqdm;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashSet};

lazy_static! {
    static ref DIGIT_RE: Regex = Regex::new(r"(\d+)").unwrap();
    static ref TRIPLET_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
}
pub fn process(input: String) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let seeds = get_seeds(parts.get(0).unwrap());
    println!("Seeds {:?} ", seeds);

    // println!("parts {:?} ", parts);

    let soilmap = get_mapping(parts.get(1).unwrap());
    let fertmap = get_mapping(parts.get(2).unwrap());
    let watermap = get_mapping(parts.get(3).unwrap());
    let lightmap = get_mapping(parts.get(4).unwrap());
    let tempmap = get_mapping(parts.get(5).unwrap());
    let hummap = get_mapping(parts.get(6).unwrap());
    let locmap = get_mapping(parts.get(7).unwrap());

    // println!("soilmap {:?} ", &soilmap);

    let soils = conversion_map(seeds, soilmap);
    println!("soils {:?} ", &soils);

    let fert = conversion_map(soils, fertmap);
    println!("fert {:?} ", &fert);

    let water = conversion_map(fert, watermap);
    println!("water {:?} ", &water);

    let light = conversion_map(water, lightmap);
    println!("light {:?} ", &light);

    let temp = conversion_map(light, tempmap);
    println!("temp {:?} ", &temp);

    let hum = conversion_map(temp, hummap);
    println!("hum {:?} ", &hum);

    let loc = conversion_map(hum, locmap);
    println!("locs {:?} ", &loc);

    loc.iter().min().unwrap().to_string()
}

pub fn process2(input: String) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let seeds = get_seeds(parts.get(0).unwrap());
    let hashedseeds = part_2_seedrange(seeds);
    // println!("Seeds {:?} ", seeds_part2);

    // println!("parts {:?} ", parts);

    let soilmap = get_mapping(parts.get(1).unwrap());
    let fertmap = get_mapping(parts.get(2).unwrap());
    let watermap = get_mapping(parts.get(3).unwrap());
    let lightmap = get_mapping(parts.get(4).unwrap());
    let tempmap = get_mapping(parts.get(5).unwrap());
    let hummap = get_mapping(parts.get(6).unwrap());
    let locmap = get_mapping(parts.get(7).unwrap());

    // println!("soilmap {:?} ", &soilmap);
    println!("got set up");
    let soils = conversion_map2(hashedseeds, soilmap);
    // println!("soils {:?} ", &soils);
    println!("soils done");

    let fert = conversion_map2(soils, fertmap);
    // println!("fert {:?} ", &fert);
    println!("fert done");

    let water = conversion_map2(fert, watermap);
    // println!("water {:?} ", &water);
    println!("water done");

    let light = conversion_map2(water, lightmap);
    // println!("light {:?} ", &light);
    println!("lightdone");

    let temp = conversion_map2(light, tempmap);
    // println!("temp {:?} ", &temp);
    println!("tempdone");

    let hum = conversion_map2(temp, hummap);
    // println!("hum {:?} ", &hum);
    println!("humdone");

    let loc = conversion_map2(hum, locmap);
    // println!("locs {:?} ", &loc);

    loc.iter().map(|[a, _]| *a).min().unwrap().to_string()
}

fn part_2_seedrange(seedrange: Vec<u64>) -> BTreeSet<[u64; 2]> {
    let mut seedranges = BTreeSet::new();

    for chunk in (seedrange.chunks_exact(2)) {
        println!("chunk {:?}", chunk);
        if let &[st, len] = chunk {
            let endseed = st + len;

            seedranges.insert([st, endseed]);
            // let mut seedrange: Vec<u64> = (st..endseed).collect();
            // for s in tqdm!(st..endseed) {
            //     seeds.insert(s);
            // }
        }
    }

    dedupe(seedranges)
}

fn get_seeds(input: &str) -> Vec<u64> {
    DIGIT_RE
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [mat])| mat.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn get_mapping(input: &str) -> Vec<(u64, u64, u64)> {
    TRIPLET_RE
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [dst, src, rng])| {
            (
                dst.parse::<u64>().unwrap(),
                src.parse::<u64>().unwrap(),
                rng.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn conversion_map(inputs: Vec<u64>, mapping: Vec<(u64, u64, u64)>) -> Vec<u64> {
    //first check if its in one of the ranges
    inputs
        .iter()
        .map(|&i| conversion_one(i, &mapping))
        .collect()
}

fn conversion_map2(
    seedranges: BTreeSet<[u64; 2]>,
    mapping: Vec<(u64, u64, u64)>,
) -> BTreeSet<[u64; 2]> {
    //first check if its in one of the ranges
    let mut outranges = BTreeSet::new();
    let mut newsplits = BTreeSet::new();
    for &[st, end] in seedranges.iter() {
        //determine if the range is split.

        //determine if the range happens to start or end between the seedrange

        //3 different possibilities:

        for &(_, src, range) in &mapping {
            let topval = src + range - 1;
            println!("comparing {:?} to {:?}", [st, end], [src, topval]);
            if (src > st && src <= end) && (topval >= st && topval < end) {
                //range is split into 3
                let ran1 = [st, src - 1];
                let ran2 = [src, topval];
                let ran3 = [topval + 1, end];
                newsplits.insert(ran1);
                newsplits.insert(ran2);
                newsplits.insert(ran3);
                println!("split! {:?} and {:?} and {:?}", ran1, ran2, ran3);
            } else if (src > st && src <= end) {
                //range is split into 2
                let ran1 = [st, src - 1];
                let ran2 = [src, topval];
                newsplits.insert(ran1);
                newsplits.insert(ran2);
                println!("split! {:?} and {:?}", ran1, ran2);
            } else if (topval >= st && topval < end) {
                //range is split into 2
                let ran1 = [st, topval];
                let ran2 = [topval + 1, end];
                newsplits.insert(ran1);
                newsplits.insert(ran2);
                println!("split! {:?} and {:?}", ran1, ran2);
            } else {
                //process normally:
                newsplits.insert([st, end]);
            }
        }
    }

    //must have missed a split

    //now process the split ranges:
    println!("outvals:");
    for [st, end] in newsplits {
        if end < st {
            //skip the bad ones??//todo
            continue;
        }
        //since they aren't split, just process the top and bottom value, and ignore the mids
        let first = conversion_one(st, &mapping);
        let last = conversion_one(end, &mapping);
        outranges.insert([first, last]);
        println!("{} to {}", st, end);
    }

    println!("before:");
    for [st, end] in &outranges {
        println!("{} to {}", st, end);
    }
    println!("deduped:");
    let deduped = dedupe(outranges);
    for [st, end] in &deduped {
        println!("{} to {}", st, end);
    }

    deduped.into()
}

fn dedupe(newsplits: BTreeSet<[u64; 2]>) -> BTreeSet<[u64; 2]> {
    let mut deduped: BTreeSet<[u64; 2]> = BTreeSet::new();
    //need to dedupe seed ranges
    let mut siter = newsplits.iter();
    let first = siter.next().unwrap();
    deduped.insert(*first);

    for to_check in siter {
        let mut base = deduped.pop_last().unwrap();
        if let Some(merged) = try_merge(&mut base, to_check) {
            // println!("compared: {:?} and {:?}", &merged, &to_check);

            // if merged[1] < merged[0] {
            //     // panic!()
            // }
            deduped.insert(*merged);
        } else {
            deduped.insert(base);
            deduped.insert(*to_check);
        }
    }

    //compare last with next

    // for (lower, higher) in newsplits.iter().tuple_windows() {
    //     if let Some(merged) = try_merge(lower, higher) {
    //         deduped.insert(merged);
    //     } else {
    //         deduped.insert(lower);
    //         deduped.insert(higher);
    //     }
    // }
    deduped
}

///assume they're sorted. smaller has a lower start num
fn try_merge<'a>(lower: &'a mut [u64; 2], higher: &'a [u64; 2]) -> Option<&'a [u64; 2]> {
    if higher[0] > 0 && lower[1] >= (higher[0] - 1) {
        lower[1] = higher[1];
        return Some(lower);
    }

    //can't merge them:
    None
}

fn conversion_one(input: u64, mapping: &Vec<(u64, u64, u64)>) -> u64 {
    //first check if its in one of the ranges
    for &(dst, src, range) in mapping {
        if input >= src && input < src + range {
            let offset = input - src;
            let newval = dst + offset;
            // println!(
            //     "{} found between {} and {}. offset is {}, mapped to {}",
            //     input,
            //     src,
            //     src + range,
            //     offset,
            //     newval
            // );
            return newval;
        }
    }

    // println!("fallen out: {}", input);
    //its not in range, it stays the same
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                .to_string(),
        );
        assert_eq!(result, "35");
    }

    #[test]
    fn test_part2() {
        let result = process2(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                .to_string(),
        );
        assert_eq!(result, "46");
    }

    #[test]
    fn test_convers() {
        assert_eq!(conversion_one(22, &Vec::from([(30, 40, 5)])), 22);
        assert_eq!(conversion_one(33, &Vec::from([(30, 40, 5)])), 33);
        assert_eq!(conversion_one(55, &Vec::from([(30, 40, 5)])), 55);
        assert_eq!(conversion_one(44, &Vec::from([(30, 40, 5)])), 34);
    }
}
