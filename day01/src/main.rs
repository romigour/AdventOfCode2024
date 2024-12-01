use std::fs::read_to_string;

fn main() {
    let lignes = read_file("day01/input/input.txt");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for ligne in lignes {
        let ligne_array: Vec<&str> = ligne.split("   ").collect();
        left_list.push(ligne_array.get(0).unwrap().parse::<i32>().unwrap());
        right_list.push(ligne_array.get(1).unwrap().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    println!("Part 1: {:?}", part_one(&left_list, &right_list));
    println!("Part 2: {:?}", part_two(&left_list, &right_list));
}

fn part_two(list_left: &Vec<i32>, list_right: &Vec<i32>) -> i32 {
    let mut somme: i32 = 0;

    for idx_left in 0..list_left.len() {
        let mut similarity: i32 = 0;
        for idx_right in 0..list_right.len() {
            if list_left[idx_left] == list_right[idx_right] {
                similarity += 1;
            }
        }
        somme += similarity * list_left[idx_left];
    }

    somme
}

fn part_one(list_left: &Vec<i32>, list_right: &Vec<i32>) -> i32 {
    let mut somme: i32 = 0;

    for n in 0..list_left.len() {
        somme += distance(list_left[n], list_right[n]);
    }

    somme
}

fn distance(a: i32, b: i32) -> i32 {
    (a - b).abs()
}


fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}