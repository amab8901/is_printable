use std::ops::Deref;

pub trait IsPrintable {
    fn is_printable(&self) -> bool;
}

impl IsPrintable for char {
    fn is_printable(&self) -> bool {
        if self.is_control() {
            return false;
        }

        if *self != ' ' && self.is_whitespace() {
            return false;
        }

        let is_space_separator_other_than_space = is_space_separator_other_than_space(*self);

        if is_space_separator_other_than_space {
            return false;
        }

        true
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

fn is_space_separator_other_than_space(character: char) -> bool {
    let is_space_separator_other_than_space = matches!(
        character,
        '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'
            | '\u{2001}'
            | '\u{2002}'
            | '\u{2003}'
            | '\u{2004}'
            | '\u{2005}'
            | '\u{2006}'
            | '\u{2007}'
            | '\u{2008}'
            | '\u{2009}'
            | '\u{200A}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
    );

    is_space_separator_other_than_space
}
