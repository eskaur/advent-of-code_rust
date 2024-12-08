use std::cmp::Ordering;

use aoc_2024::get_single_path_as_arg;

struct EquationCandidate {
    result: i64,
    terms: Vec<i64>,
}

struct Input {
    equations: Vec<EquationCandidate>,
}

fn read_input(raw_input: &str) -> Input {
    let mut equations: Vec<EquationCandidate> = Vec::new();

    for line in raw_input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let result: i64 = parts.first().unwrap().parse().unwrap();
        let terms: Vec<i64> = parts
            .get(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|ch| ch.parse().unwrap())
            .collect();
        equations.push(EquationCandidate { result, terms });
    }

    Input { equations }
}

fn apply_operations(terms: &[i64]) -> Vec<i64> {
    // Define possible operations
    let op1 = |lhs: i64, rhs: i64| lhs + rhs;
    let op2 = |lhs: i64, rhs: i64| lhs * rhs;

    // Last term is always rhs
    let rhs = *terms.last().unwrap();

    match terms.len().cmp(&2) {
        Ordering::Equal => {
            // Final two terms. Pass it up the stack.
            let lhs = *terms.first().unwrap();
            let res1 = op1(lhs, rhs);
            let res2 = op2(lhs, rhs);
            vec![res1, res2]
        }
        Ordering::Greater => {
            // More than two terms. Apply all possible results from the left.
            let mut answers: Vec<i64> = Vec::new();
            let lhs_candidates = apply_operations(&terms[..terms.len() - 1]);
            for lhs in lhs_candidates {
                answers.push(op1(lhs, rhs));
                answers.push(op2(lhs, rhs));
            }
            answers
        }
        Ordering::Less => {
            panic!("Unexpected terms list of less than two items.");
        }
    }
}

fn check_equation(eq: &EquationCandidate) -> Option<i64> {
    // Find all possible answers
    let answers = apply_operations(&eq.terms);

    // See if any of them match the desired answer
    let can_be_solved = answers.iter().any(|&answer| answer == eq.result);

    if can_be_solved {
        Some(eq.result)
    } else {
        None
    }
}

fn main() {
    let path = get_single_path_as_arg();
    let raw_input = std::fs::read_to_string(path).expect("Failed to read input as string.");
    let input = read_input(&raw_input);

    let sum_solvable_equations: i64 = input.equations.iter().filter_map(check_equation).sum();

    println!(
        "The answer to the first half is: {}",
        sum_solvable_equations
    );
}
