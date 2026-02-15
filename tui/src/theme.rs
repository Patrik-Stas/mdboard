use ratatui::style::Color;

/// Convert a hex color string like "#3b82f6" to a ratatui Color.
pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Color::White;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    Color::Rgb(r, g, b)
}

// Semantic colors
pub const HEADER_BG: Color = Color::Rgb(30, 30, 46);
pub const HEADER_FG: Color = Color::Rgb(205, 214, 244);
pub const TAB_ACTIVE_FG: Color = Color::Rgb(137, 180, 250);
pub const TAB_INACTIVE_FG: Color = Color::Rgb(108, 112, 134);
pub const BORDER_COLOR: Color = Color::Rgb(69, 71, 90);
pub const BORDER_HIGHLIGHT: Color = Color::Rgb(137, 180, 250);
pub const TEXT_PRIMARY: Color = Color::Rgb(205, 214, 244);
pub const TEXT_SECONDARY: Color = Color::Rgb(147, 153, 178);
pub const TEXT_DIM: Color = Color::Rgb(108, 112, 134);
pub const SURFACE_1: Color = Color::Rgb(49, 50, 68);
pub const OVERLAY_BG: Color = Color::Rgb(24, 24, 37);
pub const GREEN: Color = Color::Rgb(166, 227, 161);
pub const YELLOW: Color = Color::Rgb(249, 226, 175);
pub const RED: Color = Color::Rgb(243, 139, 168);
pub const SCOPE_FG: Color = Color::Rgb(180, 190, 254);
