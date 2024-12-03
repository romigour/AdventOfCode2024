use std::fs::read_to_string;
use regex::Regex;
fn main() {
    let text = read_file("day03/input/input.txt").join("");

    println!("Part 1: {:?}", part_one(&text));
    println!("Part 2: {:?}", part_two(&text));
}

fn part_two(text: &String) -> i32 {
    let regex_1 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let regex_2 = Regex::new(r"\d{1,3}").unwrap();

    let mut accept_calcul = true;
    let somme: i32 = regex_1.find_iter(&text).map(|s| {
        let string = String::from(s.as_str());
        if string.starts_with("mul") {
            if (accept_calcul) {
                let nombres: Vec<i32> = regex_2.find_iter(s.as_str()).map(|nb| nb.as_str().parse::<i32>().unwrap()).collect();
                return nombres[0].to_string().parse::<i32>().unwrap() * nombres[1].to_string().parse::<i32>().unwrap()
            }
        } else if string.eq("do()") {
            accept_calcul = true;
        } else if string.eq("don't()") {
            accept_calcul = false;
        }
        0
    }).sum();

    somme
}

fn part_one(text: &String) -> i32 {
    let regex_1 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let regex_2 = Regex::new(r"\d{1,3}").unwrap();

    let somme: i32 = regex_1.find_iter(&text).map(|s| {
        let nombres:Vec<i32> = regex_2.find_iter(s.as_str()).map(|nb| nb.as_str().parse::<i32>().unwrap()).collect();
        nombres[0].to_string().parse::<i32>().unwrap() * nombres[1].to_string().parse::<i32>().unwrap()
    }).sum();
    somme
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}