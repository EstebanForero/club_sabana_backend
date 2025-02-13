use std::ops::Deref;

pub mod err;
pub mod repository_trait;

struct StrColor<'a> {
    content: &'a str,
    color_info: Color,
}

enum Color {
    Green,
    Black,
    Blue,
}

impl<'a> Deref for StrColor<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.color_info {
            Color::Green => format!().as_str(),
            Color::Black => format!().as_ref(),
            Color::Blue => format!().as_ref(),
        }
    }
}
