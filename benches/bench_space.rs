use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chess::ai::{self, TranspositionTable, TTFlag};
use chess::{Board, Game, Move};

mod helpers;
use helpers::*;

fn bench_size_of_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("size_of_types");
    group.sample_size(10);
    group.bench_function("Board", |b| {
        b.iter(|| black_box(std::mem::size_of::<Board>()));
    });
    group.bench_function("Game", |b| {
        b.iter(|| black_box(std::mem::size_of::<Game>()));
    });
    group.bench_function("Move", |b| {
        b.iter(|| black_box(std::mem::size_of::<Move>()));
    });
    group.bench_function("Option<Move>", |b| {
        b.iter(|| black_box(std::mem::size_of::<Option<Move>>()));
    });
    group.bench_function("TranspositionTable", |b| {
        b.iter(|| black_box(std::mem::size_of::<TranspositionTable>()));
    });
    group.finish();
}

fn bench_board_clone(c: &mut Criterion) {
    let board = game_from(FEN_INITIAL).board().clone();
    c.bench_function("board_clone/initial", |b| {
        b.iter(|| black_box(board.clone()))
    });

    let board2 = game_from(FEN_KIWIPETE).board().clone();
    c.bench_function("board_clone/kiwipete", |b| {
        b.iter(|| black_box(board2.clone()))
    });
}

fn bench_game_clone(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("game_clone/initial", |b| {
        b.iter(|| black_box(game.clone()))
    });

    let game2 = game_from(FEN_KIWIPETE);
    c.bench_function("game_clone/kiwipete", |b| {
        b.iter(|| black_box(game2.clone()))
    });
}

fn bench_compute_hash(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("compute_hash/initial", |b| {
        b.iter(|| black_box(ai::compute_hash(black_box(&game))))
    });

    let game2 = game_from(FEN_KIWIPETE);
    c.bench_function("compute_hash/kiwipete", |b| {
        b.iter(|| black_box(ai::compute_hash(black_box(&game2))))
    });

    let game3 = game_from(FEN_ENDGAME);
    c.bench_function("compute_hash/endgame", |b| {
        b.iter(|| black_box(ai::compute_hash(black_box(&game3))))
    });
}

fn bench_tt_probe(c: &mut Criterion) {
    let mut tt = TranspositionTable::new();
    let hash: u64 = 0xDEAD_BEEF_CAFE_F00D;
    tt.record(hash, 4, 50, TTFlag::Exact, None);

    c.bench_function("tt_probe/miss", |b| {
        b.iter(|| {
            black_box(tt.probe(black_box(0x1111_2222_3333_4444), 1, -100_000, 100_000))
        })
    });

    c.bench_function("tt_probe/hit", |b| {
        b.iter(|| black_box(tt.probe(black_box(hash), 1, -100_000, 100_000)))
    });
}

fn bench_tt_record(c: &mut Criterion) {
    let mut tt = TranspositionTable::new();
    c.bench_function("tt_record", |b| {
        b.iter(|| {
            let h: u64 = black_box(42);
            tt.record(h, 3, 100, TTFlag::Exact, None);
            black_box(());
        })
    });
}

fn bench_update_hash(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    let moves = chess::moves::legal_moves(game.board(), game.turn(), game.ep_target(), game.castling_rights());
    let mv = moves[0];
    let piece = game.board().piece_at(mv.from).unwrap();
    let hash = ai::compute_hash(&game);
    let old_ep = game.ep_target();
    let old_castling = *game.castling_rights();

    c.bench_function("update_hash/e4", |b| {
        b.iter(|| {
            let new_hash = ai::update_hash(
                black_box(hash),
                black_box(&mv),
                black_box(piece),
                black_box(None),
                black_box(old_ep),
                black_box(&old_castling),
                black_box(None),
                black_box(&old_castling),
            );
            black_box(new_hash);
        })
    });
}

fn bench_vec_move_alloc(c: &mut Criterion) {
    c.bench_function("vec_move_alloc/64", |b| {
        b.iter(|| {
            let v: Vec<Move> = Vec::with_capacity(64);
            black_box(v)
        })
    });

    let moves_sample: Vec<Move> = (0..40).map(|_i| {
        Move::new(sq("a1"), sq("b1"))
    }).collect();
    c.bench_function("vec_move_clone/40", |b| {
        b.iter(|| {
            let v = black_box(&moves_sample).clone();
            black_box(v.len());
        })
    });
}

criterion_group!(
    name = space;
    config = Criterion::default().significance_level(0.02).sample_size(100);
    targets =
        bench_size_of_types,
        bench_board_clone,
        bench_game_clone,
        bench_compute_hash,
        bench_tt_probe,
        bench_tt_record,
        bench_update_hash,
        bench_vec_move_alloc,
);
criterion_main!(space);
