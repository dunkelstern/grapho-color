use std::convert::TryFrom;
use crate::*;

//
// Native types to Grayscale
//

impl From<[u8; 1]> for DigitalGrayscaleColor {
    fn from(octets: [u8; 1]) -> Self {
        DigitalGrayscaleColor{
            v: octets[0]
        }
    }
}

impl From<&[u8; 1]> for DigitalGrayscaleColor {
    fn from(octets: &[u8; 1]) -> Self {
        DigitalGrayscaleColor{
            v: octets[0]
        }
    }
}

impl TryFrom<&[u8]> for DigitalGrayscaleColor {
    type Error = ColorConversionError;

    fn try_from(octets: &[u8]) -> Result<Self, ColorConversionError> {
        if octets.len() < 1 {
            return Err(ColorConversionError::BufferTooSmall);
        }
        
        Ok(
            DigitalGrayscaleColor{
                v: octets[0]
            }
        )
    }
}

impl From<u8> for DigitalGrayscaleColor {
    fn from(f: u8) -> Self {
        DigitalGrayscaleColor { v: f }
    }
}

impl From<[u8; 1]> for NormalizedGrayscaleColor {
    fn from(octets: [u8; 1]) -> Self {
        NormalizedGrayscaleColor::from(DigitalGrayscaleColor::from(octets))
    }
}

impl From<u8> for NormalizedGrayscaleColor {
    fn from(f: u8) -> Self {
        NormalizedGrayscaleColor::from(DigitalGrayscaleColor::from(f))
    }
}

impl Into<[u8; 1]> for DigitalGrayscaleColor {
    fn into(self) -> [u8; 1] {
        [self.v]
    }
}

impl Into<u8> for DigitalGrayscaleColor {
    fn into(self) -> u8 {
        self.v
    }
}

impl Into<[u8; 1]> for NormalizedGrayscaleColor {
    fn into(self) -> [u8; 1] {
        DigitalGrayscaleColor::from(self).into()
    }
}

impl Into<u8> for NormalizedGrayscaleColor {
    fn into(self) -> u8 {
        DigitalGrayscaleColor::from(self).into()
    }
}

//
// Gray to gray
//

impl From<NormalizedGrayscaleColor> for DigitalGrayscaleColor {
    fn from(f: NormalizedGrayscaleColor) -> Self {
        DigitalGrayscaleColor{
            v: (f.v * 255.0) as u8
        }
    }
}

impl From<DigitalGrayscaleColor> for NormalizedGrayscaleColor {
    fn from(f: DigitalGrayscaleColor) -> Self {
        NormalizedGrayscaleColor{
            v: f.v as f32 / 255.0
        }
    }
}

//
// RGB to Gray
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBColor> for NormalizedGrayscaleColor {
    fn from(f: NormalizedRGBColor) -> Self {
        NormalizedGrayscaleColor {
            v: 0.299  * f.r + 0.587  * f.g + 0.114  * f.b
        }
    }    
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBColor> for NormalizedGrayscaleColor {
    fn from(f: DigitalRGBColor) -> Self {
        let r = f.r as f32;
        let g = f.g as f32;
        let b = f.b as f32;
        NormalizedGrayscaleColor {
            v: 0.299  * r + 0.587  * g + 0.114  * b
        }
    }    
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBColor> for DigitalGrayscaleColor {
    fn from(f: NormalizedRGBColor) -> Self {
        DigitalGrayscaleColor {
            v: ((0.299  * f.r + 0.587  * f.g + 0.114  * f.b) * 255.0) as u8
        }
    }    
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBColor> for DigitalGrayscaleColor {
    fn from(f: DigitalRGBColor) -> Self {
        let r = f.r as f32;
        let g = f.g as f32;
        let b = f.b as f32;
        DigitalGrayscaleColor {
            v: ((0.299  * r + 0.587  * g + 0.114  * b) * 255.0) as u8
        }
    }    
}

//
// YCbCr to Gray
//

impl From<NormalizedYCbCrColor> for NormalizedGrayscaleColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        NormalizedGrayscaleColor{ v: f.y }
    }
}

impl From<NormalizedYCbCrColor> for DigitalGrayscaleColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        DigitalGrayscaleColor { v: (f.y * 255.0) as u8 }
    }
}

impl From<DigitalYCbCrColor> for NormalizedGrayscaleColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        NormalizedGrayscaleColor { v: (f.y as f32) / 255.0 }
    }
}

impl From<DigitalYCbCrColor> for DigitalGrayscaleColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        DigitalGrayscaleColor { v: f.y }
    }
}

//
// RGBA to Gray
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBAColor> for NormalizedGrayscaleColor {
    fn from(f: DigitalRGBAColor) -> Self {
        NormalizedGrayscaleColor::from(DigitalRGBColor::from(f))
    }    
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBAColor> for NormalizedGrayscaleColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        NormalizedGrayscaleColor::from(NormalizedRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalRGBAColor> for DigitalGrayscaleColor {
    fn from(f: DigitalRGBAColor) -> Self {
        DigitalGrayscaleColor::from(DigitalRGBColor::from(f))
   }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedRGBAColor> for DigitalGrayscaleColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        DigitalGrayscaleColor::from(NormalizedRGBColor::from(f))
    }
}

//
// CIE XYZ to RGB
//

// Implementation lifted from lab crate

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for NormalizedGrayscaleColor {
    fn from(f: CIEXYZColor) -> Self {
        NormalizedGrayscaleColor::from(CIELabColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for DigitalGrayscaleColor {
    fn from(f: CIEXYZColor) -> Self {
        DigitalGrayscaleColor::from(CIELabColor::from(f))
    }
}

//
// CIE Lab to RGB
//

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for NormalizedGrayscaleColor {
    fn from(f: CIELabColor) -> Self {
        NormalizedGrayscaleColor { v: f.l / 100.0 }
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for DigitalGrayscaleColor {
    fn from(f: CIELabColor) -> Self {
        DigitalGrayscaleColor{ v: (f.l / 100.0 * 255.0).floor() as u8 }
    }
}


// 
// Color conversion traits
//

impl YCbCrConvertible for DigitalGrayscaleColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl YCbCrConvertible for NormalizedGrayscaleColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl RGBConvertible for NormalizedGrayscaleColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBAConvertible for DigitalGrayscaleColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl RGBAConvertible for NormalizedGrayscaleColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl CIELabConvertible for DigitalGrayscaleColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIELabConvertible for NormalizedGrayscaleColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for DigitalGrayscaleColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for NormalizedGrayscaleColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn gray_d_2_gray_n() {
        let rd = DigitalGrayscaleColor{ v: 255 };
        let rn = NormalizedGrayscaleColor::from(rd);
        assert_eq!(NormalizedGrayscaleColor{ v: 1.0 }, rn);
    }

    #[test]
    fn gray_n_2_gray_d() {
        let rn = NormalizedGrayscaleColor{ v: 1.0 };
        let rd = DigitalGrayscaleColor::from(rn);
        assert_eq!(DigitalGrayscaleColor{ v: 255 }, rd);
    }

    #[test]
    fn ycbcr_d_2_gray_d() {
        let y = DigitalYCbCrColor{y: 255, cb: 0, cr: 0};
        let r = DigitalGrayscaleColor::from(y);
        assert_eq!(DigitalGrayscaleColor{ v: 255 }, r);
    }

    #[test]
    fn ycbcr_n_2_gray_d() {
        let y = NormalizedYCbCrColor{y: 1.0, cb: -0.5, cr: -0.5};
        let r = DigitalGrayscaleColor::from(y);
        assert_eq!(DigitalGrayscaleColor{ v: 255 }, r);
    }

    #[test]
    fn ycbcr_d_2_gray_n() {
        let y = DigitalYCbCrColor{y: 255, cb: 0, cr: 0};
        let r = NormalizedGrayscaleColor::from(y);
        assert_eq!(NormalizedGrayscaleColor{ v: 1.0 }, r);
    }

    #[test]
    fn ycbcr_n_2_gray_n() {
        let y = NormalizedYCbCrColor{y: 1.0, cb: -0.5, cr: -0.5};
        let r = NormalizedGrayscaleColor::from(y);
        assert_eq!(NormalizedGrayscaleColor{ v: 1.0 }, r);
    }
}
 

