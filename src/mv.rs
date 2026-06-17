use crate::{PieceType, Square};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    pub fn new_promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }

    pub fn to_coordinate(&self) -> String {
        let promo = match self.promotion {
            Some(pt) => format!("={}", pt.to_char().to_ascii_uppercase()),
            None => String::new(),
        };
        format!(
            "{}{}{}",
            self.from.to_algebraic(),
            self.to.to_algebraic(),
            promo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Square;

    fn sq(alg: &str) -> Square {
        Square::from_algebraic(alg).unwrap()
    }

    #[test]
    fn test_move_new() {
        let mv = Move::new(sq("e2"), sq("e4"));
        assert_eq!(mv.from, sq("e2"));
        assert_eq!(mv.to, sq("e4"));
        assert_eq!(mv.promotion, None);
    }

    #[test]
    fn test_move_new_promotion() {
        let mv = Move::new_promotion(sq("e7"), sq("e8"), PieceType::Queen);
        assert_eq!(mv.from, sq("e7"));
        assert_eq!(mv.to, sq("e8"));
        assert_eq!(mv.promotion, Some(PieceType::Queen));
    }

    #[test]
    fn test_move_to_coordinate() {
        assert_eq!(Move::new(sq("e2"), sq("e4")).to_coordinate(), "e2e4");
        assert_eq!(Move::new(sq("g1"), sq("f3")).to_coordinate(), "g1f3");
        assert_eq!(Move::new_promotion(sq("e7"), sq("e8"), PieceType::Queen).to_coordinate(), "e7e8=Q");
    }

    #[test]
    fn test_move_equality() {
        let a = Move::new(sq("e2"), sq("e4"));
        let b = Move::new(sq("e2"), sq("e4"));
        let c = Move::new(sq("g1"), sq("f3"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_move_copy() {
        let a = Move::new(sq("e2"), sq("e4"));
        let b = a;
        assert_eq!(a, b);
    }
}
