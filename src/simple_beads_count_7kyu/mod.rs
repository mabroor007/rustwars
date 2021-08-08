pub fn count_red_beads(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 0;
    };
    return (n - 1) * 2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_red_beads_for_3() {
        let beads = count_red_beads(3);
        assert_eq!(beads, 4);
    }

    #[test]
    fn test_count_red_beads_for_0() {
        let beads = count_red_beads(0);
        assert_eq!(beads, 0);
    }
}
