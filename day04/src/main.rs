use std::fs::read_to_string;

fn main() {
    let lines = read_file("day04/input/input.txt");

    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_two(lines: &Vec<String>) -> i32 {
    let mut nb_xmas = 0;

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y].len()-2 > x && lines.len()-2 > y &&
                lines[y].chars().nth(x).unwrap() == 'M' &&
                lines[y].chars().nth(x+2).unwrap() == 'S' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y+2].chars().nth(x).unwrap() == 'M' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if lines[y].len()-2 > x && lines.len()-2 > y &&
                lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y].chars().nth(x+2).unwrap() == 'M' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y+2].chars().nth(x).unwrap() == 'S' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'M' {
                nb_xmas += 1;
            }


            if lines[y].len()-2 > x && lines.len()-2 > y &&
                lines[y].chars().nth(x).unwrap() == 'M' &&
                lines[y].chars().nth(x+2).unwrap() == 'M' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y+2].chars().nth(x).unwrap() == 'S' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'S' {
                nb_xmas += 1;
            }


            if lines[y].len()-2 > x && lines.len()-2 > y &&
                lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y].chars().nth(x+2).unwrap() == 'S' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y+2].chars().nth(x).unwrap() == 'M' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'M' {
                nb_xmas += 1;
            }
        }
    }
    nb_xmas
}

fn part_one(lines: &Vec<String>) -> i32 {
    let mut nb_xmas = 0;

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y].len()-3 > x &&
                lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y].chars().nth(x+1).unwrap() == 'M' &&
                lines[y].chars().nth(x+2).unwrap() == 'A' &&
                lines[y].chars().nth(x+3).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if x > 2 &&
                lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y].chars().nth(x-1).unwrap() == 'M' &&
                lines[y].chars().nth(x-2).unwrap() == 'A' &&
                lines[y].chars().nth(x-3).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if lines.len() > y+3 &&
                lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y+1].chars().nth(x).unwrap() == 'M' &&
                lines[y+2].chars().nth(x).unwrap() == 'A' &&
                lines[y+3].chars().nth(x).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if lines.len() > y+3 &&
                lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y+1].chars().nth(x).unwrap() == 'A' &&
                lines[y+2].chars().nth(x).unwrap() == 'M' &&
                lines[y+3].chars().nth(x).unwrap() == 'X' {
                nb_xmas += 1;
            }

            if lines.len() > y+3 && lines[0].len() > x+3 &&
                lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'M' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'A' &&
                lines[y+3].chars().nth(x+3).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if lines.len() > y+3 && lines[0].len() > x+3 &&
                lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y+1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y+2].chars().nth(x+2).unwrap() == 'M' &&
                lines[y+3].chars().nth(x+3).unwrap() == 'X' {
                nb_xmas += 1;
            }

            if y > 2 && lines[0].len() > x+3 &&
                lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y-1].chars().nth(x+1).unwrap() == 'M' &&
                lines[y-2].chars().nth(x+2).unwrap() == 'A' &&
                lines[y-3].chars().nth(x+3).unwrap() == 'S' {
                nb_xmas += 1;
            }

            if y > 2 && lines[0].len() > x+3 &&
                lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y-1].chars().nth(x+1).unwrap() == 'A' &&
                lines[y-2].chars().nth(x+2).unwrap() == 'M' &&
                lines[y-3].chars().nth(x+3).unwrap() == 'X' {
                nb_xmas += 1;
            }
        }
    }
    nb_xmas
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
