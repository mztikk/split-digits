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

    DigitAndPosition {
        digit,
        position: position - 1,
    }
}

pub struct SplitDigitIterator {
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
    }
}
