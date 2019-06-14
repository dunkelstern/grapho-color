//! `grapho-color` describes the color primitives that are used for computer graphics.
//! It contains color conversion functionality and will get SIMD acceleration in the future to be the most effective color-library.
//! 
//! Currently the following color types are implemented:
//! 
//! - Normalized RGB (component values from `0.0` to `1.0`)
//! - Normalized RGBA (component values from `0.0` to `1.0`)
//! - Digital RGB (1 byte per channel)
//! - Digital RGBA (1 byte per channel)
//! - Normalized YCbCr (component values: Y: `0.0` to `1.0`, Cb/Cr: `-0.5` to `0.5`)
//! - Digital YCbCr (1 byte per channel)
//! - CIE Lab (float components)
//! - CIE XYZ (float components)
//! 
//! Conversion from all color types into all others is implemented by implementing the `From`-trait.
//! 
//! Additionally there are `*Convertible`-traits for all color types which additionally implement functionality to convert iterators or `Vec`-Arrays. These will be implemented with SIMD in the future to provide the best performance possible.
//! 
//! To make integration of the color types easy with existing software there are `From`-traits for importing a color from `Vec<u8>` and `u32` types.

#[derive(Debug, PartialEq)]
pub enum ColorConversionError {
    BufferTooSmall
}

mod grayscale;

/// Types marked with this trait will be convertible to `DigitalRGBColor`
pub trait GrayscaleConvertible: From<DigitalGrayscaleColor> {
    /// Convert a vector of color values into a vector of `DigitalRGBColor` values
    fn convert_vec_grayscale(items: Vec<Self>) -> Vec<DigitalGrayscaleColor>;

    /// Create an iterator that yields `DigitalRGBColor` values for all input values
    fn convert_iter_grayscale(items: Box<dyn Iterator<Item = Self>>) -> Box<dyn Iterator<Item = DigitalGrayscaleColor>>;
}

/// Grayscale color type that is based on `u8`-components
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DigitalGrayscaleColor {
    /// red component
    pub v: u8,
}

/// Grayscale color type with normalized color values (float-components)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NormalizedGrayscaleColor {
    /// red component
    pub v: f32,
}

mod rgb;

/// Types marked with this trait will be convertible to `DigitalRGBColor`
pub trait RGBConvertible: From<DigitalRGBColor> {
    /// Convert a vector of color values into a vector of `DigitalRGBColor` values
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor>;

    /// Create an iterator that yields `DigitalRGBColor` values for all input values
    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) -> Box<dyn Iterator<Item = DigitalRGBColor>>;
}


/// RGB color type that is based on `u8`-components
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DigitalRGBColor {
    /// red component
    pub r: u8,
    /// green component
    pub g: u8,
    /// blue component
    pub b: u8
}

/// RGB color type with normalized color values (float-components)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NormalizedRGBColor {
    /// red component
    pub r: f32,
    /// green component
    pub g: f32,
    /// blue component
    pub b: f32
}

mod rgba;

/// Types marked with this trait will be convertible to `DigitalRGBAColor`
pub trait RGBAConvertible: From<DigitalRGBAColor> {

    /// Convert a vector of color values into a vector of `DigitalRGBAColor` values
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor>;
}

/// RGBA color type with `u8`-components
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DigitalRGBAColor {
    /// red component
    pub r: u8,
    /// green component
    pub g: u8,
    /// blue component
    pub b: u8,
    /// alpha component, 0 is transparent
    pub a: u8
}

/// RGBA color type with normalized color values (float-components)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NormalizedRGBAColor {
    /// red component
    pub r: f32,
    /// green component
    pub g: f32,
    /// blue component
    pub b: f32,
    /// alpha component, 0 is transparent
    pub a: f32
}

mod ycbcr;

/// Types marked with this trait will be convertible to `DigitalYCbCrColor`
pub trait YCbCrConvertible: From<DigitalYCbCrColor> {

    /// Convert a vector of color values into a vector of `DigitalYCbCrColor` values
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor>;
}

/// YUV color type with `u8`-components
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DigitalYCbCrColor {
    /// y component (luminance)
    pub y: u8,
    /// cb component (chrominance blue)
    pub cb: u8,
    /// cr component (chrominance red)
    pub cr: u8    
}

/// YUV color type with normalized values (float-components)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NormalizedYCbCrColor {
    /// y component (luminance), unclipped minimum is 0.0, maximum 1.0
    pub y: f32,
    /// cb component (chrominance blue), unclipped minimum is -0.5, maximum is 0.5
    pub cb: f32,
    /// cr component (chrominance red), unclipped minimum is -0.5, maximum 0.5
    pub cr: f32
}

mod cie;

/// Types marked with this trait will be convertible to `CIELabColor`
pub trait CIELabConvertible: From<CIELabColor> {

    /// Convert a vector of color values into a vector of `CIELabColor` values
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor>;
}

/// Types marked with this trait will be convertible to `CIEXYZColor`
pub trait CIEXYZConvertible: From<CIEXYZColor> {

    /// Convert a vector of color values into a vector of `CIEXYZColor` values
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor>;
}

/// CIE Lab color type
///
/// For description of basic concepts read: [Lab Colorspace](http://www.colourphil.co.uk/lab_lch_colour_space.shtml)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CIELabColor {
    /// luminance component, range: 0.0 to 100.0
    pub l: f32,
    /// a component, range: -100.0 to 100.0
    pub a: f32,
    /// b component, range: -100.0 to 100.0
    pub b: f32
}

/// CIE XYZ color type
///
/// For description of basic concepts read: [XYZ Colorspace](https://www.colourphil.co.uk/xyz_colour_space.shtml)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CIEXYZColor {
    /// x component (red tristimulus primary), range: 0.0 to approx. 1.5
    pub x: f32,
    /// y component (green tristimulus primary), range 0.0 to approx. 1.5
    pub y: f32,
    /// z component (blue tristimulus primary), range 0.0 to approx 2.0
    pub z: f32
}
