use std::convert::TryFrom;
use crate::*;

//
// Native types to RGBA
//

impl From<[u8; 3]> for DigitalRGBAColor {
    fn from(octets: [u8; 3]) -> Self {
        DigitalRGBAColor{
            r: octets[0],
            g: octets[1],
            b: octets[2],
            a: 255
        }
    }
}

impl From<[u8; 4]> for DigitalRGBAColor {
    fn from(octets: [u8; 4]) -> Self {
        DigitalRGBAColor{
            r: octets[0],
            g: octets[1],
            b: octets[2],
            a: octets[3]
        }
    }
}

impl From<u32> for DigitalRGBAColor {
    fn from(f: u32) -> Self {
        DigitalRGBAColor{
            r: (f >> 24 & 0xff) as u8,
            g: (f >> 16 & 0xff) as u8,
            b: (f >> 8  & 0xff) as u8,
            a: (f       & 0xff) as u8
        }
    }
}

impl From<&[u8; 3]> for DigitalRGBAColor {
    fn from(octets: &[u8; 3]) -> Self {
        DigitalRGBAColor{
            r: octets[0],
            g: octets[1],
            b: octets[2],
            a: 255
        }
    }
}

impl From<&[u8; 4]> for DigitalRGBAColor {
    fn from(octets: &[u8; 4]) -> Self {
        DigitalRGBAColor{
            r: octets[0],
            g: octets[1],
            b: octets[2],
            a: octets[3]
        }
    }
}

impl TryFrom<&[u8]> for DigitalRGBAColor {
    type Error = ColorConversionError;

    fn try_from(octets: &[u8]) -> Result<Self, ColorConversionError> {
        if octets.len() < 3 {
            return Err(ColorConversionError::BufferTooSmall);
        }
        
        if octets.len() < 4 {
            Ok(
                DigitalRGBAColor{
                    r: octets[0],
                    g: octets[1],
                    b: octets[2],
                    a: 255
                }
            )
        } else {
            Ok(
                DigitalRGBAColor{
                    r: octets[0],
                    g: octets[1],
                    b: octets[2],
                    a: octets[3]
                }
            )
        }
    }
}

impl From<[u8; 3]> for NormalizedRGBAColor {
    fn from(octets: [u8; 3]) -> Self {
        NormalizedRGBAColor::from(DigitalRGBAColor::from(octets))
    }
}

impl From<[u8; 4]> for NormalizedRGBAColor {
    fn from(octets: [u8; 4]) -> Self {
        NormalizedRGBAColor::from(DigitalRGBAColor::from(octets))
    }
}

impl From<u32> for NormalizedRGBAColor {
    fn from(f: u32) -> Self {
        NormalizedRGBAColor::from(DigitalRGBAColor::from(f))
    }
}

impl Into<[u8; 3]> for DigitalRGBAColor {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for DigitalRGBAColor {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<u32> for DigitalRGBAColor {
    fn into(self) -> u32 {
        (self.r as u32) << 24 +
        (self.g as u32) << 16 +
        (self.b as u32) << 8 +
        (self.a as u32)
    }
}

impl Into<[u8; 3]> for NormalizedRGBAColor {
    fn into(self) -> [u8; 3] {
        DigitalRGBAColor::from(self).into()
    }
}

impl Into<[u8; 4]> for NormalizedRGBAColor {
    fn into(self) -> [u8; 4] {
        DigitalRGBAColor::from(self).into()
    }
}

impl Into<u32> for NormalizedRGBAColor {
    fn into(self) -> u32 {
        DigitalRGBAColor::from(self).into()
    }
}

//
// RGBA to RGBA
//

impl From<DigitalRGBAColor> for NormalizedRGBAColor {
    fn from(f: DigitalRGBAColor) -> Self {
        NormalizedRGBAColor{
            r: (f.r as f32 / 255.0),
            g: (f.g as f32 / 255.0),
            b: (f.b as f32 / 255.0),
            a: (f.a as f32 / 255.0)
        }
    }
}

impl From<NormalizedRGBAColor> for DigitalRGBAColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        DigitalRGBAColor{
            r: (f.r * 255.0) as u8,
            g: (f.g * 255.0) as u8,
            b: (f.b * 255.0) as u8,
            a: (f.a * 255.0) as u8
        }
    }
}


//
// RGB to RGBA
//

impl From<DigitalRGBColor> for DigitalRGBAColor {
    fn from(f: DigitalRGBColor) -> Self {
        DigitalRGBAColor{
            r: f.r,
            g: f.g,
            b: f.b,
            a: 255
        }
    }
}

impl From<DigitalRGBColor> for NormalizedRGBAColor {
    fn from(f: DigitalRGBColor) -> Self {
        NormalizedRGBAColor{
            r: (f.r as f32 / 255.0),
            g: (f.g as f32 / 255.0),
            b: (f.b as f32 / 255.0),
            a: 1.0
        }
    }
}

impl From<NormalizedRGBColor> for DigitalRGBAColor {
    fn from(f: NormalizedRGBColor) -> Self {
        DigitalRGBAColor{
            r: (f.r * 255.0) as u8,
            g: (f.g * 255.0) as u8,
            b: (f.b * 255.0) as u8,
            a: 255
        }
    }
}

impl From<NormalizedRGBColor> for NormalizedRGBAColor {
    fn from(f: NormalizedRGBColor) -> Self {
        NormalizedRGBAColor{
            r: f.r,
            g: f.g,
            b: f.b,
            a: 1.0
        }
    }
}


//
// CIE to RGBA
//

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for DigitalRGBAColor {
    fn from(f: CIEXYZColor) -> Self {
        DigitalRGBAColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for NormalizedRGBAColor {
    fn from(f: CIEXYZColor) -> Self {
        NormalizedRGBAColor::from(NormalizedRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for DigitalRGBAColor {
    fn from(f: CIELabColor) -> Self {
        DigitalRGBAColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for NormalizedRGBAColor {
    fn from(f: CIELabColor) -> Self {
        NormalizedRGBAColor::from(NormalizedRGBColor::from(f))
    }
}


//
// YCbCr to RGBA
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedYCbCrColor> for DigitalRGBAColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        DigitalRGBAColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedYCbCrColor> for NormalizedRGBAColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        NormalizedRGBAColor::from(NormalizedRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalYCbCrColor> for DigitalRGBAColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        DigitalRGBAColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalYCbCrColor> for NormalizedRGBAColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        NormalizedRGBAColor::from(NormalizedRGBColor::from(f))
    }
}

// 
// Color conversion traits
//

impl YCbCrConvertible for DigitalRGBAColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl YCbCrConvertible for NormalizedRGBAColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl RGBAConvertible for NormalizedRGBAColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl CIELabConvertible for DigitalRGBAColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIELabConvertible for NormalizedRGBAColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for DigitalRGBAColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for NormalizedRGBAColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}