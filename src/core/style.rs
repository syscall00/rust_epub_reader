use druid::Color;

pub const LINK_COLOR: druid::Color = druid::Color::rgb8(0, 0, 0xEE);
pub const BAR_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#7EA0B7");
pub const _PRIMARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");
pub const CONTENT_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#597081");
