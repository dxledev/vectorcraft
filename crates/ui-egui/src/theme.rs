//! Design tokens (four UI brightness levels), fonts and egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    Dark,
    #[default]
    MediumDark,
    MediumLight,
    Light,
}

impl Brightness {
    pub const ALL: [Brightness; 4] = [Brightness::Dark, Brightness::MediumDark, Brightness::MediumLight, Brightness::Light];
    pub fn label(self) -> &'static str {
        match self {
            Brightness::Dark => "Dark",
            Brightness::MediumDark => "Medium Dark",
            Brightness::MediumLight => "Medium Light",
            Brightness::Light => "Light",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Brightness::Dark => "dark",
            Brightness::MediumDark => "mediumDark",
            Brightness::MediumLight => "mediumLight",
            Brightness::Light => "light",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }
}

/// User Interface › Accent Color: the one colour for what is on, chosen or focused. Coral, the
/// default, is the app icon's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Accent {
    #[default]
    Coral,
    Blue,
    Violet,
    Green,
    Amber,
    Pink,
}

impl Accent {
    pub const ALL: [Accent; 6] = [Accent::Coral, Accent::Blue, Accent::Violet, Accent::Green, Accent::Amber, Accent::Pink];
    pub fn id(self) -> &'static str {
        match self {
            Accent::Coral => "coral",
            Accent::Blue => "blue",
            Accent::Violet => "violet",
            Accent::Green => "green",
            Accent::Amber => "amber",
            Accent::Pink => "pink",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id().eq_ignore_ascii_case(s))
    }
    /// (accent, accent_strong) on dark panels and on light ones; the strong one fills buttons
    /// with white lettering.
    fn colors(self, dark: bool) -> (u32, u32) {
        match (self, dark) {
            (Accent::Coral, true) => (0xf06a52, 0xe8573f),
            (Accent::Coral, false) => (0xd9472f, 0xd14a33),
            (Accent::Blue, true) => (0x5b96ff, 0x2f74f0),
            (Accent::Blue, false) => (0x2563eb, 0x1f5ad6),
            (Accent::Violet, true) => (0xa98bff, 0x7c55f0),
            (Accent::Violet, false) => (0x7146dc, 0x6a3fd4),
            (Accent::Green, true) => (0x3fcf8e, 0x1f9d63),
            (Accent::Green, false) => (0x16955a, 0x14874f),
            (Accent::Amber, true) => (0xf5b544, 0xc47f0e),
            (Accent::Amber, false) => (0xb06d06, 0xb06d06),
            (Accent::Pink, true) => (0xf36aa3, 0xdb3f7c),
            (Accent::Pink, false) => (0xcc3672, 0xc22e69),
        }
    }
}

/// User Interface › Color Scheme: Modern (slate greys and the chosen accent), Classic (the
/// neutral greys and blue accent of earlier versions) or Desktop (the colours of the desktop's
/// theme, followed as it changes; Modern where there is none). All keep the same layout, cards and
/// corners; only the colours differ.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scheme {
    #[default]
    Modern,
    Classic,
    Desktop,
}

impl Scheme {
    pub const ALL: [Scheme; 3] = [Scheme::Modern, Scheme::Classic, Scheme::Desktop];
    pub fn id(self) -> &'static str {
        match self {
            Scheme::Modern => "modern",
            Scheme::Classic => "classic",
            Scheme::Desktop => "desktop",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.id().eq_ignore_ascii_case(s))
    }
}

/// All colours and metrics the UI uses. Never hard-code a colour in a widget.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    /// Lettering and marks on a fill of `accent` or `accent_strong` (white unless the accent is pale).
    pub on_accent: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub smart_guide: Color32,
    pub guide: Color32,
    pub measure_bg: Color32,
    pub button: Color32,
    pub radius: u8,
    /// Field values, button labels, link labels.
    pub text_strong: Color32,
    /// Panel tab strips, doc-tab bar, dock header.
    pub tab_strip: Color32,
    /// Outline of secondary (outlined) buttons.
    pub button_border: Color32,
    /// Default Layer 1 selection colour and selected-anchor fill.
    pub selection: Color32,
    pub anchor_selected: Color32,
    /// Active item text in tool flyouts.
    pub flyout_active: Color32,
    /// Hint-bar / tab-bar heights etc. scale with this base font size.
    pub font_size: f32,
    /// Window Close button (custom title bar) when hovered, and its glyph there.
    pub caption_close: Color32,
    pub caption_close_text: Color32,
    /// What preview images mark (Flattener Preview highlights).
    pub highlight: Color32,
    /// The bleed outline around artboards (Document Setup → Bleed).
    pub bleed: Color32,
    /// An artboard's Show Center Mark and Show Cross Hairs (Artboard Options), drawn over its
    /// paper in every brightness.
    pub artboard_mark: Color32,
    /// Error marks (a missing linked file).
    pub error: Color32,
    /// Warning marks (a modified linked file).
    pub warning: Color32,
    /// Rotation gizmo axes, also labelled X, Y and Z.
    pub axis_x: Color32,
    pub axis_y: Color32,
    pub axis_z: Color32,
}

impl Tokens {
    pub fn for_brightness(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        // The Craft look: cool slate greys, flat faces with soft corners, and one accent (the app
        // icon's coral) for what is on, chosen or focused. The four brightness levels share the
        // style and differ only in their greys.
        let base = Tokens {
            dark: true,
            app_bar: hex(0x0b0d12),
            panel: hex(0x181b22),
            panel_darker: hex(0x13161c),
            input: hex(0x0f1116),
            input_border: hex(0x2c323e),
            border: hex(0x06070a),
            divider: hex(0x232832),
            text: hex(0xdfe4ec),
            text_dim: hex(0x969faf),
            text_disabled: hex(0x5a6271),
            icon: hex(0xc9d0dc),
            hover: hex(0x262b35),
            tool_active: hex(0x2f3541),
            accent: hex(0xf06a52),
            accent_strong: hex(0xe8573f),
            on_accent: Color32::WHITE,
            row_selected: hex(0x3a2a2c),
            pasteboard: hex(0x101217),
            ruler: hex(0x13161c),
            ruler_tick: hex(0x6f788a),
            smart_guide: hex(0xff3dfc),
            guide: hex(0x4affff),
            measure_bg: Color32::from_rgba_unmultiplied(24, 27, 34, 235),
            button: hex(0x282d37),
            radius: 4,
            text_strong: hex(0xffffff),
            tab_strip: hex(0x0f1116),
            button_border: hex(0x4a5363),
            selection: hex(0x4f80ff),
            anchor_selected: hex(0x3d82ff),
            flyout_active: hex(0xff8a74),
            font_size: 13.0,
            caption_close: hex(0xc42b1c),
            caption_close_text: Color32::WHITE,
            highlight: hex(0xff3030),
            bleed: hex(0xf03030),
            artboard_mark: hex(0x808080),
            error: hex(0xe34850),
            warning: hex(0xf0a330),
            axis_x: hex(0xef6058),
            axis_y: hex(0x45af63),
            axis_z: hex(0x378ef0),
        };
        match b {
            Brightness::Dark => base,
            // Charcoal, the default.
            Brightness::MediumDark => Tokens {
                app_bar: hex(0x1b1f27),
                panel: hex(0x272c36),
                panel_darker: hex(0x21262f),
                tab_strip: hex(0x1e222a),
                input: hex(0x1a1e26),
                input_border: hex(0x3a414e),
                border: hex(0x11141a),
                divider: hex(0x323946),
                text: hex(0xe4e9f1),
                text_dim: hex(0x99a2b2),
                text_disabled: hex(0x626b7a),
                icon: hex(0xccd3de),
                hover: hex(0x353c49),
                tool_active: hex(0x3d4553),
                row_selected: hex(0x4a3236),
                pasteboard: hex(0x1e222a),
                ruler: hex(0x21262f),
                ruler_tick: hex(0x7c8597),
                measure_bg: Color32::from_rgba_unmultiplied(39, 44, 54, 235),
                button: hex(0x39404d),
                button_border: hex(0x5a6476),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xc4cad4),
                panel: hex(0xd3d8e0),
                panel_darker: hex(0xc9cfd8),
                tab_strip: hex(0xc4cad4),
                input: hex(0xe9ecf1),
                input_border: hex(0x9aa2b0),
                border: hex(0xa9b0bc),
                divider: hex(0xbcc3ce),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x4a5160),
                text_disabled: hex(0x7d8594),
                icon: hex(0x272c36),
                hover: hex(0xc5ccd6),
                tool_active: hex(0xb6bec9),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xe7c4bd),
                pasteboard: hex(0xb9c0cb),
                ruler: hex(0xccd2db),
                ruler_tick: hex(0x4a5160),
                measure_bg: Color32::from_rgba_unmultiplied(211, 216, 224, 235),
                button: hex(0xc9d0da),
                button_border: hex(0x8a93a2),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe6e9ee),
                panel: hex(0xf3f5f8),
                panel_darker: hex(0xe9ecf1),
                tab_strip: hex(0xe6e9ee),
                input: hex(0xffffff),
                input_border: hex(0xc3c9d3),
                border: hex(0xd0d5dd),
                divider: hex(0xdde1e8),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x58606e),
                text_disabled: hex(0x9aa1ad),
                icon: hex(0x2a303b),
                hover: hex(0xe3e7ed),
                tool_active: hex(0xd5dbe3),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xf8d9d2),
                pasteboard: hex(0xdfe3e9),
                ruler: hex(0xeceff3),
                ruler_tick: hex(0x58606e),
                measure_bg: Color32::from_rgba_unmultiplied(243, 245, 248, 235),
                button: hex(0xe3e7ed),
                button_border: hex(0xa4abb8),
                ..base
            },
        }
    }

    /// The Classic colour scheme at brightness `b`: neutral greys with a blue accent, a dark
    /// pressed tool and blue-grey selected rows, on the Modern metrics (corners, sizes).
    pub fn classic(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let base = Tokens {
            app_bar: hex(0x262626),
            panel: hex(0x323232),
            panel_darker: hex(0x282828),
            input: hex(0x1f1f1f),
            input_border: hex(0x464646),
            border: hex(0x1b1b1b),
            divider: hex(0x3f3f3f),
            text: hex(0xd8d8d8),
            text_dim: hex(0xa8a8a8),
            text_disabled: hex(0x6e6e6e),
            icon: hex(0xcfcfcf),
            hover: hex(0x444444),
            tool_active: hex(0x1d1d1d),
            accent: hex(0x378ef0),
            accent_strong: hex(0x1473e6),
            row_selected: hex(0x484b50),
            pasteboard: hex(0x1e1e1e),
            ruler: hex(0x2f2f2f),
            ruler_tick: hex(0x8a8a8a),
            measure_bg: Color32::from_rgba_unmultiplied(92, 92, 92, 235),
            button: hex(0x444444),
            text_strong: hex(0xf5f5f5),
            tab_strip: hex(0x282828),
            button_border: hex(0x6a6a6a),
            flyout_active: hex(0x54b2f7),
            ..Tokens::for_brightness(Brightness::Dark)
        };
        match b {
            Brightness::Dark => base,
            Brightness::MediumDark => Tokens {
                // A shade below the panels, so the cards stand out from the window behind them.
                app_bar: hex(0x464646),
                panel: hex(0x535353),
                panel_darker: hex(0x424242),
                tab_strip: hex(0x424242),
                input: hex(0x454545),
                input_border: hex(0x5f5f5f),
                border: hex(0x383838),
                divider: hex(0x4b4b4b),
                text: hex(0xd1d1d1),
                text_strong: hex(0xffffff),
                text_dim: hex(0xb0b0b0),
                text_disabled: hex(0x7a7a7a),
                icon: hex(0xc2c2c2),
                hover: hex(0x606060),
                tool_active: hex(0x303030),
                row_selected: hex(0x52677c),
                pasteboard: hex(0x606060),
                ruler: hex(0x333333),
                button: hex(0x535353),
                button_border: hex(0x747474),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xa9a9a9),
                panel: hex(0xb8b8b8),
                panel_darker: hex(0xaaaaaa),
                tab_strip: hex(0xa6a6a6),
                input: hex(0xd6d6d6),
                input_border: hex(0x8c8c8c),
                border: hex(0x969696),
                divider: hex(0x9e9e9e),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x3c3c3c),
                text_disabled: hex(0x7c7c7c),
                icon: hex(0x2a2a2a),
                hover: hex(0xc8c8c8),
                tool_active: hex(0x969696),
                row_selected: hex(0x9fb4d6),
                pasteboard: hex(0xa0a0a0),
                ruler: hex(0xc2c2c2),
                ruler_tick: hex(0x4a4a4a),
                measure_bg: Color32::from_rgba_unmultiplied(184, 184, 184, 235),
                button: hex(0xcacaca),
                button_border: hex(0x7a7a7a),
                flyout_active: hex(0x1473e6),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe4e4e4),
                panel: hex(0xf0f0f0),
                panel_darker: hex(0xe2e2e2),
                tab_strip: hex(0xdddddd),
                input: hex(0xffffff),
                input_border: hex(0xb4b4b4),
                border: hex(0xcfcfcf),
                divider: hex(0xd6d6d6),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x4b4b4b),
                text_disabled: hex(0x9a9a9a),
                icon: hex(0x303030),
                hover: hex(0xdddddd),
                tool_active: hex(0xcdcdcd),
                row_selected: hex(0xc8daf7),
                pasteboard: hex(0xdcdcdc),
                ruler: hex(0xeaeaea),
                ruler_tick: hex(0x5a5a5a),
                measure_bg: Color32::from_rgba_unmultiplied(240, 240, 240, 235),
                button: hex(0xe0e0e0),
                button_border: hex(0x9a9a9a),
                flyout_active: hex(0x1473e6),
                ..base
            },
        }
    }

    /// The tokens for `b` in `scheme`; Modern takes `accent`, Classic has its own blue, and Desktop
    /// takes the desktop theme's colours (Modern when the desktop has none), whatever `b` and `accent`.
    pub fn for_scheme(scheme: Scheme, b: Brightness, accent: Accent) -> Self {
        match scheme {
            Scheme::Modern => Self::for_brightness(b).with_accent(accent),
            Scheme::Classic => Self::classic(b),
            Scheme::Desktop => match crate::desktop_theme::palette() {
                Some(palette) => Self::desktop(&palette),
                None => Self::for_brightness(b).with_accent(accent),
            },
        }
    }

    /// These tokens with `accent`: the accent, buttons filled with it, the active item in flyouts and
    /// selected rows (the panel tinted towards it).
    pub fn with_accent(self, accent: Accent) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let (a, strong) = accent.colors(self.dark);
        self.with_accent_colors(hex(a), hex(strong), Color32::WHITE)
    }

    /// These tokens with `accent` and `strong` (the fill of buttons, under `on` lettering).
    fn with_accent_colors(self, a: Color32, strong: Color32, on: Color32) -> Self {
        let tint = |k: f32| mix(self.panel, a, k);
        Tokens {
            accent: a,
            accent_strong: strong,
            on_accent: on,
            flyout_active: if self.dark { a } else { strong },
            row_selected: tint(if self.dark { 0.24 } else { 0.2 }),
            tool_active: tint(if self.dark { 0.3 } else { 0.24 }),
            ..self
        }
    }

    /// The tokens for a desktop theme's `palette`: its background, text and primary colour, with the
    /// surfaces, borders and dimmed text between them derived, so any theme (dark or light, pale or
    /// saturated primary) keeps the contrast the UI needs. Colours that belong to the document rather
    /// than the interface (guides, selection, bleed, gizmo axes) stay as the brightness has them.
    pub fn desktop(palette: &crate::desktop_theme::DesktopPalette) -> Self {
        let rgb = |c: crate::desktop_theme::Rgb| Color32::from_rgb(c[0], c[1], c[2]);
        let (bg, fg) = (rgb(palette.background), rgb(palette.foreground));
        let dark = luminance(bg) < luminance(fg);
        let ink = if dark { Color32::WHITE } else { Color32::BLACK };
        // A foreground too close to the background is pushed apart from it first.
        let fg = readable(fg, bg, 4.5, ink);
        let base = Self::for_brightness(if dark { Brightness::MediumDark } else { Brightness::Light });
        let (white, black) = (Color32::WHITE, Color32::BLACK);
        // The cards need a panel that differs from the app bar: the theme's own step when it has one.
        let panel = match palette.surface.map(rgb) {
            Some(s) if contrast(s, bg) >= 1.08 => s,
            _ => mix(bg, fg, 0.07),
        };
        // Text, dimmed text and the rest of the lettering, each readable on the panel.
        let text = readable(fg, panel, 7.0, ink);
        let text_dim = readable(mix(panel, text, 0.62), panel, 4.5, text);
        let text_disabled = mix(panel, text, 0.38);
        // The accent shows up as text, outlines and marks on the panel: pull it towards the text
        // colour until it reads there. Fills keep it as it is and put the lettering that fits on top.
        let accent = readable(rgb(palette.primary), panel, 3.0, text);
        let on = [white, Color32::from_rgb(0x12, 0x12, 0x12)]
            .into_iter()
            .max_by(|a, b| contrast(*a, accent).total_cmp(&contrast(*b, accent)))
            .unwrap_or(white);
        // The resting fill of buttons: the accent moved away from its lettering until that reads.
        let strong =
            readable_on(mix(accent, if dark { bg } else { black }, if dark { 0.18 } else { 0.12 }), on, 4.5, if on == white { black } else { white });
        let (hover, button, input_border, divider, button_border) =
            (mix(panel, fg, 0.09), mix(panel, fg, 0.12), mix(panel, fg, 0.2), mix(panel, fg, 0.1), mix(panel, fg, 0.36));
        let panel_darker = mix(panel, bg, 0.6);
        let shade = |c: Color32, k: f32| mix(c, if dark { black } else { white }, k);
        Tokens {
            dark,
            app_bar: bg,
            panel,
            panel_darker,
            tab_strip: mix(bg, panel, 0.35),
            input: if dark { shade(panel, 0.4) } else { mix(panel, white, 0.6) },
            input_border,
            border: if dark { shade(bg, 0.5) } else { mix(panel, fg, 0.16) },
            divider,
            text,
            text_strong: mix(text, if dark { white } else { black }, 0.6),
            text_dim,
            text_disabled,
            icon: mix(panel, text, 0.88),
            hover,
            tool_active: mix(panel, fg, 0.16),
            pasteboard: if dark { shade(panel, 0.3) } else { mix(panel, fg, 0.06) },
            ruler: panel_darker,
            ruler_tick: text_dim,
            measure_bg: Color32::from_rgba_unmultiplied(panel.r(), panel.g(), panel.b(), 235),
            button,
            button_border,
            error: palette.danger.map(rgb).map_or(base.error, |c| readable(c, panel, 3.0, text)),
            warning: palette.warning.map(rgb).map_or(base.warning, |c| readable(c, panel, 3.0, text)),
            ..base
        }
        .with_accent_colors(accent, strong, on)
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::default()))
    }
    pub fn cr(&self) -> CornerRadius {
        CornerRadius::same(self.radius)
    }
}

/// `a` towards `b` by `k` (0 is `a`, 1 is `b`), per channel.
fn mix(a: Color32, b: Color32, k: f32) -> Color32 {
    let channel = |p: u8, q: u8| (f32::from(p) + (f32::from(q) - f32::from(p)) * k).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(channel(a.r(), b.r()), channel(a.g(), b.g()), channel(a.b(), b.b()))
}

/// WCAG relative luminance of an opaque colour.
fn luminance(c: Color32) -> f32 {
    let channel = |v: u8| {
        let v = f32::from(v) / 255.0;
        if v <= 0.040_45 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * channel(c.r()) + 0.7152 * channel(c.g()) + 0.0722 * channel(c.b())
}

/// The WCAG contrast ratio of two opaque colours, 1 (the same) to 21 (black on white).
fn contrast(a: Color32, b: Color32) -> f32 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

/// `fill`, or the first mix of it towards `towards` that `on` reaches `ratio` against.
fn readable_on(fill: Color32, on: Color32, ratio: f32, towards: Color32) -> Color32 {
    (0..=10).map(|step| mix(fill, towards, step as f32 / 10.0)).find(|m| contrast(on, *m) >= ratio).unwrap_or(towards)
}

/// `c`, or the first mix of it towards `towards` that reaches `ratio` against `background`.
fn readable(c: Color32, background: Color32, ratio: f32, towards: Color32) -> Color32 {
    (0..=10).map(|step| mix(c, towards, step as f32 / 10.0)).find(|m| contrast(*m, background) >= ratio).unwrap_or(towards)
}

/// The toolbar, the panel dock and the icon rail are cards: `body` drawn on the panel colour with
/// soft corners, `margin` of the window's frame colour around it.
pub fn card<R>(ui: &mut egui::Ui, t: &Tokens, margin: egui::Margin, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::NONE
        .fill(t.app_bar)
        .inner_margin(margin)
        .show(ui, |ui| {
            egui::Frame::NONE
                .fill(t.panel)
                .corner_radius(CARD_RADIUS)
                .show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    body(ui)
                })
                .inner
        })
        .inner
}

/// The corner radius of [`card`]s and floating windows.
pub const CARD_RADIUS: u8 = 8;

/// Selected text in the text fields `ui` draws from here on is in the strong text colour. egui
/// paints it in the selection stroke's colour, the accent, which is hard to read on the selection's
/// accent tint (#1465). For fields that draw their own frame: egui's frame takes its focus ring from
/// the same stroke.
pub fn strong_selected_text(ui: &mut egui::Ui) {
    ui.visuals_mut().selection.stroke.color = Tokens::get(ui.ctx()).text_strong;
}

pub const FONT_UI: &str = "ui";
pub const FONT_UI_SEMIBOLD: &str = "ui-semibold";
pub const FONT_MONO: &str = "mono";

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_UI_SEMIBOLD.into()))
}
pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_MONO.into()))
}

/// Installs the interface's fonts and binds the families the panels and dialogs draw in
/// ([`FONT_UI`], [`FONT_UI_SEMIBOLD`], [`FONT_MONO`]). egui 0.37 only bundles its own faces when
/// its `default_fonts` feature is on (this crate enables it); without any face a family lays out
/// tofu, and the `egui_kittest` harness alone panics on a missing glyph. Every context that shows
/// these families needs this once: [`crate::VectorcraftApp::ui`] calls it on its first pass, and
/// an embedder that draws the panels or dialogs without it calls it when it makes the context. It
/// replaces the context's font definitions: add other fonts after it, or keep these families in
/// definitions of one's own. Without the `interface-fonts` feature the families are egui's built-in
/// fonts (and craft-fonts' faces in a build with them), which lack some symbols the panels show
/// (→ ▾ ● ✓).
pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(interface_fonts());
}

/// The definitions [`install_fonts`] installs. [`crate::VectorcraftApp::ui`] keeps them to tell
/// whether egui applied them at once or deferred them to the next pass (it must not take input
/// or draw with fallback metrics while they are pending).
pub fn interface_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let (ui, semi, mono) = interface_faces(&mut fonts);
    let stack = |fonts: &FontDefinitions, family: FontFamily| fonts.families.get(&family).cloned().unwrap_or_default();
    let (fallback, monospace) = (stack(&fonts, FontFamily::Proportional), stack(&fonts, FontFamily::Monospace));
    // Without its own monospace face, `mono` is egui's; with it, the face falls back to the text's.
    let mono_fallback = if mono.is_empty() { monospace } else { fallback.clone() };
    let family =
        |faces: Vec<&str>, fallback: &[String]| -> Vec<String> { faces.into_iter().map(str::to_owned).chain(fallback.iter().cloned()).collect() };
    fonts.families.insert(FontFamily::Proportional, family(ui.clone(), &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI_SEMIBOLD.into()), family(semi, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI.into()), family(ui, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_MONO.into()), family(mono, &mono_fallback));
    add_craft_fonts(&mut fonts);
    fonts
}

/// The interface's own faces, added to `fonts`: the names of the regular, semibold and monospace
/// faces that head their families, in order.
#[cfg(feature = "interface-fonts")]
fn interface_faces(fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    let add = |f: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        f.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    };
    add(fonts, "SourceSans3", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    // Inter is drawn a little smaller than its size: its letters are wider than Source Sans's, and
    // the panels are laid out for those.
    let inter =
        |bytes: &'static [u8]| Arc::new(FontData::from_static(bytes).tweak(egui::epaint::text::FontTweak { scale: 0.92, ..Default::default() }));
    fonts.font_data.insert("Inter".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-Regular.ttf")));
    fonts.font_data.insert("Inter-SemiBold".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf")));
    add(fonts, "SourceSans3-Semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(fonts, "JetBrainsMono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    (vec!["Inter", "SourceSans3"], vec!["Inter-SemiBold", "SourceSans3-Semibold"], vec!["JetBrainsMono"])
}

/// Without the `interface-fonts` feature the interface has no faces of its own: every family is egui's.
#[cfg(not(feature = "interface-fonts"))]
fn interface_faces(_fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    (Vec::new(), Vec::new(), Vec::new())
}

/// The egui name of a craft-fonts face.
pub(crate) fn craft_font_name(f: &vectorcraft_text::CraftFont) -> String {
    format!("craft-fonts {} {}", f.family, f.style)
}

/// The Japanese, Georgian and Devanagari craft-fonts faces (when built with `CRAFT_FONTS_DIR`) as fallbacks at
/// the end of every family, after the app's own fonts and before the installed fonts `ui_fonts`
/// adds: BIZ UDPGothic first (bold first in the semibold family), then the Mincho faces, then Noto
/// Sans Georgian. Each face sits on the baseline of the first face of the family it is in (see
/// `ui_fonts::baseline_copies`).
fn add_craft_fonts(fonts: &mut FontDefinitions) {
    // The name of each face's copy in each family.
    let mut copies: std::collections::HashMap<(FontFamily, String), String> = std::collections::HashMap::new();
    for f in vectorcraft_text::CRAFT_FONTS.iter().filter(|f| f.is_japanese() || f.is_georgian() || f.scripts.contains(&"Deva")) {
        let name = craft_font_name(f);
        for (copy, data, families) in crate::ui_fonts::baseline_copies(fonts, &name, &FontData::from_static(f.bytes)) {
            for family in families {
                copies.insert((family, name.clone()), copy.clone());
            }
            fonts.font_data.insert(copy, Arc::new(data));
        }
    }
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name(FONT_UI_SEMIBOLD.into());
        let copy = |f: &vectorcraft_text::CraftFont| {
            let name = craft_font_name(f);
            copies.get(&(family.clone(), name.clone())).cloned().unwrap_or(name)
        };
        stack.extend(vectorcraft_text::craft_fonts::japanese_ui_fonts(bold).into_iter().map(copy));
        stack.extend(vectorcraft_text::craft_fonts::georgian_ui_fonts().into_iter().map(copy));
        stack.extend(vectorcraft_text::CRAFT_FONTS.iter().filter(|f| f.scripts.contains(&"Deva")).map(copy));
    }
}

/// Apply tokens to egui's global style.
pub fn apply(ctx: &egui::Context, b: Brightness) {
    apply_with(ctx, b, Accent::default(), Scheme::default());
}

/// [`apply`] with an accent colour and colour scheme.
pub fn apply_with(ctx: &egui::Context, b: Brightness, accent: Accent, scheme: Scheme) {
    if scheme == Scheme::Desktop {
        crate::desktop_theme::watch(ctx);
    }
    let t = Tokens::for_scheme(scheme, b, accent);
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, t));
    // egui keeps a style for each system theme and by default follows the system's setting. The
    // chosen brightness sets egui's theme, so these colors stay in force when the system's setting
    // differs or changes. A change of theme starts from the current style, which keeps settings
    // such as the tool tip delay.
    let theme = if t.dark { egui::Theme::Dark } else { egui::Theme::Light };
    if ctx.theme() != theme {
        ctx.set_style_of(theme, ctx.global_style());
    }
    ctx.set_theme(theme);
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, t.border);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.selection.bg_fill = t.accent.gamma_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(if t.dark { 120 } else { 60 }) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(if t.dark { 140 } else { 70 }) };
    let w = &mut v.widgets;
    craft_ui::theme::WidgetPalette { noninteractive: t.panel, inactive: t.button, hovered: t.hover, active: t.tool_active, open: t.hover }.apply_to(
        w,
        t.cr(),
        Stroke::new(1.0, t.text),
    );
    w.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    w.inactive.bg_stroke = Stroke::NONE;
    w.hovered.bg_stroke = Stroke::new(1.0, t.input_border);
    w.active.bg_stroke = Stroke::new(1.0, t.accent);
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(24.0, 24.0);
        s.spacing.menu_margin = egui::Margin::same(6);
        s.spacing.slider_width = 120.0;
        s.text_styles = [
            (TextStyle::Small, FontId::proportional(11.0)),
            (TextStyle::Body, FontId::proportional(13.0)),
            (TextStyle::Button, FontId::proportional(13.0)),
            (TextStyle::Heading, FontId::new(15.0, FontFamily::Name(FONT_UI_SEMIBOLD.into()))),
            (TextStyle::Monospace, FontId::new(11.5, FontFamily::Name(FONT_MONO.into()))),
        ]
        .into();
        s.animation_time = 0.08;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_roundtrip() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        assert!(!Tokens::for_brightness(Brightness::Light).dark);
    }

    /// #1662: a slider's rail (`button_border`, see `widgets::slider`) stands out from the panel
    /// in every brightness.
    #[test]
    fn slider_rails_show_on_every_panel() {
        for b in Brightness::ALL {
            for scheme in Scheme::ALL {
                let t = Tokens::for_scheme(scheme, b, Accent::default());
                let gap = (i32::from(t.button_border.r()) - i32::from(t.panel.r())).abs();
                assert!(gap >= 24, "{scheme:?} {b:?}: rail {:?} on panel {:?}", t.button_border, t.panel);
            }
        }
    }

    /// Classic keeps the Modern corners and the cards' frame, and has its own blue accent
    /// whatever accent is chosen.
    #[test]
    fn classic_scheme_keeps_the_corners_and_its_blue() {
        for b in Brightness::ALL {
            let (modern, classic) = (Tokens::for_brightness(b), Tokens::for_scheme(Scheme::Classic, b, Accent::Pink));
            assert_eq!(classic.radius, modern.radius);
            assert_eq!(classic.dark, modern.dark, "{b:?}");
            assert_ne!(classic.app_bar, classic.panel, "{b:?}: the cards need a frame");
            assert_eq!(classic.accent, Color32::from_rgb(0x37, 0x8e, 0xf0));
            assert_eq!(Scheme::parse(Scheme::Classic.id()), Some(Scheme::Classic));
        }
    }

    /// #3818: the brightness chosen in Preferences stays in force when the system's light or dark
    /// setting differs from it or changes while the app runs. Tool tips keep the theme's panel fill
    /// and text color instead of egui's own white box and gray text.
    #[test]
    fn the_theme_holds_whatever_the_system_theme() {
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 400.0));
        for b in Brightness::ALL {
            for scheme in Scheme::ALL {
                for (first, then) in [(egui::Theme::Dark, egui::Theme::Light), (egui::Theme::Light, egui::Theme::Dark)] {
                    let t = Tokens::for_scheme(scheme, b, Accent::default());
                    let ctx = egui::Context::default();
                    let mut time = 0.0;
                    let mut tip = None;
                    for i in 0..5 {
                        time += 0.5;
                        let system_theme = Some(if i == 0 { first } else { then });
                        let events = if i == 1 { vec![egui::Event::PointerMoved(egui::pos2(30.0, 15.0))] } else { vec![] };
                        let input = egui::RawInput { time: Some(time), screen_rect: Some(screen), events, system_theme, ..Default::default() };
                        let mut out = ctx.run_ui(input, |ui| {
                            if i == 0 {
                                apply_with(ui.ctx(), b, Accent::default(), scheme);
                                ui.ctx().global_style_mut(|s| s.interaction.tooltip_delay = 0.0);
                            }
                            ui.button("Hover me").on_hover_text("Tip text");
                        });
                        out.textures_delta.clear();
                        // The tool tip's text color, and the fill of the box drawn behind it.
                        let mut shapes: Vec<&egui::Shape> = Vec::new();
                        let mut open: Vec<&egui::Shape> = out.shapes.iter().rev().map(|c| &c.shape).collect();
                        while let Some(shape) = open.pop() {
                            match shape {
                                egui::Shape::Vec(inner) => open.extend(inner.iter().rev()),
                                shape => shapes.push(shape),
                            }
                        }
                        tip = shapes.iter().enumerate().find_map(|(n, shape)| match shape {
                            egui::Shape::Text(s) if s.galley.text() == "Tip text" => {
                                let fill = shapes[..n].iter().rev().find_map(|behind| match behind {
                                    egui::Shape::Rect(r) if r.rect.contains(s.pos) && r.fill.a() > 0 => Some(r.fill),
                                    _ => None,
                                });
                                Some((s.fallback_color, fill))
                            }
                            _ => None,
                        });
                    }
                    let v = ctx.global_style().visuals.clone();
                    let what = format!("{scheme:?} {b:?}, system {first:?} then {then:?}");
                    assert_eq!((v.window_fill, v.override_text_color), (t.panel, Some(t.text)), "{what}");
                    assert_eq!(tip, Some((t.text, Some(t.panel))), "{what}: the tool tip's text and box");
                    assert!(contrast(t.text, t.panel) >= 4.5, "{what}: the tool tip's text against its box, {}", contrast(t.text, t.panel));
                }
            }
        }
    }

    /// Desktop themes as the shells write them: dark with a pale primary, dark with a vivid one, a
    /// light one, and a washed-out one whose surface step is missing.
    fn sample_palettes() -> Vec<crate::desktop_theme::DesktopPalette> {
        let p = |bg, fg, surface, primary| crate::desktop_theme::DesktopPalette {
            background: bg,
            foreground: fg,
            surface,
            primary,
            danger: Some([0xeb, 0x6f, 0x92]),
            warning: Some([0xf6, 0xc1, 0x77]),
        };
        vec![
            p([0x11, 0x0e, 0x08], [0xf1, 0xe4, 0xd1], Some([0x21, 0x1d, 0x16]), [0xe4, 0xc2, 0x87]),
            p([0x19, 0x17, 0x24], [0xe0, 0xde, 0xf4], Some([0x30, 0x2d, 0x3e]), [0x9c, 0xcf, 0xd8]),
            p([0x1a, 0x1b, 0x26], [0xc0, 0xca, 0xf5], None, [0x7a, 0xa2, 0xf7]),
            p([0xfa, 0xf4, 0xed], [0x57, 0x52, 0x79], Some([0xf2, 0xe9, 0xe1]), [0xd7, 0x82, 0x7e]),
            p([0xef, 0xf1, 0xf5], [0x4c, 0x4f, 0x69], None, [0xf9, 0xe2, 0xaf]),
            p([0x80, 0x80, 0x80], [0x7c, 0x7c, 0x7c], Some([0x80, 0x80, 0x80]), [0x82, 0x82, 0x82]),
        ]
    }

    /// Whatever the desktop theme, text reads on the panel, lettering reads on filled accents, and
    /// the cards and slider rails stand out.
    #[test]
    fn desktop_themes_keep_the_ui_readable() {
        for palette in sample_palettes() {
            let t = Tokens::desktop(&palette);
            let what = format!("{palette:?}");
            assert!(contrast(t.text, t.panel) >= 4.5, "{what}: text {}", contrast(t.text, t.panel));
            assert!(contrast(t.text_dim, t.panel) >= 3.0, "{what}: dim text {}", contrast(t.text_dim, t.panel));
            assert!(contrast(t.accent, t.panel) >= 2.9, "{what}: accent {}", contrast(t.accent, t.panel));
            assert!(contrast(t.on_accent, t.accent) >= 3.0, "{what}: on the hovered accent {}", contrast(t.on_accent, t.accent));
            assert!(contrast(t.on_accent, t.accent_strong) >= 4.5, "{what}: on strong accent {}", contrast(t.on_accent, t.accent_strong));
            assert_ne!(t.app_bar, t.panel, "{what}: the cards need a frame");
            assert!(contrast(t.button_border, t.panel) >= 1.5, "{what}: slider rail {}", contrast(t.button_border, t.panel));
            assert!(contrast(t.text, t.input) >= 4.5, "{what}: text in fields {}", contrast(t.text, t.input));
        }
    }

    /// The desktop scheme changes the colours, not the shape of the interface.
    #[test]
    fn desktop_scheme_keeps_the_metrics_and_document_colors() {
        for palette in sample_palettes() {
            let (t, modern) = (
                Tokens::desktop(&palette),
                Tokens::for_brightness(if Tokens::desktop(&palette).dark { Brightness::MediumDark } else { Brightness::Light }),
            );
            assert_eq!((t.radius, t.font_size), (modern.radius, modern.font_size));
            assert_eq!(
                (t.guide, t.smart_guide, t.selection, t.bleed, t.axis_x),
                (modern.guide, modern.smart_guide, modern.selection, modern.bleed, modern.axis_x)
            );
        }
        assert_eq!(Scheme::parse("desktop"), Some(Scheme::Desktop));
    }

    /// Existing schemes keep white lettering on their accents.
    #[test]
    fn modern_and_classic_keep_white_on_their_accents() {
        for b in Brightness::ALL {
            for scheme in [Scheme::Modern, Scheme::Classic] {
                assert_eq!(Tokens::for_scheme(scheme, b, Accent::Amber).on_accent, Color32::WHITE);
            }
        }
    }

    /// Changing brightness keeps the interface settings made after the theme was applied, such as
    /// General › Show Tool Tips' delay.
    #[test]
    fn changing_brightness_keeps_other_interface_settings() {
        let ctx = egui::Context::default();
        apply(&ctx, Brightness::Dark);
        ctx.global_style_mut(|s| s.interaction.tooltip_delay = f32::INFINITY);
        apply(&ctx, Brightness::Light);
        assert_eq!(ctx.global_style().interaction.tooltip_delay, f32::INFINITY);
        assert_eq!(ctx.global_style().visuals.window_fill, Tokens::for_brightness(Brightness::Light).panel);
    }
}
