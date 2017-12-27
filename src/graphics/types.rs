use std::f32;
use std::u32;
use nalgebra as na;

/// A 2 dimensional point representing a location
pub type Point2 = na::Point2<f32>;
/// A 2 dimensional vector representing an offset of a location
pub type Vector2 = na::Vector2<f32>;
/// A 4 dimensional matrix representing an arbitrary 3d transformation
pub type Matrix4 = na::Matrix4<f32>;

/// A simple 2D rectangle.
///
/// The origin of the rectangle is at the top-left,
/// with x increasing to the right and y increasing down.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Rect {
    /// X coordinate of the center of the rect.
    pub x: f32,
    /// Y coordinate of the center of the rect.
    pub y: f32,
    /// Total width of the rect
    pub w: f32,
    /// Total height of the rect.
    pub h: f32,
}

impl Rect {
    /// Create a new rect.
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    /// Creates a new rect a la Love2D's love.graphics.newQuad,
    /// as a fraction of the reference rect's size.
    pub fn fraction(x: f32, y: f32, w: f32, h: f32, reference: &Rect) -> Rect {
        Rect {
            x: x / reference.w,
            y: y / reference.h,
            w: w / reference.w,
            h: h / reference.h,
        }
    }

    /// Create a new rect from i32 coordinates.
    pub fn new_i32(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
        }
    }

    /// Create a new `Rect` with all values zero.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Creates a new `Rect` at 0,0 with width and height 1.
    pub fn one() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// Gets the `Rect`'s x and y coordinates as a `Point2`.
    pub fn point(&self) -> Point2 {
        Point2::new(self.x, self.y)
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    /// Checks whether the `Rect` contains a `Point`
    pub fn contains(&self, point: Point2) -> bool {
        point.x >= self.left() && point.x <= self.right() && point.y <= self.bottom()
            && point.y >= self.top()
    }

    /// Checks whether the `Rect` overlaps another `Rect`
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.left() <= other.right() && self.right() >= other.left() && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    /// Translates the `Rect` by an offset of (x, y)
    pub fn translate(&mut self, offset: Vector2) {
        self.x += offset.x;
        self.y += offset.y;
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: Point2) {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.w *= sx;
        self.h *= sy;
    }
}

impl From<[f32; 4]> for Rect {
    fn from(val: [f32; 4]) -> Self {
        Rect::new(val[0], val[1], val[2], val[3])
    }
}

impl From<Rect> for [f32; 4] {
    fn from(val: Rect) -> Self {
        [val.x, val.y, val.w, val.h]
    }
}

/// A RGBA color in the sRGB color space represented as `f32`'s in the range `[0.0-1.0]`
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Color {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32,
}

/// White
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

/// Black
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

impl Color {
    /// Create a new Color from four f32's in the range [0.0-1.0]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    /// Create a new Color from four u8's in the range `[0-255]`
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from((r, g, b, a))
    }

    /// Create a new Color from three u8's in the range `[0-255]`,
    /// with the alpha component fixed to 255 (opaque)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::from((r, g, b))
    }

    /// Return a tuple of four u8's in the range `[0-255]` with the Color's
    /// components.
    pub fn to_rgba(self) -> (u8, u8, u8, u8) {
        self.into()
    }

    /// Return a tuple of three u8's in the range `[0-255]` with the Color's
    /// components.
    pub fn to_rgb(self) -> (u8, u8, u8) {
        self.into()
    }

    /// Convert a packed u32 containing 0xRRGGBBAA into a Color.conf
    pub fn from_rgba_u32(c: u32) -> Color {
        let rp = ((c & 0xFF00_0000u32) >> 24) as u8;
        let gp = ((c & 0x00FF_0000u32) >> 16) as u8;
        let bp = ((c & 0x0000_FF00u32) >> 8) as u8;
        let ap = (c & 0x0000_00FFu32) as u8;
        Color::from((rp, gp, bp, ap))
    }

    /// Convert a packed u32 containing 0x00RRGGBB into a Color.
    /// This lets you do things like `Color::from_rgb_u32(0xCD09AA)` easily if you want.
    pub fn from_rgb_u32(c: u32) -> Color {
        let rp = ((c & 0x00FF_0000u32) >> 16) as u8;
        let gp = ((c & 0x0000_FF00u32) >> 8) as u8;
        let bp = (c & 0x0000_00FFu32) as u8;
        Color::from((rp, gp, bp))
    }

    /// Convert a Color into a packed u32, containing 0xRRGGBBAA as bytes.
    pub fn to_rgba_u32(self) -> u32 {
        let (r, g, b, a): (u8, u8, u8, u8) = self.into();
        let rp = (u32::from(r)) << 24;
        let gp = (u32::from(g)) << 16;
        let bp = (u32::from(b)) << 8;
        let ap = u32::from(a);
        (rp | gp | bp | ap)
    }

    /// Convert a Color into a packed u32, containing 0x00RRGGBB as bytes.
    pub fn to_rgb_u32(self) -> u32 {
        let (r, g, b, _a): (u8, u8, u8, u8) = self.into();
        let rp = (u32::from(r)) << 16;
        let gp = (u32::from(g)) << 8;
        let bp = u32::from(b);
        (rp | gp | bp)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    /// Convert a `(R, G, B, A)` tuple of `u8`'s in the range 0-255 into a Color
    fn from(val: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = val;
        let rf = (f32::from(r)) / 255.0;
        let gf = (f32::from(g)) / 255.0;
        let bf = (f32::from(b)) / 255.0;
        let af = (f32::from(a)) / 255.0;
        Color::new(rf, gf, bf, af)
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Convert a `(R, G, B)` tuple of `u8`'s in the range 0-255 into a Color,
    /// with a value of 255 for the alpha element (ie, no transparency.)
    fn from(val: (u8, u8, u8)) -> Self {
        let (r, g, b) = val;
        Color::from((r, g, b, 255))
    }
}

impl From<[f32; 4]> for Color {
    /// Turns an `[R, G, B, A] array of f32's into a Color with no format changes.
    /// All inputs should be in the range `[0.0-1.0]`.
    fn from(val: [f32; 4]) -> Self {
        Color::new(val[0], val[1], val[2], val[3])
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    /// Convert a Color into a `(R, G, B, A)` tuple of `u8`'s in the range of 0-255.
    fn from(color: Color) -> Self {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;
        (r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8) {
    /// Convert a Color into a `(R, G, B)` tuple of `u8`'s in the range of 0-255,
    /// ignoring the alpha term
    fn from(color: Color) -> Self {
        let (r, g, b, _) = color.into();
        (r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    /// Convert a Color into an `[R, G, B, A]` array of `f32`'s in the range of `[0.0-1.0]`.
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

/// A RGBA color in the *linear* color space,
/// suitable for shoving into a shader.
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct LinearColor {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32,
}

impl From<Color> for LinearColor {
    /// Convert an (sRGB) Color into a linear color,
    /// per https://en.wikipedia.org/wiki/Srgb#The_reverse_transformation
    fn from(c: Color) -> Self {
        fn f(component: f32) -> f32 {
            let a = 0.055;
            if component <= 0.04045 {
                component / 12.92
            } else {
                ((component + a) / (1.0 + a)).powf(2.4)
            }
        }
        LinearColor {
            r: f(c.r),
            g: f(c.g),
            b: f(c.b),
            a: c.a,
        }
    }
}

impl From<LinearColor> for Color {
    fn from(c: LinearColor) -> Self {
        fn f(component: f32) -> f32 {
            let a = 0.055;
            if component <= 0.0031308 {
                component * 12.92
            } else {
                (1.0 + a) * component.powf(1.0 / 2.4)
            }
        }
        Color {
            r: f(c.r),
            g: f(c.g),
            b: f(c.b),
            a: c.a,
        }
    }
}

impl From<LinearColor> for [f32; 4] {
    fn from(color: LinearColor) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

/// Specifies whether a shape should be drawn
/// filled or as an outline.
#[derive(Debug, Copy, Clone)]
pub enum DrawMode {
    /// A stroked line with the given width
    Line(f32),
    /// A filled shape.
    Fill,
}

/// Specifies what blending method to use when scaling up/down images.
#[derive(Debug, Copy, Clone)]
pub enum FilterMode {
    /// Use linear interpolation
    Linear,
    /// Use nearest-neighbor interpolation
    Nearest,
}

use gfx;
use gfx::texture::FilterMethod;

impl From<FilterMethod> for FilterMode {
    fn from(f: FilterMethod) -> Self {
        match f {
            FilterMethod::Scale => FilterMode::Nearest,
            _other => FilterMode::Linear,
        }
    }
}

impl From<FilterMode> for FilterMethod {
    fn from(f: FilterMode) -> Self {
        match f {
            FilterMode::Nearest => FilterMethod::Scale,
            FilterMode::Linear => FilterMethod::Bilinear,
        }
    }
}

/// Specifies how to wrap textures.
pub type WrapMode = gfx::texture::WrapMode;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color_conversions() {
        let white = Color::new(1.0, 1.0, 1.0, 1.0);
        let w1 = Color::from((255, 255, 255, 255));
        assert_eq!(white, w1);
        let w2: u32 = white.to_rgba_u32();
        assert_eq!(w2, 0xFFFF_FFFFu32);

        let grey = Color::new(0.5019608, 0.5019608, 0.5019608, 1.0);
        let g1 = Color::from((128, 128, 128, 255));
        assert_eq!(grey, g1);
        let g2: u32 = grey.to_rgba_u32();
        assert_eq!(g2, 0x8080_80FFu32);

        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        let b1 = Color::from((0, 0, 0, 255));
        assert_eq!(black, b1);
        let b2: u32 = black.to_rgba_u32();
        assert_eq!(b2, 0x0000_00FFu32);
        assert_eq!(black, Color::from_rgb_u32(0x00_0000u32));
        assert_eq!(black, Color::from_rgba_u32(0x00_0000FFu32));

        let puce1 = Color::from_rgb_u32(0xCC_8899u32);
        let puce2 = Color::from_rgba_u32(0xCC88_99FFu32);
        let puce3 = Color::from((0xCC, 0x88, 0x99, 255));
        let puce4 = Color::new(0.80, 0.53333336, 0.60, 1.0);
        assert_eq!(puce1, puce2);
        assert_eq!(puce1, puce3);
        assert_eq!(puce1, puce4);
    }

    #[test]
    fn test_rect_scaling() {
        let r1 = Rect::new(0.0, 0.0, 128.0, 128.0);
        let r2 = Rect::fraction(0.0, 0.0, 32.0, 32.0, &r1);
        assert_eq!(r2, Rect::new(0.0, 0.0, 0.25, 0.25));

        let r2 = Rect::fraction(32.0, 32.0, 32.0, 32.0, &r1);
        assert_eq!(r2, Rect::new(0.25, 0.25, 0.25, 0.25));
    }

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0.0, 0.0, 128.0, 128.0);
        println!("{} {} {} {}", r.top(), r.bottom(), r.left(), r.right());
        let p = Point2::new(1.0, 1.0);
        assert!(r.contains(p));

        let p = Point2::new(500.0, 0.0);
        assert!(!r.contains(p));
    }

    #[test]
    fn test_rect_overlaps() {
        let r1 = Rect::new(0.0, 0.0, 128.0, 128.0);
        let r2 = Rect::new(0.0, 0.0, 64.0, 64.0);
        assert!(r1.overlaps(&r2));

        let r2 = Rect::new(100.0, 0.0, 128.0, 128.0);
        assert!(r1.overlaps(&r2));

        let r2 = Rect::new(500.0, 0.0, 64.0, 64.0);
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn test_rect_transform() {
        let mut r1 = Rect::new(0.0, 0.0, 64.0, 64.0);
        let r2 = Rect::new(64.0, 64.0, 64.0, 64.0);
        r1.translate(Vector2::new(64.0, 64.0));
        assert!(r1 == r2);

        let mut r1 = Rect::new(0.0, 0.0, 64.0, 64.0);
        let r2 = Rect::new(0.0, 0.0, 128.0, 128.0);
        r1.scale(2.0, 2.0);
        assert!(r1 == r2);

        let mut r1 = Rect::new(32.0, 32.0, 64.0, 64.0);
        let r2 = Rect::new(64.0, 64.0, 64.0, 64.0);
        r1.move_to(Point2::new(64.0, 64.0));
        assert!(r1 == r2);
    }
}
