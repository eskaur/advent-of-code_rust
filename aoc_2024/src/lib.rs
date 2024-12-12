use std::fmt::Display;

pub fn get_single_path_as_arg() -> std::path::PathBuf {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected exactly one argument.")
    }
    std::path::PathBuf::from(args.get(1).expect("Failed to get first argument"))
        .canonicalize()
        .expect("Failed to parse path")
}

pub struct Array2D<T: Copy> {
    pub height: usize,
    pub width: usize,
    data: Vec<T>,
}

impl<T: Copy> Array2D<T> {
    pub fn new(height: usize, width: usize, init_value: T) -> Array2D<T> {
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

    pub fn set(&mut self, row: i64, col: i64, value: T) -> Result<(), String> {
        let idx = self.idx1d(row, col)?;
        self.data[idx] = value;
        Ok(())
    }

    pub fn get(&self, row: i64, col: i64) -> Result<T, String> {
        let idx = self.idx1d(row, col)?;
        Ok(self.data[idx])
    }
}

impl<T: Copy + Display> std::fmt::Display for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..self.height as i64 {
            for col in 0..self.width as i64 {
                write!(f, "{}", self.get(row, col).unwrap()).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
