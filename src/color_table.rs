use bevy::color::palettes::css;
use bevy::color::{ColorToPacked, Srgba};
use bevy::math::VectorSpace;

pub fn parse_color(s: &str) -> Option<Srgba> {
    if s.starts_with('#') {
        return parse_hex_color(s.split_at(1).1);
    }
    Some(match s {
        "transparent" => Srgba::ZERO,
        "aqua" => css::AQUA,
        "black" => css::BLACK,
        "blue" => css::BLUE,
        "fuchsia" => css::FUCHSIA,
        "gray" => css::GRAY,
        "green" => css::GREEN,
        "lime" => css::LIME,
        "maroon" => css::MAROON,
        "navy" => css::NAVY,
        "olive" => css::OLIVE,
        "purple" => css::PURPLE,
        "red" => css::RED,
        "silver" => css::SILVER,
        "teal" => css::TEAL,
        "white" => css::WHITE,
        "yellow" => css::YELLOW,
        "alice-blue" => css::ALICE_BLUE,
        "antique-white" => css::ANTIQUE_WHITE,
        "aquamarine" => css::AQUAMARINE,
        "azure" => css::AZURE,
        "beige" => css::BEIGE,
        "bisque" => css::BISQUE,
        "blanched-almond" => css::BLANCHED_ALMOND,
        "blue-violet" => css::BLUE_VIOLET,
        "brown" => css::BROWN,
        "burlywood" => css::BURLYWOOD,
        "cadet-blue" => css::CADET_BLUE,
        "chartreuse" => css::CHARTREUSE,
        "chocolate" => css::CHOCOLATE,
        "coral" => css::CORAL,
        "cornflower-blue" => css::CORNFLOWER_BLUE,
        "cornsilk" => css::CORNSILK,
        "crimson" => css::CRIMSON,
        "dark-blue" => css::DARK_BLUE,
        "dark-cyan" => css::DARK_CYAN,
        "dark-goldenrod" => css::DARK_GOLDENROD,
        "dark-gray" => css::DARK_GRAY,
        "dark-green" => css::DARK_GREEN,
        "dark-grey" => css::DARK_GREY,
        "dark-khaki" => css::DARK_KHAKI,
        "dark-magenta" => css::DARK_MAGENTA,
        "dark-olivegreen" => css::DARK_OLIVEGREEN,
        "dark-orange" => css::DARK_ORANGE,
        "dark-orchid" => css::DARK_ORCHID,
        "dark-red" => css::DARK_RED,
        "dark-salmon" => css::DARK_SALMON,
        "dark-sea-green" => css::DARK_SEA_GREEN,
        "dark-slate-blue" => css::DARK_SLATE_BLUE,
        "dark-slate-gray" => css::DARK_SLATE_GRAY,
        "dark-slate-grey" => css::DARK_SLATE_GREY,
        "dark-turquoise" => css::DARK_TURQUOISE,
        "dark-violet" => css::DARK_VIOLET,
        "deep-pink" => css::DEEP_PINK,
        "deep-sky-blue" => css::DEEP_SKY_BLUE,
        "dim-gray" => css::DIM_GRAY,
        "dim-grey" => css::DIM_GREY,
        "dodger-blue" => css::DODGER_BLUE,
        "fire-brick" => css::FIRE_BRICK,
        "floral-white" => css::FLORAL_WHITE,
        "forest-green" => css::FOREST_GREEN,
        "gainsboro" => css::GAINSBORO,
        "ghost-white" => css::GHOST_WHITE,
        "gold" => css::GOLD,
        "goldenrod" => css::GOLDENROD,
        "green-yellow" => css::GREEN_YELLOW,
        "grey" => css::GREY,
        "honeydew" => css::HONEYDEW,
        "hot-pink" => css::HOT_PINK,
        "indian-red" => css::INDIAN_RED,
        "indigo" => css::INDIGO,
        "ivory" => css::IVORY,
        "khaki" => css::KHAKI,
        "lavender" => css::LAVENDER,
        "lavender-blush" => css::LAVENDER_BLUSH,
        "lawn-green" => css::LAWN_GREEN,
        "lemon-chiffon" => css::LEMON_CHIFFON,
        "light-blue" => css::LIGHT_BLUE,
        "light-coral" => css::LIGHT_CORAL,
        "light-cyan" => css::LIGHT_CYAN,
        "light-goldenrod-yellow" => css::LIGHT_GOLDENROD_YELLOW,
        "light-gray" => css::LIGHT_GRAY,
        "light-green" => css::LIGHT_GREEN,
        "light-grey" => css::LIGHT_GREY,
        "light-pink" => css::LIGHT_PINK,
        "light-salmon" => css::LIGHT_SALMON,
        "light-sea-green" => css::LIGHT_SEA_GREEN,
        "light-sky-blue" => css::LIGHT_SKY_BLUE,
        "light-slate-gray" => css::LIGHT_SLATE_GRAY,
        "light-slate-grey" => css::LIGHT_SLATE_GREY,
        "light-steel-blue" => css::LIGHT_STEEL_BLUE,
        "light-yellow" => css::LIGHT_YELLOW,
        "limegreen" => css::LIMEGREEN,
        "linen" => css::LINEN,
        "magenta" => css::MAGENTA,
        "medium-aquamarine" => css::MEDIUM_AQUAMARINE,
        "medium-blue" => css::MEDIUM_BLUE,
        "medium-orchid" => css::MEDIUM_ORCHID,
        "medium-purple" => css::MEDIUM_PURPLE,
        "medium-sea-green" => css::MEDIUM_SEA_GREEN,
        "medium-slate-blue" => css::MEDIUM_SLATE_BLUE,
        "medium-spring-green" => css::MEDIUM_SPRING_GREEN,
        "medium-turquoise" => css::MEDIUM_TURQUOISE,
        "medium-violet-red" => css::MEDIUM_VIOLET_RED,
        "midnight-blue" => css::MIDNIGHT_BLUE,
        "mint-cream" => css::MINT_CREAM,
        "misty-rose" => css::MISTY_ROSE,
        "moccasin" => css::MOCCASIN,
        "navajo-white" => css::NAVAJO_WHITE,
        "old-lace" => css::OLD_LACE,
        "olive-drab" => css::OLIVE_DRAB,
        "orange" => css::ORANGE,
        "orange-red" => css::ORANGE_RED,
        "orchid" => css::ORCHID,
        "pale-goldenrod" => css::PALE_GOLDENROD,
        "pale-green" => css::PALE_GREEN,
        "pale-turquoise" => css::PALE_TURQUOISE,
        "pale-violetred" => css::PALE_VIOLETRED,
        "papaya-whip" => css::PAPAYA_WHIP,
        "peachpuff" => css::PEACHPUFF,
        "peru" => css::PERU,
        "pink" => css::PINK,
        "plum" => css::PLUM,
        "powder-blue" => css::POWDER_BLUE,
        "rebecca-purple" => css::REBECCA_PURPLE,
        "rosy-brown" => css::ROSY_BROWN,
        "royal-blue" => css::ROYAL_BLUE,
        "saddle-brown" => css::SADDLE_BROWN,
        "salmon" => css::SALMON,
        "sandy-brown" => css::SANDY_BROWN,
        "sea-green" => css::SEA_GREEN,
        "seashell" => css::SEASHELL,
        "sienna" => css::SIENNA,
        "sky-blue" => css::SKY_BLUE,
        "slate-blue" => css::SLATE_BLUE,
        "slate-gray" => css::SLATE_GRAY,
        "slate-grey" => css::SLATE_GREY,
        "snow" => css::SNOW,
        "spring-green" => css::SPRING_GREEN,
        "steel-blue" => css::STEEL_BLUE,
        "tan" => css::TAN,
        "thistle" => css::THISTLE,
        "tomato" => css::TOMATO,
        "turquoise" => css::TURQUOISE,
        "violet" => css::VIOLET,
        "wheat" => css::WHEAT,
        "white-smoke" => css::WHITE_SMOKE,
        "yellow-green" => css::YELLOW_GREEN,
        _ => return None,
    })
}

pub fn parse_hex_color(s: &str) -> Option<Srgba> {
    fn h(b: &u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(*b - b'0'),
            b'a'..=b'f' => Some(*b - b'a' + 10),
            b'A'..=b'F' => Some(*b - b'A' + 10),
            _ => None,
        }
    }
    Some(Srgba::from_u8_array(match s.as_bytes() {
        [r, g, b] => [h(r)? * 0x11, h(g)? * 0x11, h(b)? * 0x11, 255],
        [r, g, b, a] => [h(r)? * 0x11, h(g)? * 0x11, h(b)? * 0x11, h(a)? * 0x11],
        [r1, r2, g1, g2, b1, b2] => [
            h(r1)? * 0x10 + h(r2)?,
            h(g1)? * 0x10 + h(g2)?,
            h(b1)? * 0x10 + h(b2)?,
            255,
        ],
        [r1, r2, g1, g2, b1, b2, a1, a2] => [
            h(r1)? * 0x10 + h(r2)?,
            h(g1)? * 0x10 + h(g2)?,
            h(b1)? * 0x10 + h(b2)?,
            h(a1)? * 0x10 + h(a2)?,
        ],
        _ => return None,
    }))
}
