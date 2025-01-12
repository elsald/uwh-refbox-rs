use iced::{
    pure::widget::{button, container},
    Background, Color, Vector,
};
use paste::paste;

pub const BORDER_RADIUS: f32 = 9.0;
pub const BORDER_WIDTH: f32 = 6.0;
pub const SPACING: u16 = 8; // Must be a multiple of 4
pub const PADDING: u16 = 8;
pub const MIN_BUTTON_SIZE: u16 = 89;

pub const SMALL_TEXT: u16 = 22;
pub const SMALL_PLUS_TEXT: u16 = 34;
pub const MEDIUM_TEXT: u16 = 44;
pub const LARGE_TEXT: u16 = 80;

// See https://stackoverflow.com/a/727339 for color mixing math. For darkening colors with pure
// black, the math simplifies to new_r = orig_r * (1 - black_alpha), so we will multiply by the
// value of (1 - black_alpha)
macro_rules! make_color {
    ($name:ident, $r:literal, $g:literal, $b:literal) => {
        paste! {
            pub const $name: iced::Color = iced::Color::from_rgb($r, $g, $b);
            pub const [<$name _PRESSED>]: iced::Color = iced::Color::from_rgb(
                $r * 0.85,
                $g * 0.85,
                $b * 0.85);
        }
    };
}

make_color!(WHITE, 1.0, 1.0, 1.0);
make_color!(RED, 1.0, 0.0, 0.0);
make_color!(ORANGE, 1.0, 0.5, 0.0);
make_color!(YELLOW, 1.0, 1.0, 0.0);
make_color!(GREEN, 0.0, 1.0, 0.0);
make_color!(BLUE, 0.0, 0.0, 1.0);
make_color!(GRAY, 0.5, 0.5, 0.5);
make_color!(LIGHT_GRAY, 0.7, 0.7, 0.7);

pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);
pub const BLACK_PRESSED: Color = Color::from_rgb(0.15, 0.15, 0.15);

pub const BORDER_COLOR: Color = Color::from_rgb(0.3, 0.47, 1.0);

pub const DISABLED_COLOR: Color = GRAY;

pub const WINDOW_BACKGROUND: Color = Color::from_rgb(0.82, 0.82, 0.82);

#[derive(Clone, Copy, Debug)]
pub enum Button {
    White,
    WhiteSelected,
    Black,
    BlackSelected,
    Red,
    RedSelected,
    Orange,
    OrangeSelected,
    Yellow,
    YellowSelected,
    Green,
    GreenSelected,
    Blue,
    Gray,
    LightGray,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let (background_color, text_color) = match self {
            Self::White | Self::WhiteSelected => (WHITE, BLACK),
            Self::Black | Self::BlackSelected => (BLACK, WHITE),
            Self::Red | Self::RedSelected => (RED, BLACK),
            Self::Orange | Self::OrangeSelected => (ORANGE, BLACK),
            Self::Yellow | Self::YellowSelected => (YELLOW, BLACK),
            Self::Green | Self::GreenSelected => (GREEN, BLACK),
            Self::Blue => (BLUE, WHITE),
            Self::Gray => (GRAY, BLACK),
            Self::LightGray => (LIGHT_GRAY, BLACK),
        };

        let border_width = match self {
            Self::White
            | Self::Black
            | Self::Red
            | Self::Orange
            | Self::Yellow
            | Self::Green
            | Self::Blue
            | Self::Gray
            | Self::LightGray => 0.0,
            Self::WhiteSelected
            | Self::BlackSelected
            | Self::RedSelected
            | Self::OrangeSelected
            | Self::YellowSelected
            | Self::GreenSelected => BORDER_WIDTH,
        };

        let background = Some(Background::Color(background_color));

        button::Style {
            shadow_offset: Vector::default(),
            background,
            border_radius: BORDER_RADIUS,
            border_width,
            border_color: BORDER_COLOR,
            text_color,
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        let background_color = match self {
            Self::White | Self::WhiteSelected => WHITE_PRESSED,
            Self::Black | Self::BlackSelected => BLACK_PRESSED,
            Self::Red | Self::RedSelected => RED_PRESSED,
            Self::Orange | Self::OrangeSelected => ORANGE_PRESSED,
            Self::Yellow | Self::YellowSelected => YELLOW_PRESSED,
            Self::Green | Self::GreenSelected => GREEN_PRESSED,
            Self::Blue => BLUE_PRESSED,
            Self::Gray => GRAY_PRESSED,
            Self::LightGray => LIGHT_GRAY_PRESSED,
        };

        button::Style {
            background: Some(Background::Color(background_color)),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(WINDOW_BACKGROUND)),
            border_color: DISABLED_COLOR,
            border_width: BORDER_WIDTH,
            text_color: DISABLED_COLOR,
            ..self.active()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Container {
    LightGray,
    Gray,
    Black,
    White,
    ScrollBar,
    Disabled,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Self::LightGray => cont_style(LIGHT_GRAY, BLACK),
            Self::Gray => cont_style(GRAY, BLACK),
            Self::Black => cont_style(BLACK, WHITE),
            Self::White => cont_style(WHITE, BLACK),
            Self::ScrollBar => cont_style(WINDOW_BACKGROUND, BLACK),
            Self::Disabled => container::Style {
                text_color: Some(DISABLED_COLOR),
                background: None,
                border_radius: BORDER_RADIUS,
                border_width: BORDER_WIDTH,
                border_color: DISABLED_COLOR,
            },
        }
    }
}

fn cont_style(bkgnd: Color, text: Color) -> container::Style {
    container::Style {
        text_color: Some(text),
        background: Some(Background::Color(bkgnd)),
        border_radius: BORDER_RADIUS,
        border_width: 0.0,
        border_color: BORDER_COLOR,
    }
}
