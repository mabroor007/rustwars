pub fn name_shuffler(s: &str) -> String {
    let s = s.to_string();
    let parts: Vec<&str> = s.split(" ").collect();

    let mut res = String::new();

    res.push_str(parts[1]);
    res.push_str(" ");
    res.push_str(parts[0]);

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_shuffler_mabroor_ahmad() {
        assert_eq!(name_shuffler("mabroor ahmad"), "ahmad mabroor".to_string());
    }

    #[test]
    fn name_shuffler_mary_jeggins() {
        assert_eq!(name_shuffler("Mery jeggins"), "jeggins Mery".to_string());
    }
}
