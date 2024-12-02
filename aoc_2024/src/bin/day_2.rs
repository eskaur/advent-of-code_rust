use aoc_2024::get_single_path_as_arg;
use std::io::BufRead;

struct Report {
    levels: Vec<u32>,
}

struct Input {
    reports: Vec<Report>,
}

fn read_input(path: &std::path::Path) -> Input {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let file_reader = std::io::BufReader::new(file);

    let read_report = |line: Result<String, std::io::Error>| -> Report {
        let line_string = line.unwrap();
        Report {
            levels: line_string
                .split_ascii_whitespace()
                .map(|word| word.parse().unwrap())
                .collect(),
        }
    };
    Input {
        reports: file_reader.lines().map(read_report).collect(),
    }
}

fn is_safe(report: &Report) -> bool {
    let diffs: Vec<i32> = report
        .levels
        .windows(2)
        .map(|pair| pair[1] as i32 - pair[0] as i32)
        .collect();

    let sign_first_diff = diffs.first().unwrap().signum();
    if sign_first_diff == 0 {
        return false;
    }

    let is_monotonic = diffs.iter().all(|diff| diff.signum() == sign_first_diff);
    let is_gradual = diffs.iter().all(|diff| 1 <= diff.abs() && diff.abs() <= 3);

    is_monotonic && is_gradual
}

fn is_safe_if_allowed_to_remove_one_bad_level(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    let modified_reports: Vec<Report> = report
        .levels
        .iter()
        .enumerate()
        .map(|(index_to_remove, _)| Report {
            levels: report
                .levels
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != index_to_remove)
                .map(|(_, val)| *val)
                .collect(),
        })
        .collect();

    modified_reports.iter().any(is_safe)
}

fn main() {
    let path = get_single_path_as_arg();
    let input = read_input(&path);

    // Do the first half
    let number_of_safe_reports = input
        .reports
        .iter()
        .filter(|report| is_safe(report))
        .count();

    println!(
        "The answer to the first half is: {}",
        number_of_safe_reports
    );

    // Do the second half
    let number_of_safe_reports_now = input
        .reports
        .iter()
        .filter(|report| is_safe_if_allowed_to_remove_one_bad_level(report))
        .count();

    println!(
        "The answer to the second half is: {}",
        number_of_safe_reports_now
    );
}
