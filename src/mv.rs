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
