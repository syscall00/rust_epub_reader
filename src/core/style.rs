use druid::{Color, piet::ColorParseError};

pub const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);

pub const PRIMARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#3C4047");
pub const PRIMARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");

//pub const SECONDARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#34383E");
//pub const SECONDARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");
//pub const COMPLEMENTARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#2F3938");
//pub const TERTIARY_DARK : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#6B6458");
//pub const TERTIARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");


pub fn get_color_unchecked(color: Result<Color, ColorParseError>) -> Color {
    match color {
        Ok(color) => color,
        Err(_) => Color::rgb8(0, 0, 0),
    }
}
