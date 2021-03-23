use std::collections::HashMap;

use itertools::Itertools;

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn resolve_1(input: &Vec<HashMap<&str, bool>>, criteria: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|hm| criteria.iter().filter_map(|key| hm.get(key)).count())
        .filter(|&key| key == 7)
        .count()
}

fn resolve_2(input: &Vec<HashMap<&str, bool>>, criteria: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|hm| {
            criteria
                .iter()
                .filter_map(|req| hm.get(req))
                .filter(|&&valid| valid)
                .count()
        })
        .filter(|&req| req == 7)
        .count()
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.
fn main() {
    let criteria = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let input_passport = include_str!("day4")
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(cond, _)| !*cond)
        .map(|(_, identity)| identity)
        .map(|identity| {
            identity
                .into_iter()
                .map(|l| l.split_whitespace())
                .flatten()
                .filter_map(|v| Some(split_once(v)))
                .map(|(k, v)| {
                    let valid = match k {
                        "byr" => (1920..=2002).contains(&v.parse::<i32>().unwrap_or(0)),
                        "iyr" => (2010..=2020).contains(&v.parse::<i32>().unwrap_or(0)),
                        "eyr" => (2020..=2030).contains(&v.parse::<i32>().unwrap_or(0)),
                        "hgt" => match v.split_at(v.len() - 2) {
                            (v, "cm") => (150..=193).contains(&v.parse::<i32>().unwrap_or(0)),
                            (v, "in") => (59..=76).contains(&v.parse::<i32>().unwrap_or(0)),
                            _ => false,
                        },
                        "hcl" => {
                            v.chars()
                                .enumerate()
                                .map(|(i, c)| {
                                    if i == 0 && c == '#' {
                                        true
                                    } else {
                                        matches!(c, 'a'..='f' | '0'..='9')
                                    }
                                })
                                .filter(|&v| v)
                                .count()
                                == 7
                        }
                        "ecl" => {
                            matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                        }
                        "pid" => v.chars().map(|c| c.is_numeric()).filter(|&v| v).count() == 9,
                        _ => false,
                    };

                    (k, valid)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    dbg!(resolve_1(&input_passport, &criteria));
    dbg!(resolve_2(&input_passport, &criteria));
}
