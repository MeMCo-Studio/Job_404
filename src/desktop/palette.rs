use egui_macroquad::egui::Color32;

// Backgrounds
pub const BACKGROUND: Color32 = Color32::from_rgb(61, 58, 107); // desktop wallpaper
pub const SURFACE: Color32 = Color32::from_rgb(244, 236, 220); // windows
pub const SURFACE_VARIANT: Color32 = Color32::from_rgb(230, 218, 195); // groups, code
pub const INPUT: Color32 = Color32::from_rgb(255, 250, 240); // text fields

// Inverse (dark) surfaces: taskbar
pub const SURFACE_INVERSE: Color32 = Color32::from_rgb(36, 31, 61);
pub const SURFACE_INVERSE_VARIANT: Color32 = Color32::from_rgb(64, 57, 102);

// Text
pub const TEXT: Color32 = Color32::from_rgb(43, 33, 64);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(107, 95, 128);
pub const TEXT_INVERSE: Color32 = SURFACE; // text on dark / primary backgrounds
pub const TEXT_INVERSE_MUTED: Color32 = SURFACE_VARIANT;

// Borders
pub const BORDER: Color32 = TEXT;
pub const BORDER_MUTED: Color32 = TEXT_MUTED;

// Brand
pub const PRIMARY: Color32 = Color32::from_rgb(112, 98, 186); // periwinkle violet
pub const PRIMARY_DARK: Color32 = Color32::from_rgb(78, 66, 145);
pub const SECONDARY: Color32 = Color32::from_rgb(79, 179, 169); // teal
pub const SECONDARY_DARK: Color32 = Color32::from_rgb(46, 124, 118);

// Controls & states
pub const BUTTON: Color32 = Color32::from_rgb(217, 201, 171);
pub const HIGHLIGHT: Color32 = Color32::from_rgb(242, 184, 128); // hover, amber
pub const WARNING: Color32 = HIGHLIGHT;
pub const ERROR: Color32 = PRIMARY_DARK;
