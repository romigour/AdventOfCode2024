use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let lines = read_file("day05/input/input.txt");

    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut is_regle = false;
    let mut pages = HashMap::new();
    let mut regles: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            is_regle = true;
            continue;
        }

        if is_regle {
            let regle: Vec<u32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            regles.push(regle);
        } else {
            let (x, y) = line.split_once('|').unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();
            pages.entry(x).or_insert_with(HashSet::new).insert(y);
        }

    }

    let mut regles_valide = Vec::new();

    for regle in regles {
        if is_regle_valide(&regle, &pages) {
            regles_valide.push(regle);
        }
    }

    let somme: u32 = regles_valide
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();
    somme
}
fn part_two(lines: &Vec<String>) -> u32 {
    let mut is_regle = false;
    let mut pages = HashMap::new();
    let mut regles: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            is_regle = true;
            continue;
        }

        if is_regle {
            let regle: Vec<u32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            regles.push(regle);
        } else {
            let (x, y) = line.split_once('|').unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();
            pages.entry(x).or_insert_with(HashSet::new).insert(y);
        }

    }

    // println!("pages {:?}", pages);
    // println!("regles {:?}", regles);

    let mut regles_invalide = Vec::new();

    for regle in regles {
        if !is_regle_valide(&regle, &pages) {
            let regle_invalide_a_jour = correction_regle(&regle, &pages);
            regles_invalide.push(regle_invalide_a_jour);
        }
    }

    let somme: u32 = regles_invalide
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();
    somme
}

fn correction_regle(regle: &[u32], pages: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut priorites = HashMap::new();

    for &num_page in regle {
        let mut priorite = 0;
        for (&p, hash_set) in pages {
            if num_page == p {
                priorite += hash_set.iter().filter(|&&y| regle.contains(&y)).count();
            }
        }
        priorites.insert(num_page, priorite);
    }

    let mut regles_a_jour = regle.to_vec();
    regles_a_jour.sort_by_key(|page| -(priorites[page] as i32));
    regles_a_jour
}
fn is_regle_valide(regle: &[u32], pages: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut positions = HashMap::new();

    for (i, &page) in regle.iter().enumerate() {
        positions.insert(page, i);
    }

    for (&x, ys) in pages {
        if let Some(&pos_x) = positions.get(&x) {
            for &y in ys {
                if let Some(&pos_y) = positions.get(&y) {
                    if pos_x > pos_y {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
