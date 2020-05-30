use ggez::graphics::Color;

#[derive(Copy, Clone, Debug)]
pub enum Pal {
    Black,
    DarkBlue,
    Purple,
    DarkGreen,
    Maroon,
    Brown,
    LightGray,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Gray,
    Pink,
    Peach,
}

impl Pal {
    pub fn darken(self, factor: f32) -> Color {
        let mut color: Color = self.into();
        color.r *= 1. - factor;
        color.g *= 1. - factor;
        color.b *= 1. - factor;

        color
    }

    pub fn dark(self) -> Color {
        self.darken(0.5)
    }

    pub fn darker(self) -> Color {
        self.darken(0.75)
    }
}

impl Into<Color> for Pal {
    fn into(self) -> Color {
        match self {
            Self::Black => Color::from((0x06, 0x08, 0x08)),
            // Self::Black => Color::from((0x22, 0x23, 0x23)),
            Self::DarkBlue => Color::from((0x1D, 0x2B, 0x53)),
            Self::Purple => Color::from((0x7E, 0x25, 0x53)),
            Self::DarkGreen => Color::from((0x00, 0x87, 0x51)),
            Self::Maroon => Color::from((0xAB, 0x52, 0x36)),
            Self::Brown => Color::from((0x5F, 0x57, 0x4F)),
            Self::LightGray => Color::from((0xC2, 0xC3, 0xC7)),
            // Self::White => Color::from((0xFF, 0xF1, 0xE8)),
            Self::White => Color::from((0xF0, 0xF6, 0xF0)),
            Self::Red => Color::from((0xFF, 0x00, 0x4D)),
            Self::Orange => Color::from((0xFF, 0xA3, 0x00)),
            Self::Yellow => Color::from((0xFF, 0xEC, 0x27)),
            Self::Green => Color::from((0x00, 0xE4, 0x36)),
            Self::Blue => Color::from((0x29, 0xAD, 0xFF)),
            Self::Gray => Color::from((0x83, 0x76, 0x9C)),
            Self::Pink => Color::from((0xFF, 0x77, 0xA8)),
            Self::Peach => Color::from((0xFF, 0xCC, 0xAA)),
        }
    }
}
