// extern crate kwindex;

#[cfg(test)]
mod kwindex_tests {

    use kwindex::*;

    #[test]
    pub fn test_is_empty() {
        let index = KWIndex::new();
        assert_eq!(true, index.is_empty());
        assert_ne!(false, index.is_empty());
    }

    #[test]
    pub fn test_is_not_empty() {
        let index = KWIndex::new().extend_from_text("Hello world.");
        assert_eq!(false, index.is_empty());
        assert_ne!(true, index.is_empty());
    }

    #[test]
    pub fn test_len() {
        let index = KWIndex::new().extend_from_text("Hello world.");
        assert_eq!(2, index.len());
        assert_ne!(0, index.len());
    }

    #[test]
    pub fn test_len2() {
        let index = KWIndex::new().extend_from_text("Hello Hel1o Hello, He'llo");
        assert_eq!(2, index.len());
        assert_ne!(0, index.len());
    }

    #[test]
    pub fn test_count_matches() {
        let index = KWIndex::new().extend_from_text("Hello world.");
        assert_eq!(1, index.count_matches("world"));
        assert_ne!(0, index.count_matches("world"));
    }

    #[test]
    pub fn test_count_matches2() {
        let index = KWIndex::new().extend_from_text("Hello Hel1o Hello, He'llo");
        assert_eq!(2, index.count_matches("Hello"));
        assert_ne!(0, index.count_matches("Hello"));
    }
}
