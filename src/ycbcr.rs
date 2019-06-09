use std::convert::TryFrom;
use crate::*;

//
// Native types to YCbCr
//

impl From<[u8; 3]> for DigitalYCbCrColor {
    fn from(octets: [u8; 3]) -> Self {
        DigitalYCbCrColor{
            y: octets[0],
            cb: octets[1],
            cr: octets[2]
        }
    }
}

impl From<[u8; 4]> for DigitalYCbCrColor {
    fn from(octets: [u8; 4]) -> Self {
        DigitalYCbCrColor{
            y: octets[0],
            cb: octets[1],
            cr: octets[2]
        }
    }
}

impl From<u32> for DigitalYCbCrColor {
    fn from(f: u32) -> Self {
        DigitalYCbCrColor{
            y: (f >> 24 & 0xff) as u8,
            cb: (f >> 16 & 0xff) as u8,
            cr: (f >> 8  & 0xff) as u8,
        }
    }
}


impl From<&[u8; 3]> for DigitalYCbCrColor {
    fn from(octets: &[u8; 3]) -> Self {
        DigitalYCbCrColor{
            y: octets[0],
            cb: octets[1],
            cr: octets[2]
        }
    }
}

impl From<&[u8; 4]> for DigitalYCbCrColor {
    fn from(octets: &[u8; 4]) -> Self {
        DigitalYCbCrColor{
            y: octets[0],
            cb: octets[1],
            cr: octets[2]
        }
    }
}

impl TryFrom<&[u8]> for DigitalYCbCrColor {
    type Error = ColorConversionError;

    fn try_from(octets: &[u8]) -> Result<Self, ColorConversionError> {
        if octets.len() < 3 {
            return Err(ColorConversionError::BufferTooSmall);
        }
        
        Ok(
            DigitalYCbCrColor{
                y: octets[0],
                cb: octets[1],
                cr: octets[2]
            }
        )
    }
}

impl From<[u8; 3]> for NormalizedYCbCrColor {
    fn from(octets: [u8; 3]) -> Self {
        NormalizedYCbCrColor::from(DigitalYCbCrColor::from(octets))
    }
}

impl From<[u8; 4]> for NormalizedYCbCrColor {
    fn from(octets: [u8; 4]) -> Self {
        NormalizedYCbCrColor::from(DigitalYCbCrColor::from(octets))
    }
}

impl From<u32> for NormalizedYCbCrColor {
    fn from(f: u32) -> Self {
        NormalizedYCbCrColor::from(DigitalYCbCrColor::from(f))
    }
}

impl Into<[u8; 3]> for DigitalYCbCrColor {
    fn into(self) -> [u8; 3] {
        [self.y, self.cb, self.cr]
    }
}

impl Into<[u8; 4]> for DigitalYCbCrColor {
    fn into(self) -> [u8; 4] {
        [self.y, self.cb, self.cr, 255]
    }
}

impl Into<u32> for DigitalYCbCrColor {
    fn into(self) -> u32 {
        (self.y as u32) << 24 +
        (self.cb as u32) << 16 +
        (self.cr as u32) << 8
    }
}

impl Into<[u8; 3]> for NormalizedYCbCrColor {
    fn into(self) -> [u8; 3] {
        DigitalYCbCrColor::from(self).into()
    }
}

impl Into<[u8; 4]> for NormalizedYCbCrColor {
    fn into(self) -> [u8; 4] {
        DigitalYCbCrColor::from(self).into()
    }
}

impl Into<u32> for NormalizedYCbCrColor {
    fn into(self) -> u32 {
        DigitalYCbCrColor::from(self).into()
    }
}

//
// YCbCr to YCbCr
//

impl From<DigitalYCbCrColor> for NormalizedYCbCrColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        NormalizedYCbCrColor{
            y: f.y as f32 / 255.0,
            cb: f.cb as f32 / 255.0 - 0.5,
            cr: f.cr as f32 / 255.0 - 0.5
        }
    }
}

impl From<NormalizedYCbCrColor> for DigitalYCbCrColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        DigitalYCbCrColor{
            y: (f.y * 255.0) as u8,
            cb: ((f.cb + 0.5) * 255.0) as u8,
            cr: ((f.cr + 0.5) * 255.0) as u8
        }
    }
}


//
// RGB to YCbCr
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBColor> for NormalizedYCbCrColor {
    fn from(f: DigitalRGBColor) -> Self {
        NormalizedYCbCrColor::from(DigitalYCbCrColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBColor> for DigitalYCbCrColor {
    fn from(f: DigitalRGBColor) -> Self {
        let r = f.r as f32;
        let g = f.g as f32;
        let b = f.b as f32;
        DigitalYCbCrColor{
            y:  ( 0.299  * r + 0.587  * g + 0.114  * b        ) as u8,
            cb: (-0.1687 * r - 0.3313 * g + 0.5    * b + 128.0) as u8,
            cr: ( 0.5    * r - 0.4187 * g - 0.0813 * b + 128.0) as u8
        }
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBColor> for NormalizedYCbCrColor {
    fn from(f: NormalizedRGBColor) -> Self {
        NormalizedYCbCrColor::from(DigitalYCbCrColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBColor> for DigitalYCbCrColor {
    fn from(f: NormalizedRGBColor) -> Self {
        DigitalYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

//
// RGBA to YCbCr
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBAColor> for NormalizedYCbCrColor {
    fn from(f: DigitalRGBAColor) -> Self {
        NormalizedYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBAColor> for DigitalYCbCrColor {
    fn from(f: DigitalRGBAColor) -> Self {
        DigitalYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBAColor> for NormalizedYCbCrColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        NormalizedYCbCrColor::from(NormalizedRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBAColor> for DigitalYCbCrColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        DigitalYCbCrColor::from(NormalizedRGBColor::from(f))
    }
}


//
// CIE to YCbCr
//


/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<CIEXYZColor> for DigitalYCbCrColor {
    fn from(f: CIEXYZColor) -> Self {
        DigitalYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<CIEXYZColor> for NormalizedYCbCrColor {
    fn from(f: CIEXYZColor) -> Self {
        NormalizedYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<CIELabColor> for DigitalYCbCrColor {
    fn from(f: CIELabColor) -> Self {
        DigitalYCbCrColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<CIELabColor> for NormalizedYCbCrColor {
    fn from(f: CIELabColor) -> Self {
        NormalizedYCbCrColor::from(DigitalRGBColor::from(f))
    }
}


// 
// Color conversion traits
//

impl YCbCrConvertible for NormalizedYCbCrColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl RGBConvertible for DigitalYCbCrColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBConvertible for NormalizedYCbCrColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBAConvertible for DigitalYCbCrColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl RGBAConvertible for NormalizedYCbCrColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl CIELabConvertible for DigitalYCbCrColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIELabConvertible for NormalizedYCbCrColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for DigitalYCbCrColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for NormalizedYCbCrColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}