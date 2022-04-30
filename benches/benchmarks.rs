use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::VecDeque;

struct DigitAndPosition {
    digit: u8,
    position: u32,
}

fn get_most_significant_digit_and_position(d: usize) -> DigitAndPosition {
    let mut d = d;
    let mut position = 0;
    let mut digit: u8 = 0;

    while d > 0 {
        digit = (d % 10) as u8;
        d /= 10;
        position += 1;
    }

    DigitAndPosition { digit, position }
}

struct SplitDigitIterator {
    num: usize,
    trailing_zeroes: u32,
}

impl SplitDigitIterator {
    pub fn new(num: usize) -> Self {
        Self {
            num,
            trailing_zeroes: 0,
        }
    }
}

impl Iterator for SplitDigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num != 0 {
            let dap = get_most_significant_digit_and_position(self.num);
            let digit = dap.digit;
            let sub = 10_usize.pow(dap.position) * dap.digit as usize;
            self.trailing_zeroes = dap.position;
            self.num -= sub;
            Some(digit)
        } else {
            if self.trailing_zeroes > 0 {
                self.trailing_zeroes -= 1;
                return Some(0);
            }

            None
        }
    }
}

impl DoubleEndedIterator for SplitDigitIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.num != 0 {
            let digit = (self.num % 10) as u8;
            self.num /= 10;
            Some(digit)
        } else {
            None
        }
    }
}

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

fn split_digits_iterator_reverse(d: usize) -> Vec<u8> {
    let mut digits: Vec<u8> = SplitDigitIterator::new(d).rev().collect();
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
        group.bench_with_input(
            BenchmarkId::new("split_digits_iterator_reverse", s),
            d,
            |b, d| b.iter(|| split_digits_iterator_reverse(black_box(*d))),
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
