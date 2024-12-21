use aoc_2024::get_single_path_as_arg;

struct Input {
    dense_layout: Vec<u8>,
}

fn read_input(raw_input: &str) -> Input {
    Input {
        dense_layout: raw_input
            .chars()
            .map(|ch| ch.to_digit(10).unwrap().try_into().unwrap())
            .collect(),
    }
}

enum BlockType {
    File,
    Free,
}

struct Disk {
    blocks: Vec<Option<u32>>,
}

impl Disk {
    fn move_block(&mut self, src_idx: usize, dst_idx: usize) -> Result<(), String> {
        // Src must be a file block
        let src_value = match self.blocks.get(src_idx) {
            Some(optval) => match optval {
                Some(val) => val,
                None => return Err("Cannot move from block. Block is already empty.".to_string()),
            },
            None => return Err("Index out of bounds".to_string()),
        };
        // Dst must be an empty block
        if let Some(optval) = self.blocks.get(dst_idx) {
            if optval.is_some() {
                return Err("Cannot move to a non-empty block".to_string());
            }
        } else {
            return Err("Index out of bounds".to_string());
        }

        // Perform the move
        self.blocks[dst_idx] = Some(*src_value);
        self.blocks[src_idx] = None;

        Ok(())
    }

    fn idx_last_used_block(&self) -> Option<usize> {
        for (inv_idx, optval) in self.blocks.iter().rev().enumerate() {
            if optval.is_some() {
                return Some((self.blocks.len().checked_sub(inv_idx + 1)).unwrap());
            }
        }
        None
    }

    fn idx_first_empty_block(&self) -> Option<usize> {
        for (idx, optval) in self.blocks.iter().enumerate() {
            match optval {
                Some(_) => {}
                None => return Some(idx),
            }
        }
        None
    }

    fn compute_checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(idx, optval)| idx * optval.unwrap_or(0) as usize)
            .sum()
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for optval in &self.blocks {
            match optval {
                Some(val) => write!(f, "|{}", val).unwrap(),
                None => write!(f, "|.").unwrap(),
            }
        }
        Ok(())
    }
}

fn decompress(input: Input) -> Disk {
    let mut file_id = 0_u32;
    let mut block_type = BlockType::File;
    let mut full_layout: Vec<Option<u32>> = Vec::new();
    for num in input.dense_layout {
        match block_type {
            BlockType::File => {
                let new_block = std::iter::repeat(file_id).take(num as usize);
                full_layout.extend(new_block.into_iter().map(Some));
                file_id += 1;
                block_type = BlockType::Free;
            }
            BlockType::Free => {
                let new_block = std::iter::repeat(None).take(num as usize);
                full_layout.extend(new_block);
                block_type = BlockType::File;
            }
        }
    }
    Disk {
        blocks: full_layout,
    }
}

fn main() {
    let path = get_single_path_as_arg();
    let raw_input = std::fs::read_to_string(path).expect("Failed to read input as string.");

    let input = read_input(&raw_input);

    let mut disk = decompress(input);

    while let Some((src_idx, dst_idx)) = {
        let src_idx = disk.idx_last_used_block().unwrap();
        let dst_idx = disk.idx_first_empty_block().unwrap();
        if dst_idx < src_idx {
            Some((src_idx, dst_idx))
        } else {
            None
        }
    } {
        disk.move_block(src_idx, dst_idx).unwrap();
    }

    println!(
        "The answer to the first half is: {}",
        disk.compute_checksum()
    );
}
