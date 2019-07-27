use crate::prelude::*;
use once_cell::sync::OnceCell;
use skia_bindings::{
    C_SkFontStyle_Construct, C_SkFontStyle_Equals, SkFontStyle, SkFontStyle_Slant,
    SkFontStyle_Weight, SkFontStyle_Width,
};
use std::mem;
use std::ops::Deref;

/// Wrapper type of a font weight.
///
/// Use Weight::from() to create a weight from an i32.
/// Use *weight to pull out the wrapped value of the Weight.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Weight(i32);

impl NativeTransmutable<i32> for Weight {}

#[test]
fn test_weight_layout() {
    Weight::test_layout()
}

impl From<i32> for Weight {
    fn from(weight: i32) -> Self {
        Weight(weight)
    }
}

impl Deref for Weight {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(non_upper_case_globals)]
impl Weight {
    #[deprecated(note = "use INVISIBLE")]
    pub const Invisible: Self = Self(SkFontStyle_Weight::kInvisible_Weight as _);
    #[deprecated(note = "use THIN")]
    pub const Thin: Self = Self(SkFontStyle_Weight::kThin_Weight as _);
    #[deprecated(note = "use EXTRA_LIGHT")]
    pub const ExtraLight: Self = Self(SkFontStyle_Weight::kExtraLight_Weight as _);
    #[deprecated(note = "use LIGHT")]
    pub const Light: Self = Self(SkFontStyle_Weight::kLight_Weight as _);
    #[deprecated(note = "use NORMAL")]
    pub const Normal: Self = Self(SkFontStyle_Weight::kNormal_Weight as _);
    #[deprecated(note = "use MEDIUM")]
    pub const Medium: Self = Self(SkFontStyle_Weight::kMedium_Weight as _);
    #[deprecated(note = "use SEMI_BOLD")]
    pub const SemiBold: Self = Self(SkFontStyle_Weight::kSemiBold_Weight as _);
    #[deprecated(note = "use BOLD")]
    pub const Bold: Self = Self(SkFontStyle_Weight::kBold_Weight as _);
    #[deprecated(note = "use EXTRA_BOLD")]
    pub const ExtraBold: Self = Self(SkFontStyle_Weight::kExtraBold_Weight as _);
    #[deprecated(note = "use BLACK")]
    pub const Black: Self = Self(SkFontStyle_Weight::kBlack_Weight as _);
    #[deprecated(note = "use EXTRA_BLACK")]
    pub const ExtraBlack: Self = Self(SkFontStyle_Weight::kExtraBlack_Weight as _);

    pub const INVISIBLE: Self = Self(SkFontStyle_Weight::kInvisible_Weight as _);
    pub const THIN: Self = Self(SkFontStyle_Weight::kThin_Weight as _);
    pub const EXTRA_LIGHT: Self = Self(SkFontStyle_Weight::kExtraLight_Weight as _);
    pub const LIGHT: Self = Self(SkFontStyle_Weight::kLight_Weight as _);
    pub const NORMAL: Self = Self(SkFontStyle_Weight::kNormal_Weight as _);
    pub const MEDIUM: Self = Self(SkFontStyle_Weight::kMedium_Weight as _);
    pub const SEMI_BOLD: Self = Self(SkFontStyle_Weight::kSemiBold_Weight as _);
    pub const BOLD: Self = Self(SkFontStyle_Weight::kBold_Weight as _);
    pub const EXTRA_BOLD: Self = Self(SkFontStyle_Weight::kExtraBold_Weight as _);
    pub const BLACK: Self = Self(SkFontStyle_Weight::kBlack_Weight as _);
    pub const EXTRA_BLACK: Self = Self(SkFontStyle_Weight::kExtraBlack_Weight as _);
}

/// Wrapper type for the width of a font.
///
/// To create a width of a font from an i32, use Width::from().
/// To access the underlying value of the font weight, dereference *weight.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Width(i32);

impl NativeTransmutable<i32> for Width {}

#[test]
fn test_width_layout() {
    Width::test_layout()
}

impl From<i32> for Width {
    fn from(width: i32) -> Self {
        Width(width)
    }
}

impl Deref for Width {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(non_upper_case_globals)]
impl Width {
    #[deprecated(note = "use ULTRA_CONDENSED")]
    pub const UltraCondensed: Self = Self(SkFontStyle_Width::kUltraCondensed_Width as _);
    #[deprecated(note = "use EXTRA_CONDENSED")]
    pub const ExtraCondensed: Self = Self(SkFontStyle_Width::kExtraCondensed_Width as _);
    #[deprecated(note = "use CONDENSED")]
    pub const Condensed: Self = Self(SkFontStyle_Width::kCondensed_Width as _);
    #[deprecated(note = "use SEMI_CONDENSED")]
    pub const SemiCondensed: Self = Self(SkFontStyle_Width::kSemiCondensed_Width as _);
    #[deprecated(note = "use NORMAL")]
    pub const Normal: Self = Self(SkFontStyle_Width::kNormal_Width as _);
    #[deprecated(note = "use SEMI_EXPANDED")]
    pub const SemiExpanded: Self = Self(SkFontStyle_Width::kSemiExpanded_Width as _);
    #[deprecated(note = "use EXPANDED")]
    pub const Expanded: Self = Self(SkFontStyle_Width::kExpanded_Width as _);
    #[deprecated(note = "use EXTRA_EXPANDED")]
    pub const ExtraExpanded: Self = Self(SkFontStyle_Width::kExtraExpanded_Width as _);
    #[deprecated(note = "use ULTRA_EXPANDED")]
    pub const UltraExpanded: Self = Self(SkFontStyle_Width::kUltraExpanded_Width as _);

    pub const ULTRA_CONDENSED: Self = Self(SkFontStyle_Width::kUltraCondensed_Width as _);
    pub const EXTRA_CONDENSED: Self = Self(SkFontStyle_Width::kExtraCondensed_Width as _);
    pub const CONDENSED: Self = Self(SkFontStyle_Width::kCondensed_Width as _);
    pub const SEMI_CONDENSED: Self = Self(SkFontStyle_Width::kSemiCondensed_Width as _);
    pub const NORMAL: Self = Self(SkFontStyle_Width::kNormal_Width as _);
    pub const SEMI_EXPANDED: Self = Self(SkFontStyle_Width::kSemiExpanded_Width as _);
    pub const EXPANDED: Self = Self(SkFontStyle_Width::kExpanded_Width as _);
    pub const EXTRA_EXPANDED: Self = Self(SkFontStyle_Width::kExtraExpanded_Width as _);
    pub const ULTRA_EXPANDED: Self = Self(SkFontStyle_Width::kUltraExpanded_Width as _);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Slant {
    Upright = SkFontStyle_Slant::kUpright_Slant as _,
    Italic = SkFontStyle_Slant::kItalic_Slant as _,
    Oblique = SkFontStyle_Slant::kOblique_Slant as _,
}

impl NativeTransmutable<SkFontStyle_Slant> for Slant {}

#[test]
fn test_slant_layout() {
    Slant::test_layout()
}

// TODO: implement Display
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FontStyle(SkFontStyle);

impl NativeTransmutable<SkFontStyle> for FontStyle {}
#[test]
fn test_font_style_layout() {
    FontStyle::test_layout()
}

impl PartialEq for FontStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFontStyle_Equals(self.native(), rhs.native()) }
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        // does not link under Linux:
        // unsafe { SkFontStyle::new1() }
        FontStyle::from_native(unsafe {
            let mut font_style = mem::uninitialized();
            C_SkFontStyle_Construct(&mut font_style);
            font_style
        })
    }
}

impl FontStyle {
    pub fn new(weight: Weight, width: Width, slant: Slant) -> Self {
        Self::from_native(unsafe {
            SkFontStyle::new(*weight.native(), *width.native(), *slant.native())
        })
    }

    pub fn weight(self) -> Weight {
        Weight::from_native(unsafe { self.native().weight() })
    }

    pub fn width(self) -> Width {
        Width::from_native(unsafe { self.native().width() })
    }

    pub fn slant(self) -> Slant {
        Slant::from_native(unsafe { self.native().slant() })
    }

    pub fn normal() -> FontStyle {
        static NORMAL: OnceCell<FontStyle> = OnceCell::new();
        *NORMAL.get_or_init(|| FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright))
    }

    pub fn bold() -> FontStyle {
        static BOLD: OnceCell<FontStyle> = OnceCell::new();
        *BOLD.get_or_init(|| FontStyle::new(Weight::BOLD, Width::NORMAL, Slant::Upright))
    }

    pub fn italic() -> FontStyle {
        static ITALIC: OnceCell<FontStyle> = OnceCell::new();
        *ITALIC.get_or_init(|| FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Italic))
    }

    pub fn bold_italic() -> FontStyle {
        static BOLD_ITALIC: OnceCell<FontStyle> = OnceCell::new();
        *BOLD_ITALIC.get_or_init(|| FontStyle::new(Weight::BOLD, Width::NORMAL, Slant::Italic))
    }
}

#[test]
fn test_equality() {
    let style: FontStyle = Default::default();
    let style2: FontStyle = Default::default();
    assert!(style == style2);
}
