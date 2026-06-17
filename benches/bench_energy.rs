use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chess::ai::{self};
use chess::{Color, PieceType};

mod helpers;
use helpers::*;

fn bench_evaluate(c: &mut Criterion) {
    let g1 = game_from(FEN_INITIAL);
    c.bench_function("evaluate/initial", |b| {
        b.iter(|| black_box(ai::evaluate(black_box(&g1))))
    });

    let g2 = game_from(FEN_KIWIPETE);
    c.bench_function("evaluate/kiwipete", |b| {
        b.iter(|| black_box(ai::evaluate(black_box(&g2))))
    });

    let g3 = game_from(FEN_ENDGAME);
    c.bench_function("evaluate/endgame", |b| {
        b.iter(|| black_box(ai::evaluate(black_box(&g3))))
    });
}

fn bench_pseudo_legal_moves(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    c.bench_function("pseudo_legal/initial", |b| {
        b.iter(|| {
            let moves = chess::moves::pseudo_legal_moves(
                black_box(g.board()),
                black_box(g.turn()),
                black_box(g.ep_target()),
                black_box(g.castling_rights()),
            );
            black_box(moves.len())
        })
    });

    let g2 = game_from(FEN_KIWIPETE);
    c.bench_function("pseudo_legal/kiwipete", |b| {
        b.iter(|| {
            let moves = chess::moves::pseudo_legal_moves(
                black_box(g2.board()),
                black_box(g2.turn()),
                black_box(g2.ep_target()),
                black_box(g2.castling_rights()),
            );
            black_box(moves.len())
        })
    });

    let g3 = game_from(FEN_CHECK);
    c.bench_function("pseudo_legal/check", |b| {
        b.iter(|| {
            let moves = chess::moves::pseudo_legal_moves(
                black_box(g3.board()),
                black_box(g3.turn()),
                black_box(g3.ep_target()),
                black_box(g3.castling_rights()),
            );
            black_box(moves.len())
        })
    });
}

fn bench_legal_moves(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    c.bench_function("legal_moves/initial", |b| {
        b.iter(|| {
            let moves = chess::moves::legal_moves(
                black_box(g.board()),
                black_box(g.turn()),
                black_box(g.ep_target()),
                black_box(g.castling_rights()),
            );
            black_box(moves.len())
        })
    });

    let g2 = game_from(FEN_KIWIPETE);
    c.bench_function("legal_moves/kiwipete", |b| {
        b.iter(|| {
            let moves = chess::moves::legal_moves(
                black_box(g2.board()),
                black_box(g2.turn()),
                black_box(g2.ep_target()),
                black_box(g2.castling_rights()),
            );
            black_box(moves.len())
        })
    });
}

fn bench_capture_moves(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    c.bench_function("capture_moves/initial", |b| {
        b.iter(|| {
            let moves = chess::moves::capture_moves(
                black_box(g.board()),
                black_box(g.turn()),
                black_box(g.ep_target()),
                black_box(g.castling_rights()),
            );
            black_box(moves.len())
        })
    });

    let g2 = game_from(FEN_KIWIPETE);
    c.bench_function("capture_moves/kiwipete", |b| {
        b.iter(|| {
            let moves = chess::moves::capture_moves(
                black_box(g2.board()),
                black_box(g2.turn()),
                black_box(g2.ep_target()),
                black_box(g2.castling_rights()),
            );
            black_box(moves.len())
        })
    });
}

fn bench_is_square_attacked(c: &mut Criterion) {
    let board = game_from(FEN_MIDGAME).board().clone();
    let center = sq("d5");
    let corner = sq("a1");
    let edge = sq("a4");

    c.bench_function("is_attacked/center", |b| {
        b.iter(|| {
            black_box(chess::moves::is_square_attacked(
                black_box(&board),
                black_box(center),
                black_box(Color::Black),
            ))
        })
    });

    c.bench_function("is_attacked/corner", |b| {
        b.iter(|| {
            black_box(chess::moves::is_square_attacked(
                black_box(&board),
                black_box(corner),
                black_box(Color::Black),
            ))
        })
    });

    c.bench_function("is_attacked/edge", |b| {
        b.iter(|| {
            black_box(chess::moves::is_square_attacked(
                black_box(&board),
                black_box(edge),
                black_box(Color::Black),
            ))
        })
    });
}

fn bench_is_legal(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    let moves = chess::moves::legal_moves(g.board(), g.turn(), g.ep_target(), g.castling_rights());
    let mv_legal = moves[0];

    c.bench_function("is_legal/legal_pawn_push", |b| {
        b.iter(|| {
            black_box(chess::moves::is_legal(
                black_box(g.board()),
                black_box(&mv_legal),
                black_box(g.turn()),
            ))
        })
    });
}

fn bench_order_moves(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    let moves = chess::moves::legal_moves(g.board(), g.turn(), g.ep_target(), g.castling_rights());
    c.bench_function("order_moves/initial", |b| {
        b.iter(|| {
            let mut m = black_box(&moves).clone();
            ai::order_moves(black_box(&g), black_box(&mut m), black_box(false), &[None; 2], &[[0; 64]; 6]);
            black_box(m.len());
        })
    });

    let g2 = game_from(FEN_KIWIPETE);
    let moves2 = chess::moves::legal_moves(g2.board(), g2.turn(), g2.ep_target(), g2.castling_rights());
    c.bench_function("order_moves/kiwipete", |b| {
        b.iter(|| {
            let mut m = black_box(&moves2).clone();
            ai::order_moves(black_box(&g2), black_box(&mut m), black_box(false), &[None; 2], &[[0; 64]; 6]);
            black_box(m.len());
        })
    });
}

fn bench_pst_bonus(c: &mut Criterion) {
    c.bench_function("pst_bonus/queen_center", |b| {
        b.iter(|| {
            black_box(ai::pst_bonus(
                black_box(PieceType::Queen),
                black_box(sq("d4")),
                black_box(Color::White),
            ))
        })
    });
}

fn bench_piece_value(c: &mut Criterion) {
    c.bench_function("piece_value/all_types", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for pt in &[PieceType::Pawn, PieceType::Knight, PieceType::Bishop,
                        PieceType::Rook, PieceType::Queen, PieceType::King] {
                sum += ai::piece_value(black_box(*pt));
            }
            black_box(sum);
        })
    });
}

fn bench_perft_depth_4(c: &mut Criterion) {
    let g = game_from(FEN_INITIAL);
    let board = g.board().clone();
    c.bench_function("perft/depth_4_initial", |b| {
        b.iter(|| {
            let count = chess::moves::perft(
                black_box(&board),
                black_box(4),
                black_box(Color::White),
                black_box(None),
                black_box(g.castling_rights()),
            );
            black_box(count);
        })
    });

    let g2 = game_from(FEN_KIWIPETE);
    let board2 = g2.board().clone();
    c.bench_function("perft/depth_4_kiwipete", |b| {
        b.iter(|| {
            let count = chess::moves::perft(
                black_box(&board2),
                black_box(4),
                black_box(g2.turn()),
                black_box(g2.ep_target()),
                black_box(g2.castling_rights()),
            );
            black_box(count);
        })
    });
}

criterion_group!(
    name = energy;
    config = Criterion::default().significance_level(0.02).sample_size(100);
    targets =
        bench_evaluate,
        bench_pseudo_legal_moves,
        bench_legal_moves,
        bench_capture_moves,
        bench_is_square_attacked,
        bench_is_legal,
        bench_order_moves,
        bench_pst_bonus,
        bench_piece_value,
        bench_perft_depth_4,
);
criterion_main!(energy);
