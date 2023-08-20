use crate::util::*;
use csscolorparser::Color;
use gloo_timers::callback::Timeout;
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use yewdux::prelude::*;

/// Fonts to fall back to if no other fonts are available.
const FALLBACK_FONTS: &[&str] = &[
    "system-ui",
    "-apple-system",
    "BlinkMacSystemFont",
    "\"Segoe UI\"",
    "Roboto",
    "Oxygen",
    "Ubuntu",
    "Cantarell",
    "\"Open Sans\"",
    "\"Helvetica Neue\"",
    "sans-serif",
];

/// The set of background colors for dark mode.
const DARK_BACKGROUND_COLORS: &[Color; 6] = &[
    Color::new(0.10196, 0.10980, 0.12157, 1.0),
    Color::new(0.11765, 0.12549, 0.13725, 1.0),
    Color::new(0.13333, 0.14118, 0.15294, 1.0),
    Color::new(0.14902, 0.15686, 0.16863, 1.0),
    Color::new(0.16471, 0.17255, 0.18431, 1.0),
    Color::new(0.18039, 0.18824, 0.20000, 1.0),
];

/// The set of background colors for light mode.
const LIGHT_BACKGROUND_COLORS: &[Color; 6] = &[
    Color::new(1.0, 1.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
    Color::new(1.0, 1.0, 1.0, 1.0),
];

/// White text color for use in dark mode.
const DARK_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);

/// Black text color for use in light mode.
const LIGHT_TEXT_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

/// A filter to apply to SVGs to make them appear white in dark mode.
const DARK_SVG_FILTER: &str =
    "invert(100%) sepia(100%) saturate(0%) hue-rotate(288deg) brightness(102%) contrast(102%)";

/// A filter to apply to SVGs to make them appear off-white in dark mode.
const DARK_SVG_FILTER_DISABLED: &str =
    "invert(91%) sepia(9%) saturate(0%) hue-rotate(170deg) brightness(90%) contrast(89%)";

/// A filter to apply to SVGs to make them appear black in light mode.
const LIGHT_SVG_FILTER: &str =
    "invert(0%) sepia(0%) saturate(0%) hue-rotate(320deg) brightness(96%) contrast(104%)";

/// A filter to apply to SVGs to make them appear off-black in light mode.
const LIGHT_SVG_FILTER_DISABLED: &str =
    "invert(18%) sepia(5%) saturate(0%) hue-rotate(253deg) brightness(96%) contrast(92%)";

/// Standard border color for dark mode.
const DARK_BORDER_COLOR: Color = Color::new(0.29020, 0.29804, 0.30980, 1.0);

/// Standard border color for a focused element in dark mode.
const DARK_FOCUS_BORDER_COLOR: Color = Color::new(0.41569, 0.42353, 0.43529, 1.0);

/// Standard border color for light mode.
const LIGHT_BORDER_COLOR: Color = Color::new(0.69020, 0.69804, 0.70980, 1.0);

/// Standard border color for a focused element in light mode.
const LIGHT_FOCUS_BORDER_COLOR: Color = Color::new(0.56471, 0.57255, 0.58431, 1.0);

/// The default color for error text.
const DEFAULT_ERROR_COLOR: Color = Color::new(0.81176, 0.0, 0.0, 1.0);

/// The default primary color.
const DEFAULT_PRIMARY_COLOR: Color = Color::new(0.15686, 0.31765, 1.0, 1.0);

/// The default secondary color.
const DEFAULT_SECONDARY_COLOR: Color = Color::new(0.35294, 0.36078, 0.37255, 1.0);

/// The default danger color.
const DEFAULT_DANGER_COLOR: Color = Color::new(0.68627, 0.0, 0.0, 1.0);

/// The color mode. Defaults to dark mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    /// Dark mode.
    #[default]
    Dark,
    /// Light mode.
    Light,
}

#[allow(dead_code)]
impl ColorMode {
    /// Is this dark mode?
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Is this light mode?
    pub fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }
}

/// A styling theme.
#[derive(Debug, Clone, PartialEq, Store)]
pub struct Theme {
    /// The theme's color mode.
    pub color_mode: ColorMode,
    /// The primary color.
    pub primary_color: Color,
    /// The secondary color.
    pub secondary_color: Color,
    /// The danger color.
    pub danger_color: Color,
    /// The error text color.
    pub error_color: Color,
    /// The fonts to be applied to all elements.
    pub fonts: Vec<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::default(),
            primary_color: DEFAULT_PRIMARY_COLOR,
            secondary_color: DEFAULT_SECONDARY_COLOR,
            danger_color: DEFAULT_DANGER_COLOR,
            error_color: DEFAULT_ERROR_COLOR,
            fonts: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Theme {
    /// Sets the color mode.
    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        self.color_mode = color_mode;
    }

    /// Sets the color mode to dark mode.
    pub fn set_dark_mode(&mut self) {
        self.color_mode = ColorMode::Dark;
    }

    /// Sets the color mode to light mode.
    pub fn set_light_mode(&mut self) {
        self.color_mode = ColorMode::Light;
    }

    /// Sets the primary color.
    pub fn set_primary_color(&mut self, primary_color: impl Into<Color>) {
        self.primary_color = primary_color.into();
    }

    /// Sets the secondary color.
    pub fn set_secondary_color(&mut self, secondary_color: impl Into<Color>) {
        self.secondary_color = secondary_color.into();
    }

    /// Sets the danger color.
    pub fn set_danger_color(&mut self, danger_color: impl Into<Color>) {
        self.danger_color = danger_color.into();
    }

    /// Sets the error text color.
    pub fn set_error_color(&mut self, error_color: impl Into<Color>) {
        self.error_color = error_color.into();
    }

    /// Sets the list of fonts.
    pub fn set_fonts(&mut self, fonts: &[impl AsRef<str>]) {
        self.fonts = fonts.iter().map(|s| s.as_ref().to_owned()).collect();
    }

    /// Adds a new font to the the font list.
    pub fn add_font(&mut self, font: &str) {
        self.fonts.push(font.to_owned());
    }

    /// Sets the color mode.
    pub fn color_mode(mut self, color_mode: ColorMode) -> Self {
        self.set_color_mode(color_mode);
        self
    }

    /// Sets the color mode to dark mode.
    pub fn dark_mode(mut self) -> Self {
        self.set_dark_mode();
        self
    }

    /// Sets the color mode to light mode.
    pub fn light_mode(mut self) -> Self {
        self.set_light_mode();
        self
    }

    /// Sets the primary color.
    pub fn primary_color(mut self, primary_color: impl Into<Color>) -> Self {
        self.set_primary_color(primary_color);
        self
    }

    /// Sets the secondary color.
    pub fn secondary_color(mut self, secondary_color: impl Into<Color>) -> Self {
        self.set_secondary_color(secondary_color);
        self
    }

    /// Sets the danger color.
    pub fn danger_color(mut self, danger_color: impl Into<Color>) -> Self {
        self.set_danger_color(danger_color);
        self
    }

    /// Sets the error text color.
    pub fn error_color(mut self, error_color: impl Into<Color>) -> Self {
        self.set_error_color(error_color);
        self
    }

    /// Sets the list of fonts.
    pub fn fonts(mut self, fonts: &[impl AsRef<str>]) -> Self {
        self.set_fonts(fonts);
        self
    }

    /// Adds a new font to the font list.
    pub fn font(mut self, font: &str) -> Self {
        self.add_font(font);
        self
    }
}

/// Determines the text color to use based on the background color.
fn derive_text_color(background_color: &Color) -> Color {
    if (background_color.r + background_color.g + background_color.b) / 3.0 < 0.6 {
        DARK_TEXT_COLOR
    } else {
        LIGHT_TEXT_COLOR
    }
}

/// Sets a CSS variable.
fn set_css_var(name: &str, value: &str) {
    let name = name.to_owned();
    let value = value.to_owned();

    Timeout::new(0, move || {
        let root = document().document_element().unwrap();
        let root: web_sys::HtmlElement = root.dyn_into().unwrap_throw();
        root.style().set_property(&name, &value).unwrap();
    })
    .forget();
}

/// Applies a styling theme.
fn apply_theme(theme: &Theme) {
    let mut fonts = theme.fonts.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
    fonts.extend(FALLBACK_FONTS);
    set_css_var("--base-fonts", &fonts.join(", "));

    let background_colors = match theme.color_mode {
        ColorMode::Dark => DARK_BACKGROUND_COLORS,
        ColorMode::Light => LIGHT_BACKGROUND_COLORS,
    };
    background_colors
        .iter()
        .enumerate()
        .for_each(|(index, background_color)| {
            set_css_var(
                &format!("--base-background-color-{}", index + 1),
                &background_color.to_hex_string(),
            );
        });

    let text_color = match theme.color_mode {
        ColorMode::Dark => DARK_TEXT_COLOR,
        ColorMode::Light => LIGHT_TEXT_COLOR,
    };
    set_css_var("--base-text-color", &text_color.to_hex_string());

    let svg_filter = match theme.color_mode {
        ColorMode::Dark => DARK_SVG_FILTER,
        ColorMode::Light => LIGHT_SVG_FILTER,
    };
    set_css_var("--base-primary-svg-filter", svg_filter);

    let svg_filter_disabled = match theme.color_mode {
        ColorMode::Dark => DARK_SVG_FILTER_DISABLED,
        ColorMode::Light => LIGHT_SVG_FILTER_DISABLED,
    };
    set_css_var("--base-primary-svg-filter-disabled", svg_filter_disabled);

    let border_color = match theme.color_mode {
        ColorMode::Dark => DARK_BORDER_COLOR,
        ColorMode::Light => LIGHT_BORDER_COLOR,
    };
    set_css_var("--base-border-color", &border_color.to_hex_string());

    let focus_border_color = match theme.color_mode {
        ColorMode::Dark => DARK_FOCUS_BORDER_COLOR,
        ColorMode::Light => LIGHT_FOCUS_BORDER_COLOR,
    };
    set_css_var(
        "--base-focus-border-color",
        &focus_border_color.to_hex_string(),
    );

    set_css_var("--base-primary-color", &theme.primary_color.to_hex_string());

    let primary_text_color = derive_text_color(&theme.primary_color);
    set_css_var(
        "--base-primary-text-color",
        &primary_text_color.to_hex_string(),
    );

    set_css_var(
        "--base-secondary-color",
        &theme.secondary_color.to_hex_string(),
    );

    let secondary_text_color = derive_text_color(&theme.secondary_color);
    set_css_var(
        "--base-secondary-text-color",
        &secondary_text_color.to_hex_string(),
    );

    let transparent_text_color = text_color;
    set_css_var(
        "--base-transparent-text-color",
        &transparent_text_color.to_hex_string(),
    );

    set_css_var("--base-danger-color", &theme.danger_color.to_hex_string());

    let danger_text_color = derive_text_color(&theme.danger_color);
    set_css_var(
        "--base-danger-text-color",
        &danger_text_color.to_hex_string(),
    );

    set_css_var("--base-error-color", &theme.error_color.to_hex_string());
}

/// Apply a styling theme. The default theme will be used initially, but it
/// can be altered via the returned handles.
#[hook]
pub fn use_theme() -> (Rc<Theme>, Dispatch<Theme>) {
    let (theme, dispatch) = use_store::<Theme>();

    apply_theme(&theme);

    (theme, dispatch)
}
