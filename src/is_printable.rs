use std::ops::Deref;

pub trait IsPrintable {
    fn is_printable(&self) -> bool;
}

impl IsPrintable for char {
    fn is_printable(&self) -> bool {
        let formatted_char = format!("{self:?}");

        let is_printable = !formatted_char.ends_with('}');

        is_printable
    }
}

impl IsPrintable for dyn Deref<Target = str> {
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for String {
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for str {
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for &String {
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}

impl IsPrintable for &str {
    fn is_printable(&self) -> bool {
        let is_printable = self.chars().all(|ch| ch.is_printable());

        is_printable
    }
}
