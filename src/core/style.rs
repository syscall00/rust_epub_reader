use druid::{Color, piet::ColorParseError, Env, Key};

pub const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);
pub const BAR_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#7EA0B7");
pub const _PRIMARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");
pub const _CONTENT_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#597081");

pub const COMPLEMENTARY_LIGHT: Key<Color> = Key::new("org.linebender.druid.theme.foreground_dark");



pub const PRIMARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#3C4047");
pub const PRIMARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");

pub const SECONDARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#34383E");
pub const SECONDARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");

pub const COMPLEMENTARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#2F3938");

pub const TERTIARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#6B6458");
pub const TERTIARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");


pub fn get_color_unchecked(color: Result<Color, ColorParseError>) -> Color {
    match color {
        Ok(color) => color,
        Err(_) => Color::rgb8(0, 0, 0),
    }
}


pub fn add_to_env(env : &mut Env) {
    let env = env;
    //env.set(druid::theme::WINDOW_BACKGROUND_COLOR, get_color_unchecked(BAR_COLOR));
    //env.set(druid::theme::PRIMARY_LIGHT, get_color_unchecked(PRIMARY_LIGHT));
    //env.set(druid::theme::PRIMARY_DARK, get_color_unchecked(PRIMARY_DARK));
    //env.set(druid::theme::SECONDARY_LIGHT, get_color_unchecked(SECONDARY_LIGHT));
    //env.set(druid::theme::SECONDARY_DARK, get_color_unchecked(SECONDARY_DARK));
    env.set(crate::style::COMPLEMENTARY_LIGHT, get_color_unchecked(Color::from_hex_str("#637391")));
    //env.set(druid::theme::COMPLEMENTARY_DARK, get_color_unchecked(COMPLEMENTARY_DARK));
    //env.set(druid::theme::TERTIARY_LIGHT, get_color_unchecked(TERTIARY_LIGHT));
    //env.set(druid::theme::TERTIARY_DARK, get_color_unchecked(TERTIARY_DARK));
   // env
}


use lazy_static::lazy_static;
// generate a singleton struct that will hold the a struct for the theme
lazy_static! {
    pub static ref THEME: Theme = Theme::new();
}

// define the theme struct
pub struct Theme {
    pub bar_color: Color,
    pub link_color: Color,
    pub primary_light: Color,
    pub primary_dark: Color,
    pub secondary_light: Color,
    pub secondary_dark: Color,
   // pub complementary_light: Color,
    pub complementary_dark: Color,
    pub tertiary_light: Color,
    pub tertiary_dark: Color,
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            bar_color: get_color_unchecked(BAR_COLOR),
            link_color: LINK_COLOR,
            primary_light: get_color_unchecked(PRIMARY_LIGHT),
            primary_dark: get_color_unchecked(PRIMARY_DARK),
            secondary_light: get_color_unchecked(SECONDARY_LIGHT),
            secondary_dark: get_color_unchecked(SECONDARY_DARK),
            //complementary_light: get_color_unchecked(COMPLEMENTARY_LIGHT),
            complementary_dark: get_color_unchecked(COMPLEMENTARY_DARK),
            tertiary_light: get_color_unchecked(TERTIARY_LIGHT),
            tertiary_dark: get_color_unchecked(TERTIARY_DARK),
        }
    }
}