use criterion::{criterion_group, criterion_main, Criterion};
use liquids_separation::separate_liquids;
use rand::seq::IndexedMutRandom;

fn generate_big_glass(n_rows: usize, width: usize) -> Vec<Vec<char>> {
    let mut symbols = ['H', 'W', 'A', 'O'];
    let mut rng = rand::rng();

    // Генерируем строки
    (0..n_rows)
        .map(|_| {
            (0..width)
                .map(|_| *symbols.choose_mut(&mut rng).unwrap())
                .collect()
        })
        .collect()
}

fn bench_large_glass(c: &mut Criterion) {
    let glass = generate_big_glass(100_000, 10);
    c.bench_function("separate_liquids large glass", |b| {
        b.iter(|| separate_liquids(&glass))
    });
}

criterion_group!(benches, bench_large_glass);
criterion_main!(benches);
