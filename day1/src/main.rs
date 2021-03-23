use std::{error::Error};


fn get_numbers(input: &Vec<isize>, nb_entries: isize, number: isize) -> Option<isize>{
    if nb_entries == 1 {
        input.iter().find(|&&x| x == number).copied()
    } else {
        input
            .iter()
            .filter_map(|num| get_numbers(input, nb_entries-1, number-num)
                .map(|result| result*num) )
            .next()
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let liste_number = 
        include_str!("day1")
        .lines()
        .map(str::parse::<isize>)
        .collect::<Result<Vec<_>,_>>()?;
    dbg!(get_numbers(&liste_number, 2, 2020));
    dbg!(get_numbers(&liste_number, 3, 2020));
    Ok(())
}
