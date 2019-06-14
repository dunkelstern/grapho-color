use crate::*;

// κ and ε parameters used in conversion between XYZ and La*b*.  See
// http://www.brucelindbloom.com/LContinuity.html for explanation as to why
// those are different values than those provided by CIE standard.
const KAPPA: f32 = 24389.0 / 27.0;
const EPSILON: f32 = 216.0 / 24389.0;
const CBRT_EPSILON: f32 = 0.20689655172413796;

//
// CIE to CIE
//

impl From<CIELabColor> for CIEXYZColor {
    fn from(f: CIELabColor) -> Self {
        let fy = (f.l + 16.0) / 116.0;
        let fx = (f.a / 500.0) + fy;
        let fz = fy - (f.b / 200.0);
        let xr = if fx > CBRT_EPSILON {
            fx.powi(3)
        } else {
            ((fx * 116.0) - 16.0) / KAPPA
        };
        let yr = if f.l > EPSILON * KAPPA {
            fy.powi(3)
        } else {
            f.l / KAPPA
        };
        let zr = if fz > CBRT_EPSILON {
            fz.powi(3)
        } else {
            ((fz * 116.0) - 16.0) / KAPPA
        };

        CIEXYZColor{
            x: xr * 0.95047,
            y: yr,
            z: zr * 1.08883
        }
    }
}

impl From<CIEXYZColor> for CIELabColor {
    fn from(f: CIEXYZColor) -> Self {
        let x = xyz_to_lab_map(f.x / 0.95047);
        let y = xyz_to_lab_map(f.y);
        let z = xyz_to_lab_map(f.z / 1.08883);

        CIELabColor {
            l: (116.0 * y) - 16.0,
            a: 500.0 * (x - y),
            b: 200.0 * (y - z),
        }
    }
}

#[inline]
fn xyz_to_lab_map(c: f32) -> f32 {
    if c > EPSILON {
        c.powf(1.0 / 3.0)
    } else {
        (KAPPA * c + 16.0) / 116.0
    }
}

//
// Gray to CIE
//

impl From<DigitalGrayscaleColor> for CIELabColor {
    fn from(f: DigitalGrayscaleColor) -> Self {
        CIELabColor {
            l: (f.v as f32) / 255.0 * 100.0,
            a: 0.0,
            b: 0.0
        }
    }
}

impl From<NormalizedGrayscaleColor> for CIELabColor {
    fn from(f: NormalizedGrayscaleColor) -> Self {
        CIELabColor {
            l: f.v * 100.0,
            a: 0.0,
            b: 0.0
        }
    }
}

impl From<DigitalGrayscaleColor> for CIEXYZColor {
    fn from(f: DigitalGrayscaleColor) -> Self {
        CIEXYZColor::from(CIELabColor::from(f))
    }
}

impl From<NormalizedGrayscaleColor> for CIEXYZColor {
    fn from(f: NormalizedGrayscaleColor) -> Self {
        CIEXYZColor::from(CIELabColor::from(f))
    }
}

//
// RGB to CIE
//

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<DigitalRGBColor> for CIEXYZColor {
    fn from(f: DigitalRGBColor) -> Self {
        let r = rgb_to_xyz_map(f.r);
        let g = rgb_to_xyz_map(f.g);
        let b = rgb_to_xyz_map(f.b);

        CIEXYZColor{
            x: r * 0.4124564390896921 + g * 0.357576077643909 + b * 0.18043748326639894,
            y: r * 0.21267285140562248 + g * 0.715152155287818 + b * 0.07217499330655958,
            z: r * 0.019333895582329317 + g * 0.119192025881303 + b * 0.9503040785363677
        }
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<NormalizedRGBColor> for CIEXYZColor {
    fn from(f: NormalizedRGBColor) -> Self {
        CIEXYZColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<DigitalRGBAColor> for CIEXYZColor {
    fn from(f: DigitalRGBAColor) -> Self {
        CIEXYZColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<NormalizedRGBAColor> for CIEXYZColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        CIEXYZColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<DigitalRGBColor> for CIELabColor {
    fn from(f: DigitalRGBColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<NormalizedRGBColor> for CIELabColor {
    fn from(f: NormalizedRGBColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<DigitalRGBAColor> for CIELabColor {
    fn from(f: DigitalRGBAColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<NormalizedRGBAColor> for CIELabColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

#[inline]
fn rgb_to_xyz_map(c: u8) -> f32 {
    if c > 10 {
        const A: f32 = 0.055 * 255.0;
        const D: f32 = 1.055 * 255.0;
        ((c as f32 + A) / D).powf(2.4)
    } else {
        const D: f32 = 12.92 * 255.0;
        c as f32 / D
    }
}

//
// YCbCr to CIE
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<NormalizedYCbCrColor> for CIEXYZColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        CIEXYZColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<DigitalYCbCrColor> for CIEXYZColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        CIEXYZColor::from(DigitalRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<NormalizedYCbCrColor> for CIELabColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf) for conversion to RGB
/// and [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html) for conversion from CIE
impl From<DigitalYCbCrColor> for CIELabColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        CIELabColor::from(CIEXYZColor::from(f))
    }
}

// 
// Color conversion traits
//

impl YCbCrConvertible for CIELabColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl YCbCrConvertible for CIEXYZColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl RGBConvertible for CIELabColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBConvertible for CIEXYZColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBAConvertible for CIELabColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl RGBAConvertible for CIEXYZColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}
