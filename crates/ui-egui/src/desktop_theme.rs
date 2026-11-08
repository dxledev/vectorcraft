//! The desktop's theme as the UI's colours (User Interface › Color Scheme › Desktop).
//!
//! Some desktops keep one directory per theme, static or generated from the wallpaper, and point a
//! symlink (the "application selector") at the one in use. Each theme directory holds a
//! `Colors.qml` of named colours and, for generated themes, a `palette.json`. This module finds that
//! directory, reads the colours from it and watches it, so the UI follows a change of theme (a new
//! wallpaper, a different static theme, another shell) while the app runs.
//!
//! Where the directory is looked for, in order:
//! 1. `VECTORCRAFT_DESKTOP_THEME`: a theme directory (or a symlink to one), mainly for testing;
//! 2. `$XDG_CONFIG_HOME/themes/.caelestia-use` (default `~/.config/themes/...`), the selector;
//! 3. `$XDG_CONFIG_HOME/themes/current`, the static theme.
//!
//! Nothing here can fail loudly: a missing, half-written or unreadable theme leaves the colours
//! last read (or, with none, the Modern scheme) in force and is tried again a second later.

use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// Theme files are a few hundred bytes; anything larger is not one.
#[cfg(not(target_arch = "wasm32"))]
const MAX_FILE_BYTES: u64 = 1024 * 1024;
/// More named colours than any theme has; bounds what a hostile file makes us keep.
const MAX_COLORS: usize = 512;

/// An opaque colour, `[red, green, blue]`.
pub type Rgb = [u8; 3];

/// The colours the UI takes from a desktop theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DesktopPalette {
    pub background: Rgb,
    pub foreground: Rgb,
    /// A step lighter (dark themes) or darker (light ones) than `background`, when the theme has one.
    pub surface: Option<Rgb>,
    pub primary: Rgb,
    pub danger: Option<Rgb>,
    pub warning: Option<Rgb>,
}

/// Parse a colour as theme files write it: `#rrggbb`, `#aarrggbb` (QML's order, alpha dropped), the
/// same without `#` (palette.json), or `Qt.rgba(r, g, b, a)` with channels in 0..=1 or as `n / 255`.
pub fn parse_color(text: &str) -> Option<Rgb> {
    let text = text.trim().trim_matches(|c| c == '"' || c == '\'');
    if let Some(args) = text.strip_prefix("Qt.rgba(").and_then(|rest| rest.strip_suffix(')')) {
        let mut channels = args.split(',').map(channel);
        let (r, g, b) = (channels.next()??, channels.next()??, channels.next()??);
        return Some([to_byte(r), to_byte(g), to_byte(b)]);
    }
    let hex = text.strip_prefix('#').unwrap_or(text);
    if !hex.is_ascii() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let digits = match hex.len() {
        6 => hex,
        8 => hex.get(2..)?,
        _ => return None,
    };
    let byte = |at: usize| u8::from_str_radix(digits.get(at..at + 2)?, 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

/// One `Qt.rgba` channel: a number, or a quotient such as `33 / 255`.
fn channel(text: &str) -> Option<f64> {
    let mut parts = text.split('/');
    let top: f64 = parts.next()?.trim().parse().ok()?;
    let value = match parts.next() {
        Some(bottom) => {
            let bottom: f64 = bottom.trim().parse().ok()?;
            if bottom <= 0.0 {
                return None;
            }
            top / bottom
        }
        None => top,
    };
    (parts.next().is_none() && value.is_finite() && (0.0..=1.0).contains(&value)).then_some(value)
}

fn to_byte(unit: f64) -> u8 {
    (unit * 255.0).round().clamp(0.0, 255.0) as u8
}

/// The `property color name: value` lines of a `Colors.qml`, by name.
pub fn parse_colors_qml(source: &str) -> BTreeMap<String, Rgb> {
    let mut colors = BTreeMap::new();
    for line in source.lines() {
        if colors.len() >= MAX_COLORS {
            break;
        }
        let line = line.split("//").next().unwrap_or_default().trim();
        let Some(rest) = line.split_once("property color ").map(|(_, rest)| rest) else { continue };
        let Some((name, value)) = rest.split_once(':') else { continue };
        if let Some(color) = parse_color(value.trim().trim_end_matches(';')) {
            colors.insert(name.trim().to_string(), color);
        }
    }
    colors
}

/// The string colours of a `palette.json` (a flat object such as `{"base": "191724", ...}`).
pub fn parse_palette_json(source: &str) -> BTreeMap<String, Rgb> {
    let mut colors = BTreeMap::new();
    if let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(source) {
        for (name, value) in map.iter().take(MAX_COLORS) {
            if let Some(color) = value.as_str().and_then(parse_color) {
                colors.insert(name.clone(), color);
            }
        }
    }
    colors
}

/// The palette from a theme's `Colors.qml` colours, with `palette.json`'s for what it lacks.
pub fn palette_from(qml: &BTreeMap<String, Rgb>, json: &BTreeMap<String, Rgb>) -> Option<DesktopPalette> {
    let pick = |names: &[&str], from: &BTreeMap<String, Rgb>| names.iter().find_map(|n| from.get(*n).copied());
    let both = |q: &[&str], j: &[&str]| pick(q, qml).or_else(|| pick(j, json));
    Some(DesktopPalette {
        background: both(&["background"], &["base"])?,
        foreground: both(&["foreground"], &["text"])?,
        surface: both(&["backgroundGray"], &["surface0"]),
        primary: both(&["primary"], &["primary"])?,
        danger: both(&["danger"], &["red"]),
        warning: both(&["warning"], &["yellow", "peach"]),
    })
}

/// The theme directory in use, or `None` when there is nowhere to look for one.
#[cfg(not(target_arch = "wasm32"))]
pub fn theme_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("VECTORCRAFT_DESKTOP_THEME").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(dir));
    }
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|d| !d.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").filter(|d| !d.is_empty()).map(|h| Path::new(&h).join(".config")))?;
    Some(select_in(&config.join("themes")))
}

/// The theme directory under `themes`. The selector wins whenever it exists, even while it points
/// at nothing (a theme being switched or regenerated): then there is no palette to read, and the
/// colours last read stay, rather than the static theme flashing up in between. Only a desktop
/// without a selector uses `current`.
#[cfg(not(target_arch = "wasm32"))]
fn select_in(themes: &Path) -> PathBuf {
    let selector = themes.join(".caelestia-use");
    if selector.symlink_metadata().is_ok() { selector } else { themes.join("current") }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_small(path: &Path) -> Option<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut text = String::new();
    file.take(MAX_FILE_BYTES).read_to_string(&mut text).ok()?;
    Some(text)
}

/// The palette of the theme in `dir`: `Err` says why there is none.
#[cfg(not(target_arch = "wasm32"))]
pub fn load(dir: &Path) -> Result<DesktopPalette, String> {
    let qml = read_small(&dir.join("Colors.qml")).map(|s| parse_colors_qml(&s)).unwrap_or_default();
    let json = read_small(&dir.join("palette.json")).map(|s| parse_palette_json(&s)).unwrap_or_default();
    palette_from(&qml, &json).ok_or_else(|| format!("no usable colours in {}", dir.display()))
}

static CURRENT: RwLock<Option<Arc<DesktopPalette>>> = RwLock::new(None);
static GENERATION: AtomicU64 = AtomicU64::new(0);
#[cfg(not(target_arch = "wasm32"))]
static WATCHING: AtomicBool = AtomicBool::new(false);

/// The desktop theme's palette as last read, if any.
pub fn palette() -> Option<Arc<DesktopPalette>> {
    CURRENT.read().ok().and_then(|current| current.clone())
}

/// Changes each time the palette does; the UI re-applies its theme when it moves.
pub fn generation() -> u64 {
    GENERATION.load(std::sync::atomic::Ordering::Acquire)
}

fn publish(palette: DesktopPalette) -> bool {
    let Ok(mut current) = CURRENT.write() else { return false };
    if current.as_deref() == Some(&palette) {
        return false;
    }
    *current = Some(Arc::new(palette));
    GENERATION.fetch_add(1, std::sync::atomic::Ordering::Release);
    true
}

/// Read the desktop theme now, and keep reading it once a second on a background thread, asking
/// `ctx` for a frame when it changes. Safe to call every frame: only the first call does anything.
/// Not available on the web, where the page has no desktop to follow.
#[cfg(not(target_arch = "wasm32"))]
pub fn watch(ctx: &egui::Context) {
    // Tests must not depend on the colours of the desktop they run on.
    if cfg!(test) || WATCHING.swap(true, Ordering::AcqRel) {
        return;
    }
    if let Some(dir) = theme_dir() {
        match load(&dir) {
            Ok(palette) => {
                publish(palette);
            }
            Err(why) => log::info!("desktop theme: {why}"),
        }
    }
    let ctx = ctx.clone();
    let started = std::thread::Builder::new().name("desktop-theme".into()).spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            // A theme being switched is briefly missing or half written: keep the last good one.
            if let Some(Ok(palette)) = theme_dir().map(|dir| load(&dir))
                && publish(palette)
            {
                ctx.request_repaint();
            }
        }
    });
    if let Err(why) = started {
        log::warn!("desktop theme: not following changes: {why}");
    }
}

/// The web build has no desktop theme to follow.
#[cfg(target_arch = "wasm32")]
pub fn watch(_ctx: &egui::Context) {}

#[cfg(test)]
mod tests {
    use super::*;

    const ROSE_PINE: &str = r##"pragma Singleton

import Quickshell
import QtQuick

Singleton {
    readonly property color background: "#191724"
    readonly property color backgroundAlpha: Qt.rgba(31 / 255, 29 / 255, 46 / 255, 0.87)
    readonly property color backgroundGray: "#302d3e" // a step up
    readonly property color foreground: "#e0def4"
    readonly property color primary: "#9ccfd8"
    readonly property color danger: "#eb6f92"
    readonly property color warning: "#f6c177"
}
"##;

    #[test]
    fn colours_parse_in_every_notation_the_themes_use() {
        assert_eq!(parse_color("#191724"), Some([0x19, 0x17, 0x24]));
        assert_eq!(parse_color("\"#191724\""), Some([0x19, 0x17, 0x24]));
        assert_eq!(parse_color("191724"), Some([0x19, 0x17, 0x24]));
        assert_eq!(parse_color("#80191724"), Some([0x19, 0x17, 0x24]), "QML puts alpha first");
        assert_eq!(parse_color("Qt.rgba(31 / 255, 29 / 255, 46 / 255, 0.87)"), Some([31, 29, 46]));
        assert_eq!(parse_color("Qt.rgba(1, 0.5, 0, 1)"), Some([255, 128, 0]));
        for bad in
            ["", "#", "#12345", "#ggggg0", "Qt.rgba(2, 0, 0, 1)", "Qt.rgba(1 / 0, 0, 0, 1)", "Qt.rgba(1, 2)", "Qt.rgba(nan, 0, 0, 1)", "#é19724"]
        {
            assert_eq!(parse_color(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn a_colors_qml_gives_the_palette() {
        let palette = palette_from(&parse_colors_qml(ROSE_PINE), &BTreeMap::new());
        assert_eq!(
            palette,
            Some(DesktopPalette {
                background: [0x19, 0x17, 0x24],
                foreground: [0xe0, 0xde, 0xf4],
                surface: Some([0x30, 0x2d, 0x3e]),
                primary: [0x9c, 0xcf, 0xd8],
                danger: Some([0xeb, 0x6f, 0x92]),
                warning: Some([0xf6, 0xc1, 0x77]),
            })
        );
    }

    #[test]
    fn a_palette_json_fills_in_for_a_missing_colors_qml() {
        let json = parse_palette_json(
            r#"{"mode": "dark", "base": "110e08", "text": "f1e4d1", "surface0": "211d16", "primary": "e4c287", "red": "f2a348", "peach": "f6c580", "n": 3}"#,
        );
        let palette = palette_from(&BTreeMap::new(), &json).expect("a palette");
        assert_eq!((palette.background, palette.foreground, palette.primary), ([0x11, 0x0e, 0x08], [0xf1, 0xe4, 0xd1], [0xe4, 0xc2, 0x87]));
        assert_eq!(
            (palette.surface, palette.danger, palette.warning),
            (Some([0x21, 0x1d, 0x16]), Some([0xf2, 0xa3, 0x48]), Some([0xf6, 0xc5, 0x80]))
        );
        assert_eq!(palette_from(&BTreeMap::new(), &BTreeMap::new()), None);
    }

    /// The parsers take whatever the file holds: no byte sequence makes them panic.
    #[test]
    fn the_parsers_survive_arbitrary_bytes() {
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let pieces = ["property color ", "Qt.rgba(", "#", ":", "/", "\"", "{", "}", ",", ")", "\n", "//", "é", "0", "f", "255", "-1", "nan", "1e999"];
        for _ in 0..2000 {
            let mut text = String::new();
            for _ in 0..(next() % 40) {
                if next() % 3 == 0 {
                    text.push(char::from_u32((next() % 0x250) as u32).unwrap_or('?'));
                } else {
                    text.push_str(pieces[(next() % pieces.len() as u64) as usize]);
                }
            }
            let _ = (parse_color(&text), parse_colors_qml(&text), parse_palette_json(&text));
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_dangling_selector_never_falls_back_to_the_static_theme() {
        let root = std::env::temp_dir().join(format!("vectorcraft-selector-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("island")).expect("dirs");
        assert_eq!(select_in(&root), root.join("current"), "no selector");
        std::os::unix::fs::symlink("island", root.join(".caelestia-use")).expect("link");
        assert_eq!(select_in(&root), root.join(".caelestia-use"));
        std::fs::remove_dir_all(root.join("island")).expect("remove");
        assert_eq!(select_in(&root), root.join(".caelestia-use"), "dangling");
        assert!(load(&select_in(&root)).is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_theme_directory_is_read_and_a_broken_one_is_refused() {
        let dir = std::env::temp_dir().join(format!("vectorcraft-desktop-theme-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        assert!(load(&dir).is_err(), "no files");
        std::fs::write(dir.join("Colors.qml"), "Singleton { readonly property color background: \"#12").expect("write");
        assert!(load(&dir).is_err(), "half written");
        std::fs::write(dir.join("Colors.qml"), ROSE_PINE).expect("write");
        assert_eq!(load(&dir).map(|p| p.primary), Ok([0x9c, 0xcf, 0xd8]));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
