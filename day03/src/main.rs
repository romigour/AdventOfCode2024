use std::fs::read_to_string;

fn main() {
    let lignes = read_file("day03/input/input.txt");

    let mut list_report: Vec<Vec<i32>> = Vec::new();

    for ligne in lignes {
        let ligne_array: Vec<i32> = ligne.split(" ").flat_map(|s| s.parse().ok()).collect();
        list_report.push(ligne_array);
    }

    // println!("len {:?}", list_report);

    println!("Part 1: {:?}", part_one(&list_report));
    println!("Part 2: {:?}", part_two(&list_report));
}

fn part_two(list_report: &Vec<Vec<i32>>) -> i32 {
    let mut nb_good_report = 0;
    for num_report in 0..list_report.len() {
        if is_good_report_with_one_error(&list_report[num_report]) {
            nb_good_report += 1;
        }
    }
    nb_good_report
}

fn part_one(list_report: &Vec<Vec<i32>>) -> i32 {
    let mut nb_good_report = 0;
    for num_report in 0..list_report.len() {
        if is_good_report(&list_report[num_report]) {
            nb_good_report += 1;
        }
    }
    nb_good_report
}

fn is_good_report_with_one_error(report: &Vec<i32>) -> bool {
    let mut is_good = true;

    for idx in 0..report.len() - 1 {
        if !(report[idx] < report[idx + 1] && report[idx + 1] - report[idx] < 4) {
            let mut report_without_error_1 = report.clone();
            let mut report_without_error_2 = report.clone();
            report_without_error_1.remove(idx);
            report_without_error_2.remove(idx + 1);
            is_good = is_good_report(&report_without_error_1) || is_good_report(&report_without_error_2);
            break;
        }
    }

    if is_good {
        return is_good
    }

    is_good = true;

    for idx in 0..report.len() - 1 {
        if !(report[idx] > report[idx + 1] && report[idx] - report[idx + 1] < 4) {
            let mut report_without_error_1 = report.clone();
            let mut report_without_error_2 = report.clone();
            report_without_error_1.remove(idx);
            report_without_error_2.remove(idx + 1);
            is_good = is_good_report(&report_without_error_1) || is_good_report(&report_without_error_2);
            break;
        }
    }

    if is_good {
        return is_good
    }

    false
}

fn is_good_report(report: &Vec<i32>) -> bool {
    let mut is_good = true;

    for idx in 0..report.len() - 1 {
        if !(report[idx] < report[idx + 1] && report[idx + 1] - report[idx] < 4) {
            is_good = false;
            break;
        }
    }

    if is_good {
        return is_good
    }

    is_good = true;

    for idx in 0..report.len() - 1 {
        if !(report[idx] > report[idx + 1] && report[idx] - report[idx + 1] < 4) {
            is_good = false;
            break;
        }
    }

    if is_good {
        return is_good
    }

    false
}

fn read_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}