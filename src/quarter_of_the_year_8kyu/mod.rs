pub fn quarter_of(month: u8) -> u8 {
    if month < 4 {
        return 1;
    };
    if month < 7 {
        return 2;
    };
    if month < 10 {
        return 3;
    };
    return 4;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quarter_of_3() {
        assert_eq!(quarter_of(3), 1);
    }

    #[test]
    fn quarter_of_12() {
        assert_eq!(quarter_of(12), 4);
    }

    #[test]
    fn quarter_of_8() {
        assert_eq!(quarter_of(8), 3);
    }

    #[test]
    fn quarter_of_11() {
        assert_eq!(quarter_of(11), 4);
    }
}
