use criterion::{black_box, criterion_group, Criterion};

use chess::ai;

mod helpers;
use helpers::*;

fn bench_best_move_easy(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("best_move/easy_depth_2", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(2));
            black_box(mv)
        })
    });
}

fn bench_best_move_medium(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("best_move/medium_depth_4", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(4));
            black_box(mv)
        })
    });
}

fn bench_best_move_hard(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("best_move/hard_depth_7", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(7));
            black_box(mv)
        })
    });
}

fn bench_best_move_hard14(c: &mut Criterion) {
    let game = game_from(FEN_INITIAL);
    c.bench_function("best_move/hard_depth_14", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(14));
            black_box(mv)
        })
    });
}

fn bench_best_move_kiwipete_easy(c: &mut Criterion) {
    let game = game_from(FEN_KIWIPETE);
    c.bench_function("best_move/kiwipete_easy_depth_2", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(2));
            black_box(mv)
        })
    });
}

fn bench_best_move_endgame_easy(c: &mut Criterion) {
    let game = game_from(FEN_ENDGAME);
    c.bench_function("best_move/endgame_easy_depth_2", |b| {
        b.iter(|| {
            let mv = ai::best_move_with_depth(black_box(&game), black_box(2));
            black_box(mv)
        })
    });
}

criterion_group!(
    name = time;
    config = Criterion::default().significance_level(0.02).sample_size(50).warm_up_time(std::time::Duration::from_secs(2));
    targets =
        bench_best_move_easy,
        bench_best_move_medium,
        bench_best_move_hard,
        bench_best_move_hard14,
        bench_best_move_kiwipete_easy,
        bench_best_move_endgame_easy,
);

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .name("criterion-main".into())
        .spawn(|| {
            time();
        })
        .unwrap()
        .join()
        .unwrap();
}
