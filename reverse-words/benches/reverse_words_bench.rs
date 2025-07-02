use criterion::{Criterion, criterion_group, criterion_main};
use reverse_words::{reverse_words_fast, reverse_words_slow};
use std::hint::black_box;

fn bench_reverse_words(c: &mut Criterion) {
    let text = "This sentence contains several words that may or may not be reversed depending on their length This sentence contains several words that may";

    c.bench_function("reverse_words_fast", |b| {
        b.iter(|| reverse_words_fast(black_box(text)))
    });

    c.bench_function("reverse_words_slow", |b| {
        b.iter(|| reverse_words_slow(black_box(text)))
    });
}

criterion_group!(benches, bench_reverse_words);
criterion_main!(benches);
