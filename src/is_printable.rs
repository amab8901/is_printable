use core::ops::Deref;

pub trait IsPrintable {
    fn is_printable(&self) -> bool;
}

impl IsPrintable for char {
    fn is_printable(&self) -> bool {
        let is_typical_printable = self.escape_debug().count() == 1;
        let is_special_printable = matches!(self, '\'' | '\"' | '\\');
        is_typical_printable || is_special_printable
    }
}

impl IsPrintable for dyn Deref<Target = str> {
    fn is_printable(&self) -> bool {
        self.chars().all(|ch| ch.is_printable())
    }
}

impl IsPrintable for str {
    fn is_printable(&self) -> bool {
        self.chars().all(|ch| ch.is_printable())
    }
}

impl<T: ?Sized + IsPrintable> IsPrintable for &T {
    fn is_printable(&self) -> bool {
        (**self).is_printable()
    }
}

#[cfg(test)]
mod test {

    use super::IsPrintable;

    #[test]
    fn unprintable() {
        assert!(!'\u{7}'.is_printable()); // char
        assert!(!'\u{7}'.to_string().is_printable()); // String
        assert!(!'\u{7}'.to_string().into_boxed_str().is_printable()); // Box<str>
        assert!(!'\u{7}'.to_string().as_str().is_printable()); // &str
        assert!(!(*'\u{7}'.to_string().as_str()).is_printable()); // str
    }

    #[test]
    fn printable() {
        assert!('\u{30}'.is_printable()); // char
        assert!('\u{30}'.to_string().is_printable()); // String
        assert!('\u{30}'.to_string().into_boxed_str().is_printable()); // Box<str>
        assert!('\u{30}'.to_string().as_str().is_printable()); // &str
        assert!((*'\u{30}'.to_string().as_str()).is_printable()); // str
    }

    #[test]
    /// Run this with `cargo t visual_inspection -- --nocapture`.
    fn visual_inspection() {
        use colored::*;
        use std::{thread::sleep, time::Duration};

        let min_value = 0x1;
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
