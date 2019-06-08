use crate::*;

//
// Native types to RGB
//

impl From<[u8; 3]> for DigitalRGBColor {
    fn from(octets: [u8; 3]) -> Self {
        DigitalRGBColor{
            r: octets[0],
            g: octets[1],
            b: octets[2]
        }
    }
}

impl From<[u8; 4]> for DigitalRGBColor {
    fn from(octets: [u8; 4]) -> Self {
        DigitalRGBColor{
            r: octets[0],
            g: octets[1],
            b: octets[2]
        }
    }
}

impl From<u32> for DigitalRGBColor {
    fn from(f: u32) -> Self {
        DigitalRGBColor{
            r: (f >> 24 & 0xff) as u8,
            g: (f >> 16 & 0xff) as u8,
            b: (f >> 8  & 0xff) as u8
        }
    }
}

impl From<[u8; 3]> for NormalizedRGBColor {
    fn from(octets: [u8; 3]) -> Self {
        NormalizedRGBColor::from(DigitalRGBColor::from(octets))
    }
}

impl From<[u8; 4]> for NormalizedRGBColor {
    fn from(octets: [u8; 4]) -> Self {
        NormalizedRGBColor::from(DigitalRGBColor::from(octets))
    }
}

impl From<u32> for NormalizedRGBColor {
    fn from(f: u32) -> Self {
        NormalizedRGBColor::from(DigitalRGBColor::from(f))
    }
}

impl Into<[u8; 3]> for DigitalRGBColor {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for DigitalRGBColor {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }
}

impl Into<u32> for DigitalRGBColor {
    fn into(self) -> u32 {
        (self.r as u32) << 24 +
        (self.g as u32) << 16 +
        (self.b as u32) << 8
    }
}

impl Into<[u8; 3]> for NormalizedRGBColor {
    fn into(self) -> [u8; 3] {
        DigitalRGBColor::from(self).into()
    }
}

impl Into<[u8; 4]> for NormalizedRGBColor {
    fn into(self) -> [u8; 4] {
        DigitalRGBColor::from(self).into()
    }
}

impl Into<u32> for NormalizedRGBColor {
    fn into(self) -> u32 {
        DigitalRGBColor::from(self).into()
    }
}

//
// RGB to RGB
//

impl From<NormalizedRGBColor> for DigitalRGBColor {
    fn from(f: NormalizedRGBColor) -> Self {
        DigitalRGBColor{
            r: (f.r * 255.0) as u8,
            g: (f.g * 255.0) as u8,
            b: (f.b * 255.0) as u8
        }
    }
}

impl From<DigitalRGBColor> for NormalizedRGBColor {
    fn from(f: DigitalRGBColor) -> Self {
        NormalizedRGBColor{
            r: f.r as f32 / 255.0,
            g: f.g as f32 / 255.0,
            b: f.b as f32 / 255.0
        }
    }
}

//
// YCbCr to RGB
//

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedYCbCrColor> for NormalizedRGBColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        NormalizedRGBColor{
            r: (f.y                   + 1.402    * f.cr).min(1.0),
            g: (f.y - 0.344136 * f.cb - 0.714136 * f.cr).min(1.0),
            b: (f.y + 1.772    * f.cb                  ).min(1.0)
        }
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<NormalizedYCbCrColor> for DigitalRGBColor {
    fn from(f: NormalizedYCbCrColor) -> Self {
        DigitalRGBColor::from(NormalizedRGBColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalYCbCrColor> for NormalizedRGBColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        NormalizedRGBColor::from(NormalizedYCbCrColor::from(f))
    }
}

/// using [JFIF/JPEG conversion](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
impl From<DigitalYCbCrColor> for DigitalRGBColor {
    fn from(f: DigitalYCbCrColor) -> Self {
        DigitalRGBColor::from(NormalizedRGBColor::from(f))
    }
}

//
// RGBA to RGB
//

impl From<NormalizedRGBAColor> for DigitalRGBColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        DigitalRGBColor{
            r: (f.r * 255.0).min(255.0) as u8,
            g: (f.g * 255.0).min(255.0) as u8,
            b: (f.b * 255.0).min(255.0) as u8
        }
    }
}

impl From<DigitalRGBAColor> for DigitalRGBColor {
    fn from(f: DigitalRGBAColor) -> Self {
        DigitalRGBColor{
            r: f.r,
            g: f.g,
            b: f.b
        }
    }
}

impl From<NormalizedRGBAColor> for NormalizedRGBColor {
    fn from(f: NormalizedRGBAColor) -> Self {
        NormalizedRGBColor{
            r: f.r,
            g: f.g,
            b: f.b
        }
    }

}

impl From<DigitalRGBAColor> for NormalizedRGBColor {
    fn from(f: DigitalRGBAColor) -> Self {
        NormalizedRGBColor{
            r: (f.r as f32 / 255.0),
            g: (f.g as f32 / 255.0),
            b: (f.b as f32 / 255.0)
        }
    }
}

//
// CIE XYZ to RGB
//

// Implementation lifted from lab crate

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for NormalizedRGBColor {
    fn from(f: CIEXYZColor) -> Self {
        NormalizedRGBColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIEXYZColor> for DigitalRGBColor {
    fn from(f: CIEXYZColor) -> Self {
        let r = f.x * 3.2404541621141054 - f.y * 1.5371385127977166 - f.z * 0.4985314095560162;
        let g = f.x * -0.9692660305051868 + f.y * 1.8760108454466942 + f.z * 0.04155601753034984;
        let b = f.x * 0.05564343095911469 - f.y * 0.20402591351675387 + f.z * 1.0572251882231791;

        DigitalRGBColor{
            r: xyz_to_rgb_map(r),
            g: xyz_to_rgb_map(g),
            b: xyz_to_rgb_map(b)
        }
    }
}

//
// CIE Lab to RGB
//

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for NormalizedRGBColor {
    fn from(f: CIELabColor) -> Self {
        NormalizedRGBColor::from(DigitalRGBColor::from(f))
    }
}

/// using [sRGB conversion matrix](http://www.brucelindbloom.com/index.html?Calc.html)
impl From<CIELabColor> for DigitalRGBColor {
    fn from(f: CIELabColor) -> Self {
        DigitalRGBColor::from(CIEXYZColor::from(f))
    }
}

#[inline]
fn xyz_to_rgb_map(c: f32) -> u8 {
    ((if c > 0.0031308 {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * c
    }) * 255.0)
        .round()
        .min(255.0)
        .max(0.0) as u8
}


// 
// Color conversion traits
//

impl YCbCrConvertible for DigitalRGBColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl YCbCrConvertible for NormalizedRGBColor {
    fn convert_vec_ycbcr(items: Vec<Self>) -> Vec<DigitalYCbCrColor> {
        items.into_iter().map(|x| DigitalYCbCrColor::from(x)).collect()
    }
}

impl RGBConvertible for NormalizedRGBColor {
    fn convert_vec_rgb(items: Vec<Self>) -> Vec<DigitalRGBColor> {
        items.into_iter().map(|x| DigitalRGBColor::from(x)).collect()
    }

    fn convert_iter_rgb(items: Box<dyn Iterator<Item = Self>>) ->  Box<dyn Iterator<Item = DigitalRGBColor>> {
        Box::new(items.map(|x| DigitalRGBColor::from(x)))
    }
}

impl RGBAConvertible for DigitalRGBColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl RGBAConvertible for NormalizedRGBColor {
    fn convert_vec_rgba(items: Vec<Self>) -> Vec<DigitalRGBAColor> {
        items.into_iter().map(|x| DigitalRGBAColor::from(x)).collect()
    }
}

impl CIELabConvertible for DigitalRGBColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIELabConvertible for NormalizedRGBColor {
    fn convert_vec_lab(items: Vec<Self>) -> Vec<CIELabColor> {
        items.into_iter().map(|x| CIELabColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for DigitalRGBColor {
    fn convert_vec_xyz(items: Vec<Self>) -> Vec<CIEXYZColor> {
        items.into_iter().map(|x| CIEXYZColor::from(x)).collect()
    }
}

impl CIEXYZConvertible for NormalizedRGBColor {
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
    fn rgb_d_2_rgb_n() {
        let rd = DigitalRGBColor{r: 255, g: 0, b: 127};
        let rn = NormalizedRGBColor::from(rd);
        assert_eq!(NormalizedRGBColor{r: 1.0, g: 0.0, b: 0.49803922}, rn);
    }

    #[test]
    fn rgb_n_2_rgb_d() {
        let rn = NormalizedRGBColor{r: 1.0, g: 0.0, b: 0.5};
        let rd = DigitalRGBColor::from(rn);
        assert_eq!(DigitalRGBColor{r:255, g: 0, b: 127}, rd);
    }

    #[test]
    fn ycbcr_d_2_rgb_d() {
        let y = DigitalYCbCrColor{y: 255, cb: 0, cr: 0};
        let r = DigitalRGBColor::from(y);
        assert_eq!(DigitalRGBColor{r:76, g: 255, b: 29}, r);
    }

    #[test]
    fn ycbcr_n_2_rgb_d() {
        let y = NormalizedYCbCrColor{y: 1.0, cb: -0.5, cr: -0.5};
        let r = DigitalRGBColor::from(y);
        assert_eq!(DigitalRGBColor{r:76, g: 255, b: 29}, r);
    }

    #[test]
    fn ycbcr_d_2_rgb_n() {
        let y = DigitalYCbCrColor{y: 255, cb: 0, cr: 0};
        let r = NormalizedRGBColor::from(y);
        assert_eq!(NormalizedRGBColor{r: 0.29900002, g: 1.0, b: 0.11400002}, r);
    }

    #[test]
    fn ycbcr_n_2_rgb_n() {
        let y = NormalizedYCbCrColor{y: 1.0, cb: -0.5, cr: -0.5};
        let r = NormalizedRGBColor::from(y);
        assert_eq!(NormalizedRGBColor{r: 0.29900002, g: 1.0, b: 0.11400002}, r);
    }

    #[test]
    fn rgba_n_2_rgb_n() {
        let ra = NormalizedRGBAColor{r: 1.0, g: 0.5, b: 0.2, a: 0.5};
        let r = NormalizedRGBColor::from(ra);
        assert_eq!(NormalizedRGBColor{r: 1.0, g: 0.5, b: 0.2}, r);
    }

    #[test]
    fn rgba_d_2_rgb_n() {
        let ra = DigitalRGBAColor{r: 255, g: 127, b: 63, a: 128};
        let r = NormalizedRGBColor::from(ra);
        assert_eq!(NormalizedRGBColor{r: 1.0, g: 0.49803922, b: 0.24705882}, r);        
    }

    #[test]
    fn rgba_n_2_rgb_d() {
        let ra = NormalizedRGBAColor{r: 1.0, g: 0.5, b: 0.25, a: 0.5};
        let r = DigitalRGBColor::from(ra);
        assert_eq!(DigitalRGBColor{r: 255, g: 127, b: 63}, r);                
    }

    #[test]
    fn rgba_d_2_rgb_d() {
        let ra = DigitalRGBAColor{r: 255, g: 128, b: 64, a: 128};
        let r = DigitalRGBColor::from(ra);
        assert_eq!(DigitalRGBColor{r: 255, g: 128, b: 64}, r);        
    }

    #[rustfmt::skip]
    static LAB_COLOURS: [(DigitalRGBColor, CIELabColor); 16] = [
        (DigitalRGBColor { r:127, g:   0, b:   0 }, CIELabColor { l: 25.301395, a: 47.77433,   b: 37.754025 }),
        (DigitalRGBColor { r:  0, g: 127, b:   0 }, CIELabColor { l: 45.87666,  a: -51.40707,  b: 49.615574 }),
        (DigitalRGBColor { r:  0, g:   0, b: 127 }, CIELabColor { l: 12.808655, a: 47.23452,   b: -64.33745 }),
        (DigitalRGBColor { r:  0, g: 127, b: 127 }, CIELabColor { l: 47.8919,   a: -28.683678, b: -8.42911 }),
        (DigitalRGBColor { r:127, g:   0, b: 127 }, CIELabColor { l: 29.52658,  a: 58.595745,  b: -36.281406 }),
        (DigitalRGBColor { r:255, g:   0, b:   0 }, CIELabColor { l: 53.240784, a: 80.09252,   b: 67.203186 }),
        (DigitalRGBColor { r:  0, g: 255, b:   0 }, CIELabColor { l: 87.73472,  a: -86.18272,  b: 83.17931 }),
        (DigitalRGBColor { r:  0, g:   0, b: 255 }, CIELabColor { l: 32.29701,  a: 79.187515,  b: -107.86016 }),
        (DigitalRGBColor { r:  0, g: 255, b: 255 }, CIELabColor { l: 91.11321,  a: -48.08751,  b: -14.131201 }),
        (DigitalRGBColor { r:255, g:   0, b: 255 }, CIELabColor { l: 60.32421,  a: 98.23433,   b: -60.824894 }),
        (DigitalRGBColor { r:255, g: 255, b:   0 }, CIELabColor { l: 97.13926,  a: -21.553724, b: 94.47797 }),

        (DigitalRGBColor { r:  0, g:   0, b:   0 }, CIELabColor { l: 0.0,       a: 0.0,        b: 0.0 }),
        (DigitalRGBColor { r: 64, g:  64, b:  64 }, CIELabColor { l: 27.09341,  a: 0.0,        b: 0.0 }),
        (DigitalRGBColor { r:127, g: 127, b: 127 }, CIELabColor { l: 53.192772, a: 0.0,        b: 0.0 }),
        (DigitalRGBColor { r:196, g: 196, b: 196 }, CIELabColor { l: 79.15698,  a: 0.0,        b: 0.0 }),
        (DigitalRGBColor { r:255, g: 255, b: 255 }, CIELabColor { l: 100.0,     a: 0.0,        b: 0.0 }),
    ];

    #[test]
    fn lab_2_rgb_d() {
        for test in LAB_COLOURS.iter() {
            assert_eq!(test.0, DigitalRGBColor::from(test.1))
        }
    }
}

