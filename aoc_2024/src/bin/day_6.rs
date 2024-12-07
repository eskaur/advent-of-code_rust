use aoc_2024::get_single_path_as_arg;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
struct VecXY<T> {
    x: T,
    y: T,
}

impl std::ops::Add for VecXY<i64> {
    type Output = VecXY<i64>;

    fn add(self, other: VecXY<i64>) -> VecXY<i64> {
        VecXY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Map {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

impl Map {
    fn new(raw_input: &str) -> Map {
        let lines: Vec<&str> = raw_input.lines().collect();

        let width = lines.first().unwrap().len();
        let height = lines.len();
        let mut data: Vec<u8> = Vec::new();

        for line in lines {
            for ch in line.as_bytes() {
                data.push(*ch);
            }
        }

        Map {
            height,
            width,
            data,
        }
    }

    fn idx1d(&self, pos: &VecXY<i64>) -> Result<usize, String> {
        let col = pos.x;
        let row = pos.y;
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

    fn get(&self, pos: &VecXY<i64>) -> Result<u8, String> {
        let idx = self.idx1d(pos)?;
        Ok(self.data[idx])
    }

    fn set(&mut self, pos: &VecXY<i64>, value: u8) -> Result<(), String> {
        let idx = self.idx1d(pos)?;
        self.data[idx] = value;
        Ok(())
    }

    fn find(&self, target: u8) -> Option<VecXY<i64>> {
        for row in 0..self.height as i64 {
            for col in 0..self.width as i64 {
                let pos = VecXY { x: col, y: row };
                if self.get(&pos).unwrap() == target {
                    return Some(pos);
                }
            }
        }
        None
    }
}

struct Guard {
    pos: VecXY<i64>,
    dir: VecXY<i64>,
}

struct Game {
    map: Map,
    guard: Guard,
    char_to_dir_map: HashMap<u8, VecXY<i64>>,
    positions_visited: Vec<VecXY<i64>>,
}

impl Game {
    // TODO(ESKIL): Don't store guard in map. Only store the static bits.
    fn new(raw_input: &str) -> Game {
        // Load initial map with all input including guard
        let mut map = Map::new(raw_input);

        // Set up mapping between guard char and direction
        let mut char_to_dir_map: HashMap<u8, VecXY<i64>> = HashMap::new();
        char_to_dir_map.insert(b'>', VecXY { x: 1, y: 0 });
        char_to_dir_map.insert(b'v', VecXY { x: 0, y: 1 });
        char_to_dir_map.insert(b'<', VecXY { x: -1, y: 0 });
        char_to_dir_map.insert(b'^', VecXY { x: 0, y: -1 });

        let guards_found: Vec<VecXY<i64>> = char_to_dir_map
            .keys()
            .filter_map(|key| map.find(*key))
            .collect();

        if guards_found.len() != 1 {
            panic!("Expected exactly one matching guard character in raw input.");
        }
        let guard_initial_pos = guards_found.first().unwrap();

        // Find initial orientation of guard
        let guard_char = map.get(guard_initial_pos).unwrap();
        let guard_initial_dir = char_to_dir_map.get(&guard_char).unwrap();

        // Create guard
        let guard = Guard {
            pos: *guard_initial_pos,
            dir: *guard_initial_dir,
        };

        // Remove guard from map so it represents the static part
        map.set(guard_initial_pos, b'.').unwrap();

        Game {
            map,
            guard,
            char_to_dir_map,
            positions_visited: Vec::new(),
        }
    }

    fn update(&mut self) -> bool {
        // Register current position
        if !self
            .positions_visited
            .iter()
            .any(|pos| *pos == self.guard.pos)
        {
            self.positions_visited.push(self.guard.pos);
        }

        // Attempt move
        let new_guard_pos = self.guard.pos + self.guard.dir;
        let new_pos_obj = self.map.get(&new_guard_pos);
        if new_pos_obj.is_err() {
            // Moved outside of map
            return false;
        }
        let new_pos_obj = new_pos_obj.unwrap();

        if new_pos_obj == b'#' {
            // Occuped: Turn clockwise
            self.guard.dir = VecXY {
                x: -self.guard.dir.y,
                y: self.guard.dir.x,
            };
        } else if new_pos_obj == b'.' {
            // Unoccupied: Move forward
            self.guard.pos = new_guard_pos;
        } else {
            panic!("Unexpected symbol on map")
        }

        true
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.map.height as i64 {
            for col in 0..self.map.width as i64 {
                let pos = VecXY { x: col, y: row };

                if pos == self.guard.pos {
                    let guard_char = self
                        .char_to_dir_map
                        .iter()
                        .find(|(_, &val)| val == self.guard.dir)
                        .map(|(key, _)| key)
                        .unwrap();

                    write!(f, "{}", *guard_char as char).unwrap();
                } else {
                    write!(f, "{}", self.map.get(&pos).unwrap() as char).unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

fn main() {
    let path = get_single_path_as_arg();
    let raw_input = std::fs::read_to_string(path).expect("Failed to read input as string.");

    let mut game = Game::new(&raw_input);
    println!("{}", game);

    while game.update() {
        game.update();
    }

    println!(
        "The answer to the first half is: {}",
        game.positions_visited.len()
    );
}
