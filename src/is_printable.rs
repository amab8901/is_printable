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

    #[test]
    /// Run this with `cargo t visual_inspection -- --nocapture`.
    pub fn visual_inspection() { 
        use std::{thread::sleep, time::Duration};

        dbg!("lol");

        use colored::*;

        let min_value = 0x1;
        // let min_value = 75;
        // let max_value = 120;
        let max_value = 0x99_999;

        println!("lol");

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

        pub fn is_special_printable(escape_debug: &str) -> bool {
            let character = escape_debug.chars().last().unwrap();

            let is_special_printable = matches!(character, '\'' | '\"' | '\\' | '/');
            is_special_printable
        }
    }
}
