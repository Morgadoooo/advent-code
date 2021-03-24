use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input_answers = include_str!("day6")
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(cond, _)| !*cond)
        .map(|(_, identity)| {
            identity
                .map(str::chars)
                .map(|chars| chars.map(|c| c).collect::<Vec<char>>())
                .into_iter()
                .map(|v| {
                    v.into_iter()
                        .map(|values| {
                            let mut m = HashMap::new();
                            m.entry(values).or_insert_with(|| 1);
                            m
                        })
                        .collect::<Vec<HashMap<_, _>>>()
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|vhm| {
                    let mut new_hm: HashMap<char, i32> = HashMap::new();
                    vhm.into_iter().for_each(|h| new_hm.extend(h));
                    new_hm
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<HashMap<char, i32>>>>()
        .into_iter()
        .map(|vhm| {
            let mut new_hm: HashMap<char, i32> = HashMap::new();
            vhm.into_iter().for_each(|h| new_hm.extend(h));
            new_hm
        })
        .collect::<Vec<_>>();
    dbg!(&input_answers
        .iter()
        .map(|v| v.keys().len())
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>());
}
