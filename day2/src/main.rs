use std::error::Error;

fn resolve_policy_1(input: &Vec<((usize, usize), char, &str)>) -> usize {
    input
        .iter()
        .map(|value| {
            let valid = {
                if value.0 .0 <= value.2.matches(value.1).count()
                    && value.0 .1 >= value.2.matches(value.1).count()
                {
                    true
                } else {
                    false
                }
            };
            valid
        })
        .collect::<Vec<bool>>()
        .iter()
        .filter(|&n| *n == true)
        .count()
}

fn resolve_policy_2(input: &Vec<((usize, usize), char, &str)>) -> usize {
    input
        .iter()
        .map(|value| {
            let valid = {
                if value.2.chars().nth(value.0 .0 - 1) == Some(value.1)
                    && value.2.chars().nth(value.0 .1 - 1) != Some(value.1)
                {
                    true
                } else if value.2.chars().nth(value.0 .0 - 1) != Some(value.1)
                    && value.2.chars().nth(value.0 .1 - 1) == Some(value.1)
                {
                    true
                } else {
                    false
                }
            };
            valid
        })
        .collect::<Vec<bool>>()
        .iter()
        .filter(|&n| *n == true)
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let liste_pwd: Vec<((usize, usize), char, &str)> = include_str!("day2")
        .lines()
        .map(|l| l.split_whitespace().into_iter().take(3).collect::<Vec<_>>())
        .map(|v| {
            let rule = {
                let r = v[0]
                    .split('-')
                    .take(2)
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap_or(vec![]);
                (r[0], r[1])
            };
            let (letter_rule, password) = (v[1].chars().next().unwrap(), v[2]);
            (rule, letter_rule, password)
        })
        .collect::<Vec<_>>();
    dbg!(resolve_policy_1(&liste_pwd));
    dbg!(resolve_policy_2(&liste_pwd));
    Ok(())
}
