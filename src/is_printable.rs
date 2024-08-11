use std::ops::Deref;

pub trait IsPrintable {
    fn is_printable(&self) -> bool;
}

impl IsPrintable for char {
    /// Returns `true` if `self` is printable.
    fn is_printable(&self) -> bool {
        let is_typical_printable = self.escape_debug().count() == 1;
        let is_special_printable = matches!(self, '\'' | '\"' | '\\');
        let is_printable = is_typical_printable || is_special_printable;

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

    #[test]
    /// Run this with `cargo t visual_inspection -- --nocapture`.
    pub fn visual_inspection() {
        use std::{thread::sleep, time::Duration};

        use colored::*;

        let min_value = 0x1;
        // let min_value = 75;
        // let max_value = 120;
        let max_value = 0x99_999;

        for unicode in min_value..=max_value {
            let Some(character) = char::from_u32(unicode) else {
                println!("{unicode} is unprintable");
                continue;
            };

            let is_printable = character.is_printable();

            if is_printable {
                println!("{unicode} is {}", "printable".green());
                dbg!(character);
            } else {
                println!("{unicode} is {}", "unprintable".red());
                dbg!(character);
            }

            println!();
            println!();
            sleep(Duration::from_millis(100));
        }
    }
}
