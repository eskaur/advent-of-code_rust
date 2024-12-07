use aoc_2024::get_single_path_as_arg;

struct Array2D<T: Clone> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2D<T> {
    fn new(height: usize, width: usize, init_value: T) -> Array2D<T> {
        let size: usize = height.checked_mul(width).unwrap();

        let data: Vec<T> = std::iter::repeat(init_value).take(size).collect();
        Array2D {
            height,
            width,
            data,
        }
    }

    fn idx1d(&self, row: i64, col: i64) -> Result<usize, String> {
        if row >= 0 && row < self.height as i64 && col >= 0 && col < self.width as i64 {
            let idx: usize = (row * self.width as i64 + col).try_into().unwrap();
            Ok(idx)
        } else {
            Err(format!(
                "Index [{},{}] incompatible with size [0-{},0-{}]",
                row, col, self.height, self.width
            ))
        }
    }

    fn set(&mut self, row: i64, col: i64, value: T) -> Result<(), String> {
        let idx = self.idx1d(row, col)?;
        self.data[idx] = value;
        Ok(())
    }

    fn get(&self, row: i64, col: i64) -> Result<T, String> {
        let idx = self.idx1d(row, col)?;
        Ok(self.data[idx].clone())
    }
}

fn check_match(sequence: &[u8], pattern: &str) -> bool {
    std::str::from_utf8(sequence).unwrap() == pattern
}

fn main() {
    // Read input
    let path = get_single_path_as_arg();
    let input = std::fs::read_to_string(path).expect("Failed to read input as string.");
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.first().unwrap().len();
    let height = lines.len();

    // Fill input into array
    let mut array = Array2D::new(height, width, 0_u8);
    for (row, line) in lines.iter().enumerate() {
        for (col, character) in line.as_bytes().iter().enumerate() {
            array.set(row as i64, col as i64, *character).unwrap();
        }
    }

    // Look for matches
    let mut match_counter = 0;
    let pattern = "XMAS".to_string();
    for row in 0..array.height as i64 {
        for col in 0..array.width as i64 {
            let forward: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row, col + i).unwrap_or(b' '))
                .collect();

            let backward: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row, col - i).unwrap_or(b' '))
                .collect();

            let down: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row + i, col).unwrap_or(b' '))
                .collect();

            let up: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row - i, col).unwrap_or(b' '))
                .collect();

            let forward_down: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row + i, col + i).unwrap_or(b' '))
                .collect();

            let forward_up: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row - i, col + i).unwrap_or(b' '))
                .collect();

            let backward_up: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row - i, col - i).unwrap_or(b' '))
                .collect();

            let backward_down: Vec<u8> = (0..pattern.len() as i64)
                .map(|i| array.get(row + i, col - i).unwrap_or(b' '))
                .collect();

            let sequences = vec![
                forward,
                backward,
                down,
                up,
                forward_down,
                forward_up,
                backward_up,
                backward_down,
            ];

            match_counter += sequences
                .into_iter()
                .filter(|sequence| check_match(sequence, &pattern))
                .count();
        }
    }

    println!("The answer to the first half is: {}", match_counter);
}
