use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use split_digits::SplitDigitIterator;
use std::collections::VecDeque;

fn split_digits_vec_deque(d: usize) -> Vec<u8> {
    let mut digits = VecDeque::new();
    let mut n = d;
    while n > 0 {
        digits.push_front((n % 10) as u8);
        n /= 10;
    }

    digits.into()
}

fn split_digits_vec(d: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = d;
    while n > 0 {
        digits.insert(0, (n % 10) as u8);
        n /= 10;
    }

    digits
}

fn split_digits_vec_push_reverse(d: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = d;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    digits.reverse();

    digits
}

fn split_digits_iterator(d: usize) -> Vec<u8> {
    SplitDigitIterator::new(d).collect()
}

fn split_digits_string(d: usize) -> Vec<u8> {
    d.to_string()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect()
}

fn benchmark_split_digits(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_digits");
    for (d, s) in [
        (1, 1),
        (12, 2),
        (123, 3),
        (1234, 4),
        (12345, 5),
        (123456, 6),
        (1234567, 7),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("split_digits_vec_deque", s), d, |b, d| {
            b.iter(|| split_digits_vec_deque(black_box(*d)))
        });
        group.bench_with_input(BenchmarkId::new("split_digits_vec", s), d, |b, d| {
            b.iter(|| split_digits_vec(black_box(*d)))
        });
        group.bench_with_input(
            BenchmarkId::new("split_digits_vec_push_reverse", s),
            d,
            |b, d| b.iter(|| split_digits_vec_push_reverse(black_box(*d))),
        );
        group.bench_with_input(BenchmarkId::new("split_digits_iterator", s), d, |b, d| {
            b.iter(|| split_digits_iterator(black_box(*d)))
        });
        group.bench_with_input(BenchmarkId::new("split_digits_string", s), d, |b, d| {
            b.iter(|| split_digits_string(black_box(*d)))
        });
    }

    group.finish()
}

criterion_group!(benches, benchmark_split_digits);
criterion_main!(benches);
