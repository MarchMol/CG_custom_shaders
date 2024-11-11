#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self  {
        Color {
            r: (r as f32).clamp(0.0, 255.0) as u8,
            g: (g as f32).clamp(0.0, 255.0) as u8,
            b: (b as f32).clamp(0.0, 255.0) as u8
        }
    }
    pub fn black() -> Self {
        Color{
            r: 0x00,
            g: 0x00,
            b: 0x00
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = (hex >> 16 & 0xFF) as u8;
        let g = (hex >> 8 & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color {r, g, b}
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32)<< 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
          r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
          g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
          b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
        }
      }

      pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color{
            r: 255 - ((255 - self.r as u16) * (255 - blend.r as u16) / 255) as u8,
            g: 255 - ((255 - self.g as u16) * (255 - blend.g as u16) / 255) as u8,
            b: 255 - ((255 - self.b as u16) * (255 - blend.b as u16) / 255) as u8
        }
    }

    // New blend mode methods
    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() { *self } else { *blend }
    }

    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color{
            r: ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            g: ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            b: ((self.b as f32 * blend.b as f32) / 255.0) as u8
        }
    }

    pub fn blend_add(&self, blend: &Color) -> Color {
        Color{
            r: (self.r as u16 + blend.r as u16).min(255) as u8,
            g: (self.g as u16 + blend.g as u16).min(255) as u8,
            b: (self.b as u16 + blend.b as u16).min(255) as u8
        }
    }

    
  pub fn blend_subtract(&self, blend: &Color) -> Color {
    if blend.is_black() {
      *self
    } else {
      Color {
        r: self.r.saturating_sub(blend.r),
        g: self.g.saturating_sub(blend.g),
        b: self.b.saturating_sub(blend.b)
      }
    }
  }
}

impl std::ops::Add for Color{
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color { r, g, b}
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color{
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b}
    }
}