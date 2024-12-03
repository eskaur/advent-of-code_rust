use aoc_2024::get_single_path_as_arg;

struct ParsedMulStatement {
    lhs: u32,
    rhs: u32,
}

impl ParsedMulStatement {
    fn execute(&self) -> u32 {
        self.lhs * self.rhs
    }
}

fn main() {
    // Read input
    let path = get_single_path_as_arg();
    let input = std::fs::read_to_string(path).expect("Failed to read input as string.");

    // Locate all valid statements in input
    // Yes I could use regexp but what would be too easy wouldn't it?
    let pattern: Vec<u8> = vec![b'm', b'u', b'l', b'(', b'X', b',', b'X', b')'];
    let imax = pattern.len();
    let mut inext = 0;

    let mut statement_end_indices: Vec<usize> = Vec::new();

    for (ichar, char) in input.bytes().enumerate() {
        if inext == imax {
            // Done with sequence. Resetting.
            inext = 0;
            statement_end_indices.push(ichar - 1);
        }

        let next = pattern[inext];

        if next == b'X' {
            if (b'0'..=b'9').contains(&char) {
                // Still parsing a number
                continue;
            } else if char == pattern[inext + 1] {
                // Done with number. Char matches pattern after end of number.
                inext += 2;
            } else {
                // Got something that is neither number not what's expected after it.
                inext = 0;
            }
        } else if char == pattern[inext] {
            // Continuing sequence
            inext += 1;
        } else {
            // Sequence failed. Resetting.
            inext = 0;
        }
    }

    // Parse the valid statements found
    let mut statements: Vec<ParsedMulStatement> = Vec::new();
    for idx in statement_end_indices {
        let last = input.as_bytes()[idx];
        assert!(last == b')');

        // Look for rhs number
        let mut rhs_digits: Vec<u8> = Vec::new();
        for prev in input.as_bytes()[idx - 3..idx].iter().rev() {
            if *prev == b',' {
                break;
            }
            rhs_digits.push(*prev);
        }
        let rhs_num_digits = rhs_digits.len();
        let rhs_string = String::from_utf8(rhs_digits.into_iter().rev().collect()).unwrap();
        let rhs_number: u32 = rhs_string.parse().unwrap();

        // Look for lhs number
        let mut lhs_digits: Vec<u8> = Vec::new();
        let comma_idx = idx - rhs_num_digits - 1;
        assert!(input.as_bytes()[comma_idx] == b',');

        for prev in input.as_bytes()[comma_idx - 3..comma_idx].iter().rev() {
            if *prev == b'(' {
                break;
            }
            lhs_digits.push(*prev);
        }
        let lhs_string = String::from_utf8(lhs_digits.into_iter().rev().collect()).unwrap();
        let lhs_number: u32 = lhs_string.parse().unwrap();

        statements.push(ParsedMulStatement {
            lhs: lhs_number,
            rhs: rhs_number,
        });
    }

    // Execute and sum all the statements
    let sum_of_statements: u32 = statements.iter().map(|statement| statement.execute()).sum();
    println!("The answer to the first half is: {}", sum_of_statements);
}
