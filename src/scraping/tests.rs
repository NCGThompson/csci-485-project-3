#![cfg(test)]

#[allow(unused_imports)]
use super::*;

mod regex_pattern_creation {
    use super::*;

    #[test]
    fn test_no_targets() {
        let targets: Vec<&str> = vec![];
        assert_eq!(concatenate_targets(&targets), "");
    }

    #[test]
    fn test_single_target() {
        let targets = vec!["hello"];
        assert_eq!(concatenate_targets(&targets), "hello");
    }

    #[test]
    fn test_empty_target() {
        let targets = vec![""];
        assert_eq!(concatenate_targets(&targets), "");
    }

    #[test]
    fn test_multiple_unicode_characters_in_targets() {
        let targets = ["foo", "bar", "baz"];
        assert_eq!(concatenate_targets(&targets), r"(?:foo|bar|baz)");
        let targets = ["foo", "bar"];
        assert_eq!(concatenate_targets(&targets), r"(?:foo|bar)");
        let targets = ["foo", "bar", "baz", "boop"];
        assert_eq!(concatenate_targets(&targets), r"(?:foo|bar|baz|boop)");
        let targets = ["a.c", "^*", "$$"];
        assert_eq!(concatenate_targets(&targets), r"(?:a.c|^*|$$)");
        let targets = ["日本", "España", "😊"];
        assert_eq!(concatenate_targets(&targets), r"(?:日本|España|😊)");
    }
}
