use std::collections::HashMap;

use itertools::Itertools;

type HashIter = Box<dyn Iterator<Item = (char, i32)>>;

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
                    v.into_iter().fold(HashMap::new(), |mut acc, values| {
                        let count = acc.entry(values).or_insert_with(|| 0);
                        *count += 1;
                        acc
                    })
                })
                .collect::<Vec<HashMap<char, i32>>>()
        })
        .collect::<Vec<Vec<HashMap<char, i32>>>>();

    // dbg!(&input_answers);

    // let resolve1 = &input_answers
    //     .into_iter()
    //     .map(|v| {
    //         v.into_iter()
    //             .fold(Box::new(std::iter::empty()) as HashIter, |acc, hm| {
    //                 Box::new(acc.chain(Box::new(hm.into_iter()) as HashIter)) as HashIter
    //             })
    //             .fold(HashMap::new(), |mut acc, tup| {
    //                 let count = acc.entry(tup.0).or_insert_with(|| 0);
    //                 *count += 1;
    //                 acc
    //             })
    //     })
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .map(|v| v.keys().len())
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .sum::<usize>();
    // dbg!(&resolve1);

    let resolve2 = &input_answers
        .into_iter()
        .map(|v| {
            let mut final_hm: HashMap<char, i32> = HashMap::new();
            v.into_iter().for_each(|hm| {
                final_hm.extend(hm.into_iter().map(|(k, v)| {
                    // let count = final_hm.entry(k).or_insert_with(|| 0);
                    // *count += 1;

                    (k.clone(), v.clone())
                }))
            });
            final_hm
        })
        .collect::<Vec<_>>();

    dbg!(&resolve2);
}
