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
        for row in self.grid {
            for col in row {
                match col {
                    true => write!(f, "░")?,
                    false => write!(f, "█")?,
                };
            }
            println!();
        }

        Ok(())
    }
}

impl State {
    pub fn load(fp: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut fc = String::new();
        File::open(fp)?.read_to_string(&mut fc)?;
        Ok(Self::try_from(fc)?)
    }
}
