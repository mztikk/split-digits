pub struct SplitDigitIterator {
    num: usize,
}

impl SplitDigitIterator {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}

impl Iterator for SplitDigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            None
        } else {
            let digit = (self.num % 10) as u8;
            self.num /= 10;
            Some(digit)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SplitDigitIterator;

    #[test]
    fn test_split_digits() {
        let digits: Vec<u8> = SplitDigitIterator { num: 568764567 }.collect();
        assert_eq!(digits, vec![7, 6, 5, 4, 6, 7, 8, 6, 5]);
    }
}
