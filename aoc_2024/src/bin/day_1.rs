use std::io::BufRead;

struct Input {
    lhs: Vec<u32>,
    rhs: Vec<u32>,
}

fn read_input(path: &std::path::Path) -> Input {
    println!("Loading from: {}", path.display());

    let file = std::fs::File::open(path).expect("Failed to open file");
    let file_reader = std::io::BufReader::new(file);

    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();

    for line in file_reader.lines() {
        let line_string = line.expect("Failed to get line");

        let words: Vec<&str> = line_string.split_ascii_whitespace().collect();
        let left_word = words.first().expect("Failed to get first word");
        let right_word = words.get(1).expect("Failed to get second word");

        lhs.push(left_word.parse().expect("Failed to convert to int"));
        rhs.push(right_word.parse().expect("Failed to convert to int"));
    }
    Input { lhs, rhs }
}

fn main() {
    // Read command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected exactly one argument.")
    }
    let file_path = std::path::Path::new(args.get(1).expect("Failed to get first argument"))
        .canonicalize()
        .expect("Failed to parse path");
    let mut input = read_input(&file_path);

    // Do the first half
    input.lhs.sort_unstable();
    input.rhs.sort_unstable();

    let total_distance: u32 = input
        .lhs
        .iter()
        .zip(input.rhs.iter())
        .map(|tup| u32::abs_diff(*tup.0, *tup.1))
        .sum();

    println!("The answer to the first half is: {}", total_distance);

    // Do the second half
    let count_matches_in_rhs = |&val: &u32| -> u32 {
        input
            .rhs
            .iter()
            .filter(|&&rhs_val| rhs_val == val)
            .count()
            .try_into()
            .unwrap()
    };

    let similarity_score: u32 = input
        .lhs
        .iter()
        .map(|item| item * count_matches_in_rhs(item))
        .sum();

    println!("The answer to the second half is: {}", similarity_score);
}
