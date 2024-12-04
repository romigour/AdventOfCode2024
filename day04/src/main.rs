use std::fs::read_to_string;

fn main() {
    let lines = read_file("day04/input/sample.txt");

    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_two(lines: &Vec<String>) -> i32 {
    lines.len() as i32
}

fn part_one(lines: &Vec<String>) -> i32 {
    let mut nb_xmas = 0;
    for line in lines {
        if line.contains("XMAS") {
            nb_xmas += 1;
        }
        if line.contains("SAMX") {
            nb_xmas += 1;
        }
    }

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
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


// 2358 trop bas
