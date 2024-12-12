use aoc_2024::get_single_path_as_arg;
use aoc_2024::Array2D;
use std::collections::HashSet;

fn read_input(raw_input: &str) -> Array2D<char> {
    let height = raw_input.lines().count();
    let width = raw_input.lines().next().unwrap().len();
    let mut array2d = Array2D::new(height, width, 0 as char);

    for (row, line) in raw_input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            array2d.set(row as i64, col as i64, ch).unwrap();
        }
    }

    array2d
}

fn find_antinodes_for_node(
    node_row: i64,
    node_col: i64,
    array2d: &Array2D<char>,
) -> Vec<(i64, i64)> {
    let mut antinodes: Vec<(i64, i64)> = Vec::new();

    let node = array2d.get(node_row, node_col).unwrap();

    for row in 0..array2d.height as i64 {
        for col in 0..array2d.width as i64 {
            if row != node_row && col != node_col && array2d.get(row, col).unwrap() == node {
                // Found a matching antenna
                let dx = col - node_col;
                let dy = row - node_row;

                let antinode_col = col + dx;
                let antinode_row = row + dy;

                if array2d.get(antinode_row, antinode_col).is_ok() {
                    // Antinode is within the map
                    antinodes.push((antinode_row, antinode_col));
                }
            }
        }
    }
    antinodes
}

fn main() {
    let path = get_single_path_as_arg();
    let raw_input = std::fs::read_to_string(path).expect("Failed to read input as string.");
    let array2d = read_input(&raw_input);
    println!("{}", array2d);

    let mut antinodes: Vec<(i64, i64)> = Vec::new();

    for row in 0..array2d.height as i64 {
        for col in 0..array2d.width as i64 {
            let node = array2d.get(row, col).unwrap();
            if node.is_alphanumeric() {
                antinodes.extend(find_antinodes_for_node(row, col, &array2d));
            }
        }
    }

    for antinode in &antinodes {
        println!("{},{}", antinode.0, antinode.1);
    }

    let num_unique_antinodes = HashSet::<_>::from_iter(antinodes).len();

    println!("The answer to the first half is: {}", num_unique_antinodes);
}
