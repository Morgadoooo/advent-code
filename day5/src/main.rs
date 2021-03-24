fn convert(bits: &Vec<usize>) -> usize {
    bits.iter().fold(0, |mut acc, b| {
        acc <<= 1;
        acc ^= b;
        acc
    })
}

fn main() {
    let mut input_boarding = include_str!("day5")
        .lines()
        .map(|l| l.split_at(7))
        .map(|(row, columns)| {
            let binary = {
                (
                    row.chars()
                        .map(|c| {
                            let binary = {
                                match c {
                                    'F' => 0,
                                    'B' => 1,
                                    _ => unreachable!(),
                                }
                            };
                            binary
                        })
                        .collect::<Vec<usize>>(),
                    columns
                        .chars()
                        .map(|c| {
                            let binary = {
                                match c {
                                    'L' => 0,
                                    'R' => 1,
                                    _ => unreachable!(),
                                }
                            };
                            binary
                        })
                        .collect::<Vec<usize>>(),
                )
            };
            convert(&binary.0) * 8 + convert(&binary.1)
        })
        .collect::<Vec<_>>();
    input_boarding.sort();
    let resolve1 = input_boarding.last().unwrap();

    let resolve2 = input_boarding
        .windows(2)
        .find(|v| v[0] == v[1] - 2)
        .unwrap()[0]
        + 1;
    dbg!(resolve1);
    dbg!(resolve2);
}
