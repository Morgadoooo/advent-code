use std::error::Error;

fn check_tree_on_position(input: &Vec<bool>, pos: u32) -> u32 {
  if input[pos as usize] == true{
      1
  }
  else{
    0
  }
}

fn get_new_position(mut pos: u32, endmap: u32,slop: (u32,u32)) -> u32{
if pos+slop.1 >= endmap{
      pos = (pos+slop.1) % endmap;
    }
    else {
      pos = pos+slop.1;
    }
    pos
  }

fn resolve(input: &Vec<Vec<bool>>, slop: (u32,u32)) -> u32 {
  let mut trees: u32 = 0;
  let endmap: u32 = input[0].len() as u32;
  let mut pos: u32 = 0;
  let mut count_jump: u32 = slop.0;
  for values in input.iter() {
    if count_jump == slop.0 {
      trees += check_tree_on_position(values,pos);
      pos = get_new_position(pos, endmap,slop);
      count_jump = 1;
    }
    else{
      count_jump += 1;
    }
  }
  trees
}

fn main() -> Result<(), Box<dyn Error>> {
  let numbers =
    include_str!("day3")
          .lines()
          .map(str::chars)
          .map(|chars| { chars.map(|c| { c == '#'}).collect::<Vec<_>>() })
          .collect::<Vec<_>>();
  
  let mut resolve1 = dbg!(resolve(&numbers, (1,1)));
  resolve1 *= dbg!(resolve(&numbers, (1,3)));
  resolve1 *= dbg!(resolve(&numbers, (1,5)));
  resolve1 *= dbg!(resolve(&numbers, (1,7)));
  resolve1 *= dbg!(resolve(&numbers, (2,1)));
  dbg!(resolve1);
  Ok(())
}