pub struct SplitDigitIterator {
    num: usize,
    divisor: usize,
}

impl SplitDigitIterator {
    pub fn new(num: usize) -> Self {
        let mut divisor = 1;
        while num >= divisor * 10 {
            divisor *= 10;
        }
        Self { num, divisor }
    }
}

impl Iterator for SplitDigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v: u8 = (self.num / self.divisor) as u8;
            self.num %= self.divisor;
            self.divisor /= 10;
            Some(v)
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

#[cfg(test)]
mod tests {
    use crate::SplitDigitIterator;

    #[test]
    fn test_split_digits() {
        assert_eq!(
            vec![1, 2, 3],
            SplitDigitIterator::new(123).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![3, 2, 1],
            SplitDigitIterator::new(321).collect::<Vec<_>>()
        );
        assert_eq!(vec![1], SplitDigitIterator::new(1).collect::<Vec<_>>());
        assert_eq!(
            vec![1, 0, 0, 1],
            SplitDigitIterator::new(1001).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_split_digits_trailing_zeroes() {
        assert_eq!(
            vec![1, 2, 0, 0, 0],
            SplitDigitIterator::new(12000).collect::<Vec<_>>()
        );
        assert_eq!(vec![1, 0], SplitDigitIterator::new(10).collect::<Vec<_>>());
    }

    #[test]
    fn test_split_digits_reverse() {
        let digits: Vec<u8> = SplitDigitIterator::new(568764567).rev().collect();
        assert_eq!(digits, vec![7, 6, 5, 4, 6, 7, 8, 6, 5]);

        let digits: Vec<u8> = SplitDigitIterator::new(1001).rev().collect();
        assert_eq!(digits, vec![1, 0, 0, 1]);
    }
}
