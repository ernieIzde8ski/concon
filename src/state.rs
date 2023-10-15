use core::fmt;
use std::{error, fs::File, io::Read};

pub struct State {
    grid: [[bool; 64]; 64],
}

const TOTAL_ROWS: usize = 64;
const TOTAL_COLS: usize = 64;

#[derive(Debug)]
pub struct GridParseError(String, (usize, usize));
impl fmt::Display for GridParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GridParseError: {} at {},{}",
            self.0, self.1 .0, self.1 .1
        )
    }
}
impl error::Error for GridParseError {}

impl TryFrom<String> for State {
    type Error = GridParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut grid = [[false; TOTAL_COLS]; TOTAL_ROWS];

        let mut row = 0;
        let mut col = 0;

        for ch in value.chars() {
            match ch {
                '\n' => {
                    row += 1;
                    col = 0;
                }
                '0' => col += 1,
                '1' => {
                    if row >= TOTAL_ROWS {
                        return Err(GridParseError("too many rows!".into(), (row, col)));
                    } else if col >= TOTAL_COLS {
                        return Err(GridParseError("too many cols!".into(), (row, col)));
                    };

                    grid[row][col] = true;
                    col += 1;
                }
                _ => {
                    return Err(GridParseError(
                        format!("unexpected char: '{ch}'"),
                        (row, col),
                    ))
                }
            }
        }

        Ok(Self { grid })
    }
}


impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "╔{}╗", str::repeat("═", TOTAL_COLS))?;
        for row in self.grid {
            write!(f, "║")?;
            for col in row {
                match col {
                    true => write!(f, "█")?,
                    false => write!(f, " ")?,
                };
            }
            write!(f, "║\n")?;
        }
        write!(f, "╚{}╝", str::repeat("═", TOTAL_COLS))
    }
}

impl State {
    pub fn load(fp: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut fc = String::new();
        File::open(fp)?.read_to_string(&mut fc)?;
        Ok(Self::try_from(fc)?)
    }
}

impl State {
    /// Gets total number of live neighbors next to a grid point.
    fn check_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut total_live_neighbors = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let r = match (i, row) {
                    (-1, 0) => TOTAL_ROWS - 1,
                    (-1, cur) => cur - 1,
                    (1, mut cur) => {
                        cur += 1;
                        if cur == TOTAL_ROWS {
                            0
                        } else {
                            cur
                        }
                    }
                    (_, cur) => cur,
                };

                let c = match (j, col) {
                    (-1, 0) => TOTAL_COLS - 1,
                    (-1, cur) => cur - 1,
                    (1, mut cur) => {
                        cur += 1;
                        if cur == TOTAL_COLS {
                            0
                        } else {
                            cur
                        }
                    }
                    (_, cur) => cur,
                };

                if self.grid[r][c] {
                    total_live_neighbors += 1;
                }
            }
        }
        total_live_neighbors
    }

    /// Advances the grid by one cycle.
    pub fn advance(&mut self) {
        let mut next_grid = [[false; TOTAL_COLS]; TOTAL_ROWS];

        for r in 0..TOTAL_ROWS {
            for c in 0..TOTAL_COLS {
                next_grid[r][c] = match (self.grid[r][c], self.check_neighbors(r, c)) {
                    // Survival
                    (true, 2 | 3) => true,
                    // Birth
                    (false, 3) => true,
                    // No birth, or death by underpopulation or overpopulation
                    (..) => false,
                };
            }
        }

        self.grid = next_grid;
    }
}
