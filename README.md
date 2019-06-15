# grapho-color

This is the color library used by the `grapho`-set of crates.

There is also:

- `grapho-bitplane`: describes pixel-buffers and allows converting color and interleave types
- `grapho-2d`: describes all kinds of 2D vector graphics and math (like polygon clipping)
- `grapho-rasterize-2d`: 2D rasterizer for the vectors described in `grapho-2d`
- `grapho-filters`: pixel based effects and filters for pixel buffers
- `grapho-cv`: computer vision library for grapho stack
- `grapho-3d`: 3D vector math

## What does it do

`grapho-color` describes the color primitives that are used for computer graphics.
It contains color conversion functionality and will get SIMD acceleration in the future to be the most effective color-library.

Currently the following color types are implemented:

- Normalized Grayscale (component values from 0.0 - 1.0)
- Normalized RGB (component values from 0.0 - 1.0)
- Normalized RGBA (component values from 0.0 - 1.0)
- Digital Grayscale (1 byte)
- Digital RGB (1 byte per channel)
- Digital RGBA (1 byte per channel)
- Normalized YCbCr (component values: Y -> 0.0 - 1.0, Cb/Cr -> -0.5 - 0.5)
- Digital YCbCr (1 byte per channel)
- CIE Lab (float components)
- CIE XYZ (float components)

Conversion from all color types into all others is implemented by implementing the `From`-trait.

Additionally there are `*Convertible`-traits for all color types which additionally implement functionality to convert iterators or `Vec`-Arrays. These will be implemented with SIMD in the future to provide the best performance possible.

To make integration of the color types easy with existing software there are `From`-traits for importing a color from `Vec<u8>` and `u32` types.

## TODO

- `HSLColor`
- `HSVColor`
- `YCgCoColor`
- SIMD Optimization for `*Convertible`-traits
