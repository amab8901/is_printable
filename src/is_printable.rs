use std::ops::Deref;

pub trait IsPrintable {
    fn is_printable(&self) -> bool;
}

impl IsPrintable for char {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let escape_debug = self.escape_debug().to_string();

        let is_special_printable = is_special_printable(&escape_debug);

        let escapes = escape_debug.starts_with('\\');

        let single_char = escape_debug.len() == 1;
        let typical_printable = !escapes || single_char;

        let is_printable = typical_printable || is_special_printable;

        is_printable
    }
}

impl IsPrintable for dyn Deref<Target = str> {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for String {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for str {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for &String {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for &str {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

pub fn is_special_printable(escape_debug: &str) -> bool {
    let character = escape_debug.chars().last().unwrap();

    let is_special_printable = matches!(character, '\'' | '\"' | '\\' | '/');
    is_special_printable
}

#[cfg(test)]
mod test {
    use super::IsPrintable;

    #[test]
    pub fn unprintable() {
        let character = '\u{7}';

        let is_printable = character.is_printable();
        assert!(!is_printable);
    }

    #[test]
    pub fn printable() {
        let character = '\u{30}';

        let is_printable = character.is_printable();
        assert!(is_printable);
    }
}
