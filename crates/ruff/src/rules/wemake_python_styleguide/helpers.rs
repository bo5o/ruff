use once_cell::sync::Lazy;
use regex::Regex;

const ALIAS_NAMES_WHITELIST: [&str; 7] = ["np", "pd", "df", "plt", "sns", "tf", "cv"];

const UNUSED_PLACEHOLDER: char = '_';

static UNUSED_VARIABLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^_+$").unwrap());

static UNDERSCORED_NUMBER_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".+\D\_\d+(\D|$)").unwrap());

/// Checks whether the given ``name`` is unused.
pub(super) fn is_unused(name: &str) -> bool {
    UNUSED_VARIABLE_REGEX.is_match(name)
}

/// Checks for too short names.
pub(super) fn is_too_short_name(name: &str, min_length: usize, trim: bool) -> bool {
    if ALIAS_NAMES_WHITELIST.contains(&name) {
        return false;
    }

    if is_unused(name) {
        return false;
    }

    let length = if trim {
        name.trim_matches(UNUSED_PLACEHOLDER).len()
    } else {
        name.len()
    };

    length < min_length
}

/// Checks for names with underscored number.
pub(super) fn does_contain_underscored_number(name: &str) -> bool {
    UNDERSCORED_NUMBER_PATTERN.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::{does_contain_underscored_number, is_too_short_name, is_unused};

    #[test]
    fn test_is_too_short_name() {
        assert!(!is_too_short_name("test", 2, true));
        assert!(is_too_short_name("o", 2, true));
        assert!(!is_too_short_name("_", 2, true));
        assert!(!is_too_short_name("_", 1, true));
        assert!(!is_too_short_name("z1", 2, true));
        assert!(!is_too_short_name("z", 1, true));
        assert!(is_too_short_name("_z", 2, true));
        assert!(is_too_short_name("z_", 2, true));
        assert!(!is_too_short_name("z_", 2, false));
        assert!(is_too_short_name("__z", 2, true));
        assert!(!is_too_short_name("xy", 2, true));
        assert!(!is_too_short_name("np", 3, true));
    }

    #[test]
    fn test_is_unused() {
        assert!(is_unused("_"));
        assert!(is_unused("___"));
        assert!(!is_unused("_protected"));
        assert!(!is_unused("__private"));
    }

    #[test]
    fn test_does_contain_underscored_number() {
        assert!(!does_contain_underscored_number("star_wars_episode2"));
        assert!(!does_contain_underscored_number("come2_me"));
        assert!(!does_contain_underscored_number("_"));
        assert!(!does_contain_underscored_number("z1"));
        assert!(!does_contain_underscored_number("iso123_456"));
        assert!(does_contain_underscored_number("star_wars_episode_2"));
        assert!(does_contain_underscored_number("come_2_me"));
        assert!(does_contain_underscored_number("come_44_me"));
        assert!(does_contain_underscored_number("iso_123_456"));
    }
}
