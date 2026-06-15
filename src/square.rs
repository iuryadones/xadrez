use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
}

impl Square {
    pub fn new(file: usize, rank: usize) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }

    pub fn new_unchecked(file: usize, rank: usize) -> Self {
        Self { file, rank }
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let file = (bytes[0].to_ascii_lowercase() - b'a') as usize;
        let rank = (bytes[1] - b'1') as usize;
        if file < 8 && rank < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }

    pub fn to_algebraic(self) -> String {
        let file = (b'a' + self.file as u8) as char;
        let rank = (b'1' + self.rank as u8) as char;
        format!("{}{}", file, rank)
    }

    pub fn is_valid(self) -> bool {
        self.file < 8 && self.rank < 8
    }

    pub fn offset(self, df: isize, dr: isize) -> Option<Self> {
        let f = self.file as isize + df;
        let r = self.rank as isize + dr;
        if f >= 0 && f < 8 && r >= 0 && r < 8 {
            Some(Self {
                file: f as usize,
                rank: r as usize,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}
