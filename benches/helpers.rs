use chess::{Game, Square};

pub const FEN_INITIAL: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE: &str  = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
pub const FEN_MIDGAME: &str   = "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - -";
pub const FEN_ENDGAME: &str   = "4k3/8/8/3p4/8/8/4K3/8 w - -";
pub const FEN_CHECK: &str     = "rnb1kbnr/pppp1ppp/8/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -";

pub fn game_from(fen: &str) -> Game {
    Game::from_fen(fen).expect("valid FEN")
}



pub fn sq(alg: &str) -> Square {
    Square::from_algebraic(alg).unwrap()
}
