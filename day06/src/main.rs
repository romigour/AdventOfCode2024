use std::fs::read_to_string;

fn main() {
    let lines = read_file("day06/input/sample.txt");

    println!("Part 1: {:?}", part_one(&lines));
    // println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    let width = lines[0].len();
    let height = lines.len();
    let mut carte: Vec<Vec<&str>> = vec![vec!["."; width]; height];

    let mut position = (0, 0);

    for y in 0..height {
        let mut line_array: Vec<&str> = lines[y].split("").filter(|&x| !x.is_empty()).collect();
        println!("{:?}", line_array);
        for x in 0..width {
            carte[y][x] = line_array[x];
            if carte[y][x] == "<" || carte[y][x] == "^" || carte[y][x] == ">" || carte[y][x] == "v" {
                println!("Start at [{:?};{:?}]", x, y);
                position = (x, y);
            }
        }
    }
   // println!("{:?}", carte);
   println!("{:?}", position);
   0
}
fn part_two(lines: &Vec<String>) -> u32 {
   0
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
