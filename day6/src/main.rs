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
                .map(|v| {
                    v.into_iter().fold(HashMap::new(), |mut acc, value| {
                        let count = acc.entry(value).or_insert_with(|| 0);
                        *count += 1;
                        acc
                    })
                })
                .collect::<Vec<HashMap<char, i32>>>()
        })
        .collect::<Vec<Vec<HashMap<char, i32>>>>();

    let resolve = &input_answers
        .into_iter()
        .map(|v| {
            let len = v.len();
            v.into_iter().fold(HashMap::new(), |mut acc, hm| {
                hm.into_iter().for_each(|(k, _)| {
                    let count = acc.entry(k).or_insert(len);
                    *count -= 1;
                });
                acc
            })
        })
        .collect::<Vec<_>>();

    let resolve1 = &resolve
        .iter()
        .map(|v| v.keys().len())
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>();

    let resolve2 = &resolve
        .iter()
        .map(|hm| {
            hm.into_iter().fold(Vec::new(), |mut acc: Vec<_>, values| {
                if values.1 == &0 {
                    acc.push(*values.0)
                }
                acc
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|v| v.len())
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>();
    dbg!(&resolve1);
    dbg!(&resolve2);
}
