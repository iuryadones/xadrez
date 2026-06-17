use std::fmt;
use std::str::FromStr;

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
        if (0..8).contains(&f) && (0..8).contains(&r) {
            Some(Self {
                file: f as usize,
                rank: r as usize,
            })
        } else {
            None
        }
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_algebraic(s).ok_or_else(|| format!("Casa invalida: {}", s))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new_valid() {
        let sq = Square::new(0, 0).unwrap();
        assert_eq!(sq.file, 0);
        assert_eq!(sq.rank, 0);
    }

    #[test]
    fn test_square_new_invalid() {
        assert!(Square::new(8, 0).is_none());
        assert!(Square::new(0, 8).is_none());
        assert!(Square::new(8, 8).is_none());
    }

    #[test]
    fn test_square_from_algebraic() {
        let sq = Square::from_algebraic("e4").unwrap();
        assert_eq!(sq.file, 4);
        assert_eq!(sq.rank, 3);
        assert_eq!(Square::from_algebraic("a1").unwrap().file, 0);
        assert_eq!(Square::from_algebraic("a1").unwrap().rank, 0);
        assert_eq!(Square::from_algebraic("h8").unwrap().file, 7);
        assert_eq!(Square::from_algebraic("h8").unwrap().rank, 7);
    }

    #[test]
    fn test_square_from_algebraic_invalid() {
        assert!(Square::from_algebraic("i1").is_none());
        assert!(Square::from_algebraic("a9").is_none());
        assert!(Square::from_algebraic("foo").is_none());
        assert!(Square::from_algebraic("").is_none());
    }

    #[test]
    fn test_square_to_algebraic() {
        assert_eq!(Square::new_unchecked(0, 0).to_algebraic(), "a1");
        assert_eq!(Square::new_unchecked(7, 7).to_algebraic(), "h8");
        assert_eq!(Square::new_unchecked(4, 3).to_algebraic(), "e4");
    }

    #[test]
    fn test_square_offset() {
        let sq = Square::from_algebraic("e4").unwrap();
        assert_eq!(sq.offset(1, 1).map(|s| s.to_algebraic()).as_deref(), Some("f5"));
        assert_eq!(sq.offset(-1, -1).map(|s| s.to_algebraic()).as_deref(), Some("d3"));
        assert_eq!(sq.offset(10, 0), None);
        assert_eq!(sq.offset(0, -10), None);
    }

    #[test]
    fn test_square_is_valid() {
        assert!(Square::new_unchecked(0, 0).is_valid());
        assert!(Square::new_unchecked(7, 7).is_valid());
        assert!(!Square::new_unchecked(8, 0).is_valid());
        assert!(!Square::new_unchecked(0, 8).is_valid());
    }

    #[test]
    fn test_square_display() {
        assert_eq!(format!("{}", Square::from_algebraic("e4").unwrap()), "e4");
        assert_eq!(format!("{}", Square::from_algebraic("a1").unwrap()), "a1");
    }
}
